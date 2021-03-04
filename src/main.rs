use structopt::clap::AppSettings;
use structopt::StructOpt;

mod new;
use crate::new::{handle_new, NewCli};
mod install;
use crate::install::{handle_install, InstallCli};
mod generate;
use crate::generate::{handle_generate, GenerateCli};

/// This renders appropriately with escape characters
const ASCII: &str = "
                 ___  ___ 
__      ____ _  / _ \\/ __\\
\\ \\ /\\ / / _` |/ /_)/ /   
 \\ V  V / (_| / ___/ /___ 
  \\_/\\_/ \\__,_\\/   \\____/ 

Rust command line interface for creating new waPC projects and code generation
";

#[derive(Debug, Clone, StructOpt)]
#[structopt(global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands, AppSettings::DisableHelpSubcommand]),
            name = "wapc",
            about = ASCII)]
struct Cli {
    #[structopt(flatten)]
    command: CliCommand,
}

#[derive(Debug, Clone, StructOpt)]
enum CliCommand {
    /// create a new waPC project
    #[structopt(name = "new")]
    New(NewCli),
    /// generate code from a configuration file
    Generate(GenerateCli),
    /// install waPC extensions
    Install(InstallCli),
}

fn main() {
    let cli = Cli::from_args();
    let res = match cli.command {
        CliCommand::New(new_cli) => handle_new(new_cli.command()),
        CliCommand::Generate(generate_cli) => handle_generate(generate_cli.command()),
        CliCommand::Install(install_cli) => handle_install(install_cli.command()),
    };
    match res {
        Ok(_) => println!("Command executed successfully"),
        Err(_) => println!("Command failed"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true)
    }
}
