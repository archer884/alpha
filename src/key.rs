use std::{error, result};

type Result<T> = result::Result<T, Box<error::Error>>;

static ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Key {
    idx: usize,
    data: Vec<u8>,
}

impl Key {
    pub fn from_str(key: &str) -> Result<Self> {
        if key.is_empty() {
            Err("Invalid key")?;
        }
        
        Ok(Self {
            idx: 0,
            data: key.bytes()
                .filter(|u| u.is_ascii_alphabetic())
                .map(|u| u.to_ascii_uppercase())
                .collect(),
        })
    }

    pub fn encode(&mut self, u: u8) -> Option<u8> {
        if !u.is_ascii_alphabetic() {
            return None;
        }

        let u_idx = u.to_ascii_uppercase() - b'A';
        let u_idx = (u_idx + self.next_offset()) as usize % ALPHABET.len();

        ALPHABET.get(u_idx).cloned()
    }

    pub fn decode(&mut self, u: u8) -> Option<u8> {
        if !u.is_ascii_alphabetic() {
            return None;
        }

        let u_idx = i16::from(u.to_ascii_uppercase() - b'A');
        let raw_offset = u_idx - i16::from(self.next_offset());
        let u_idx = if raw_offset < 0 {
            (raw_offset + 26) as usize
        } else {
            raw_offset as usize
        };

        ALPHABET.get(u_idx).cloned()
    }

    fn next_offset(&mut self) -> u8 {
        if self.idx == self.data.len() {
            self.idx = 1;
            self.data[0] - b'A'
        } else {
            let result = self.data[self.idx];
            self.idx += 1;
            result - b'A'
        }
    }
}
