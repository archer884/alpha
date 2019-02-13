mod key;
mod opt;

use crate::key::Key;
use std::{error, result};

type Result<T> = result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    use opt::Command;

    let output = match Command::from_args()? {
        Command::Encode(key) => encode(key, &read_content()?),
        Command::Decode(key) => decode(key, &read_content()?),
    };

    println!("{}", output);

    Ok(())
}

fn encode(mut key: Key, content: &str) -> String {
    content
        .bytes()
        .filter_map(|u| key.encode(u.to_ascii_uppercase()).map(|u| u as char))
        .collect()
}

fn decode(mut key: Key, content: &str) -> String {
    content
        .bytes()
        .filter_map(|u| key.decode(u.to_ascii_uppercase()).map(|u| u as char))
        .collect()
}

fn read_content() -> Result<String> {
    use std::io::{self, Read};
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
