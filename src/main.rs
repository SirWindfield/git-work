use clap::{App, AppSettings, Clap, IntoApp};
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};
use heck::KebabCase;
use std::io;

include!(concat!(env!("OUT_DIR"), "/subcommand.rs"));

#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
struct Cli {
    #[clap(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
struct CommitTypeArgs {
    title: Vec<String>,
}

#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
struct CompletionsArgs {
    #[clap(possible_values(&[
        "bash",
        "elvish",
        "fish",
        "powershell",
        "zsh",
    ]))]
    shell: String,
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

macro_rules! gen_match {
    ($cmd:ident, $($variant:ident),*) => {
        match $cmd {
            SubCommand::Completions(args) => {
                let mut app = Cli::into_app();
                match args.shell.as_ref() {
                    "bash" => print_completions::<Bash>(&mut app),
                    "elvish" => print_completions::<Elvish>(&mut app),
                    "fish" => print_completions::<Fish>(&mut app),
                    "powershell" => print_completions::<PowerShell>(&mut app),
                    "zsh" => print_completions::<Zsh>(&mut app),
                    _ => panic!("Unknown generator"),
                }
            }
            $(
                SubCommand::$variant(args) => {
                    let commit_type = stringify!($variant).to_lowercase();
                    let branch_name = format!("{}/{}", commit_type, args.title.join(" ").to_kebab_case());
                    create_branch(&branch_name).expect(&format!("Failed to create branch: {}", branch_name));
                }
            )*
        }
    }
}

fn create_branch(name: &str) -> Result<(), ()> {
    println!("Creating branch: {}", name);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Check if completions should be generated.
    if let Some(cmd) = cli.cmd {
        gen_match!(cmd, Build, Chore, Ci, Docs, Feat, Fix, Perf, Refactor, Revert, Style, Test)
    }
}
