use flate2::read::GzDecoder;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::{copy, Cursor, Read};
use std::path::PathBuf;
use structopt::StructOpt;

// Thanks to @ChrisRx for this
const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];

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

    /// folder to install extension into
    #[structopt(long = "wapc-home", env = "WAPC_HOME")]
    wapc_home: Option<String>,
}

pub(crate) fn handle_install(cmd: InstallCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    // get home dir (can be optionally provided as arg)
    let wapc_dir = if let Some(wapc_home) = cmd.wapc_home {
        wapc_home
    } else if let Some(dir) = home::home_dir() {
        format!("{}/.wapc", dir.to_str().unwrap())
    } else {
        return Err(
            "Could not determine home directory. Try setting the WAPC_HOME environment variable"
                .into(),
        );
    };

    // download tar (optional compression) and create in-memory archive
    // let extension: &[u8] = &reqwest::blocking::get(cmd.location)?.bytes()?;
    // remove this block, replace with above let statement
    let mut buf = Vec::new();
    let mut f: File = File::open("../cli-brainstorming/extension.tar.gz")?;
    f.read_to_end(&mut buf)?;
    let extension: &[u8] = &buf;
    // remove this block, replace with above let statement

    let mut extension_archive = tar::Archive::new(if extension[0..2] == GZIP_MAGIC {
        Box::new(GzDecoder::new(extension)) as Box<dyn Read>
    } else {
        Box::new(Cursor::new(extension)) as Box<dyn Read>
    });

    // validate archive contents?
    // unpack tarball into home dir
    create_dir_all(wapc_dir.clone())?;
    for entry in extension_archive.entries()? {
        println!("{:?}", entry.unwrap().path());
    }
    // extension_archive.unpack(wapc_dir)?;

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn it_works() {
        assert!(true)
    }
}
