use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct InstallCli {
    #[structopt(flatten)]
    command: InstallCommand,
}

impl InstallCli {
    pub(crate) fn command(self) -> InstallCommand {
        self.command
    }
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct InstallCommand {
    /// the location of the module
    #[structopt(name = "location")]
    location: String,
}

pub(crate) fn handle_install(cmd: InstallCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("Install command: {:?}", cmd);
    Ok(())
}
