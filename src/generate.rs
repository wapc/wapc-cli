use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct GenerateCli {
    #[structopt(flatten)]
    command: GenerateCommand,
}

impl GenerateCli {
    pub(crate) fn command(self) -> GenerateCommand {
        self.command
    }
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct GenerateCommand {
    /// code generation configuration file
    #[structopt(name = "file")]
    file: String,
}

pub(crate) fn handle_generate(cmd: GenerateCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("Generate command: {:?}", cmd);
    Ok(())
}
