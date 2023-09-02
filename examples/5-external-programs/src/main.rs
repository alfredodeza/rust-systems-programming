use clap::{Parser, ArgAction};
use rdf::run_df;
use log::LevelFilter;
use env_logger::{Builder, Target};
use std::fs::OpenOptions;


#[derive(Parser)]
#[command(name = "rdf", version = "0.0.1", author = "Alfredo Deza", about = "df wrapper in Rust")]
struct Opts {
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(long, help= "Enable logging to a file")]
    log_file: bool,

    #[clap(short, long, env = "RDF_DEBUG")]
    debug: bool,

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "Get information about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Path to query", default_value = "")]
    path: String,
}

fn main() {
    let opts = Opts::parse();

    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Debug);

    if opts.log_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rdf.log")
            .unwrap();
        builder.target(Target::Pipe(Box::new(file)));
    }

    builder.init();

    // Example usage of the global flags
    if opts.debug {
        log::debug!("Debug mode enabled");
    }

    match opts.cmd {
        Command::Info(info_opts) => {
            // Example usage of the verbosity level
            match opts.verbose_level {
                0 => {
                    // Quiet mode
                }
                1 => {
                    println!("Running in verbose mode level 1");
                }
                2 => {
                    println!("Running in verbose mode level 2");
                }
                3 | _ => {
                    println!("Running in verbose mode level 3");
                }
            }

            let output = serde_json::to_string_pretty(&run_df(&info_opts.path)).unwrap();
            println!("{}", output);
        }
    }
}
