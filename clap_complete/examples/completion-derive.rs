//! How to use value hints and generate shell completions.
//!
//! Usage with zsh:
//! ```sh
//! cargo run --example value_hints_derive -- --generate=zsh > /usr/local/share/zsh/site-functions/_value_hints_derive
//! compinit
//! ./target/debug/examples/value_hints_derive --<TAB>
//! ```
//! fish:
//! ```sh
//! cargo run --example value_hints_derive -- --generate=fish > value_hints_derive.fish
//! . ./value_hints_derive.fish
//! ./target/debug/examples/value_hints_derive --<TAB>
//! ```
use clap::{Command, CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "value_hints_derive")]
struct Opt {
    /// If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    // Showcasing all possible ValueHints:
    #[arg(long, value_hint = ValueHint::Unknown)]
    unknown: Option<String>,
    #[arg(long, value_hint = ValueHint::Other)]
    other: Option<String>,
    #[arg(short, long, value_hint = ValueHint::AnyPath)]
    path: Option<PathBuf>,
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
    #[arg(short, long, value_hint = ValueHint::DirPath)]
    dir: Option<PathBuf>,
    #[arg(short, long, value_hint = ValueHint::ExecutablePath)]
    exe: Option<PathBuf>,
    #[arg(long, value_hint = ValueHint::CommandName)]
    cmd_name: Option<OsString>,
    #[arg(short, long, value_hint = ValueHint::CommandString)]
    cmd: Option<String>,
    // Command::trailing_var_ar is required to use ValueHint::CommandWithArguments
    #[arg(trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
    command_with_args: Vec<String>,
    #[arg(short, long, value_hint = ValueHint::Username)]
    user: Option<String>,
    #[arg(long, value_hint = ValueHint::Hostname)]
    host: Option<String>,
    #[arg(long, value_hint = ValueHint::Url)]
    url: Option<String>,
    #[arg(long, value_hint = ValueHint::EmailAddress)]
    email: Option<String>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let opt = Opt::parse();

    if let Some(generator) = opt.generator {
        let mut cmd = Opt::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
    } else {
        println!("{:#?}", opt);
    }
}
