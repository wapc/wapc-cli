use structopt::clap::AppSettings;
use structopt::StructOpt;

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
    /// new command
    New,
    /// generate command
    Generate,
}

fn main() {
    let cli = Cli::from_args();
    match cli.command {
        CliCommand::New => println!("You ran `wapc new`"),
        CliCommand::Generate => println!("You ran `wapc generate`"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true)
    }
}
