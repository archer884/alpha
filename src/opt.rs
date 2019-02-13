use crate::{key::Key, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "alpha", about = "Alphabet cipher implementation")]
pub enum Opt {
    #[structopt(name = "encode")]
    Encode { key: String },
    #[structopt(name = "decode")]
    Decode { key: String },
}

pub enum Command {
    Encode(Key),
    Decode(Key),
}

impl Command {
    pub fn from_args() -> Result<Command> {
        match structopt::StructOpt::from_args() {
            Opt::Encode { key } => Ok(Command::Encode(Key::from_str(&key)?)),
            Opt::Decode { key } => Ok(Command::Decode(Key::from_str(&key)?)),
        }
    }
}
