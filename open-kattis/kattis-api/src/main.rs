use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
#[value()]
enum Command {
    /// [Default] Run testing on the sample test case locally
    #[value()]
    LocalTest,
    /// Submit the solution and get results
    #[value()]
    Submit,
}

/// A simple program to create and manage rust workspace sub-projects for each open-kattis problem
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Name of the problem as seen in the URL on open-kattis, eg. sequences
    #[arg()]
    problem_name: String,

    /// Task to perform
    #[arg(value_enum, default_value_t = Command::LocalTest)]
    command: Command,
}

fn main() {
    match Args::try_parse() {
        Err(e) => {
            print!("{}", e)
        }
        Ok(r) => {
            print!("{:?}", r)
        }
    }
}
