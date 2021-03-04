use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct NewCli {
    #[structopt(flatten)]
    command: NewCommand,
}

impl NewCli {
    pub(crate) fn command(self) -> NewCommand {
        self.command
    }
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct NewCommand {
    /// the project extension
    #[structopt(name = "extension")]
    extension: String,

    /// the project template to create from
    #[structopt(name = "template")]
    template: String,

    /// the project name
    #[structopt(name = "name")]
    name: String,

    /// author of the project
    #[structopt(short = "a", long = "author")]
    author: String,

    /// description of the project
    #[structopt(short = "d", long = "description")]
    description: String,

    /// the module name for TinyGo
    #[structopt(
        short = "m",
        long = "module",
        default_value = "github.com/myorg/mymodule"
    )]
    module: String,

    /// version of the project
    #[structopt(short = "v", long = "version", default_value = "0.0.1")]
    version: String,

    /// variables as specified by an extension
    #[structopt(long = "var")]
    vars: Option<Vec<String>>,
}

pub(crate) fn handle_new(cmd: NewCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("New command: {:?}", cmd);
    Ok(())
}
