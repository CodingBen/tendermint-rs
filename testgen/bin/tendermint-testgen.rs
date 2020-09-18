use gumdrop::Options;
use simple_error::*;
use tendermint_testgen::{fuzzer::RandomFuzzer, helpers::*, Commit, Generator, Header, Time, Validator, Vote};
use rand::{self, Rng};

const USAGE: &str = r#"
This is a small utility for producing tendermint datastructures
from minimal input (for testing purposes only).

For example, a tendermint validator can be produced only from an identifier,
or a tendermint header only from a set of validators.

To get an idea which input is needed for each datastructure, try '--help CMD':
it will list the required and optional parameters.

The parameters can be supplied in two ways:
  - via STDIN: in that case they are expected to be a valid JSON object,
    with each parameter being a field of this object
  - via command line arguments to the specific command.

If a parameter is supplied both via STDIN and CLI, the latter is given preference.

In case a particular datastructure can be produced from a single parameter
(like validator), there is a shortcut that allows to provide this parameter
directly via STDIN, without wrapping it into JSON object.
E.g., in the validator case, the following commands are all equivalent:

    tendermint-testgen validator --id a --voting-power 3
    echo -n '{"id": "a", "voting_power": 3}' | tendermint-testgen --stdin validator
    echo -n a | tendermint-testgen --stdin validator --voting-power 3
    echo -n '{"id": "a"}' | tendermint-testgen --stdin validator --voting-power 3
    echo -n '{"id": "a", "voting_power": 100}' | tendermint-testgen --stdin validator --voting-power 3

The result is:
    {
      "address": "730D3D6B2E9F4F0F23879458F2D02E0004F0F241",
      "pub_key": {
        "type": "tendermint/PubKeyEd25519",
        "value": "YnT69eNDaRaNU7teDTcyBedSD0B/Ziqx+sejm0wQba0="
      },
      "voting_power": "3",
      "proposer_priority": null
    }
"#;

#[derive(Debug, Options)]
struct CliOptions {
    #[options(help = "print this help and exit (--help CMD for command-specific help)")]
    help: bool,
    #[options(help = "provide detailed usage instructions")]
    usage: bool,
    #[options(help = "read input from STDIN (default: no)")]
    stdin: bool,
    #[options(help = "reproduce input in JSON format (default: no)")]
    input: bool,
    #[options(help = "fuzz produced values")]
    fuzz: bool,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "produce validator from identifier and other parameters")]
    Validator(Validator),
    #[options(help = "produce header from validator array and other parameters")]
    Header(Header),
    #[options(help = "produce vote from validator and other parameters")]
    Vote(Vote),
    #[options(help = "produce commit from validator array and other parameters")]
    Commit(Commit),
    #[options(help = "produce timestamp from number of seconds since epoch")]
    Time(Time),
    #[options(help = "seed random number generator for fuzzing")]
    Seed(Seed)
}

#[derive(Debug, Options, Clone)]
pub struct Seed {
    #[options(free, help = "unsigned number to use as a seed; omit to use random seed")]
    pub number: Option<u64>,
}

const FUZZER_STATE: &str = ".tendermint_testgen_fuzzer";

fn encode<Gen: Generator<T> + Options, T: serde::Serialize>(
    cli: &Gen,
    opts: &CliOptions,
) -> Result<String, SimpleError> {
    let mut producer = if opts.stdin {
        let stdin = read_stdin()?;
        let default = Gen::from_str(&stdin)?;
        cli.clone().merge_with_default(default)
    } else {
        cli.clone()
    };
    if opts.fuzz {
        let mut fuzzer = require_with!(RandomFuzzer::read_from_file(FUZZER_STATE),
            "failed to read fuzzer state; please initialize using --seed.");
        producer = producer.fuzz(&mut fuzzer);
        fuzzer.write_to_file(FUZZER_STATE);
    }
    if opts.input {
        producer.encode_input()
    } else {
        producer.encode()
    }

}

fn run_command<Gen, T>(cli: &Gen, opts: &CliOptions)
where
    Gen: Generator<T> + Options,
    T: serde::Serialize,
{
    let usage = cli.self_usage();
    let res = encode(cli, opts);
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => {
            eprintln!("Error: {}\n", e);
            eprintln!("Supported parameters for this command are: ");
            print_params(usage);
            std::process::exit(1);
        }
    }
}

fn print_params(options: &str) {
    for line in options.lines().skip(1) {
        eprintln!("{}", line);
    }
}

fn seed_fuzzing(seed: u64) {
    let seed = if seed == 0 {
        rand::thread_rng().gen()
    } else {
        seed
    };
    let fuzzer = RandomFuzzer::new(seed);
    fuzzer.write_to_file(FUZZER_STATE);
    eprintln!("Seeded fuzzing with:");
    println!("{}", seed)
}


fn main() {
    let opts = CliOptions::parse_args_default_or_exit();
    if opts.usage {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }

    match &opts.command {
        None => {
            eprintln!("Produce tendermint datastructures for testing from minimal input\n");
            eprintln!("Please specify a command:");
            eprintln!("{}\n", CliOptions::command_list().unwrap());
            eprintln!("{}\n", CliOptions::usage());
            for cmd in CliOptions::command_list()
                .unwrap()
                .split('\n')
                .map(|s| s.split_whitespace().next().unwrap())
            {
                eprintln!("\n{} parameters:", cmd);
                print_params(CliOptions::command_usage(cmd).unwrap())
            }
            std::process::exit(1);
        }
        Some(Command::Validator(cli)) => run_command(cli, &opts),
        Some(Command::Header(cli)) => run_command(cli, &opts),
        Some(Command::Vote(cli)) => run_command(cli, &opts),
        Some(Command::Commit(cli)) => run_command(cli, &opts),
        Some(Command::Time(cli)) => run_command(cli, &opts),
        Some(Command::Seed(seed)) => seed_fuzzing(seed.number.unwrap_or_default()),
    }
}
