use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use strum::{Display, EnumString, EnumVariantNames};

use crate::quasigroups::{Order, UnimplementedBaseError};

/// Provides methods for decoding and encoding binary-to-text data
pub struct Alphabet {
    base: Order,
    alphabet: Rc<[char]>,
    map: HashMap<char, u8>,
    pad: Option<char>,
}

impl Alphabet {
    pub fn new<S: Into<Rc<[char]>>>(alphabet: S) -> Result<Self, UnimplementedBaseError> {
        let alphabet = alphabet.into();
        let base = alphabet.len().try_into()?;
        let map = alphabet
            .iter()
            .enumerate()
            .map(|(v, &k)| (k, v.try_into().unwrap())) // no base above 255 is implemented
            .collect();
        Ok(Self {
            base,
            alphabet,
            map,
            pad: None,
        })
    }

    /// Adds a pad character to the alphabet
    pub fn with_pad(mut self, pad: char) -> Self {
        self.pad = Some(pad);
        self
    }

    pub fn base(&self) -> Order {
        self.base
    }

    pub fn pad(&self) -> Option<char> {
        self.pad
    }

    /// Decode a single char.
    /// Returns `Err(EncodingError::Pad)` if `c` is the pad character,
    /// or `Err(EncodingError::DecodeError)` if unable to decode for another reason.
    pub fn decode_char(&self, c: char) -> Result<u8, EncodingError> {
        match self.pad {
            Some(p) if p == c => return Err(EncodingError::Pad),
            _ => (),
        };
        self.map
            .get(&c)
            .ok_or(EncodingError::DecodeError(c))
            .copied()
    }

    /// Encode a single char.
    /// If `b >= self.base.alphabet_length()`
    /// then `Err(EncodingError::EncodeError)` is returned.
    pub fn encode_char(&self, b: u8) -> Result<char, EncodingError> {
        self.alphabet
            .get(usize::from(b))
            .ok_or(EncodingError::EncodeError(b))
            .copied()
    }
}

#[derive(Debug)]
pub enum EncodingError {
    DecodeError(char),
    EncodeError(u8),
    Pad,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodeError(c) => write!(f, "Character '{}' not present in alphabet", c),
            Self::EncodeError(b) => write!(f, "Alphabet has less than {} characters", b),
            Self::Pad => write!(f, "Pad char"),
        }
    }
}

impl Error for EncodingError {}

impl From<Alphabets> for Alphabet {
    fn from(alphabet: Alphabets) -> Self {
        let mut out = Self::new(alphabet.as_str().chars().collect::<Rc<[char]>>()).unwrap();
        out.map
            .extend(alphabet.extra_decode_rules().iter().copied());
        if let Some(pad) = alphabet.pad() {
            out = out.with_pad(pad);
        }
        out
    }
}

/// Built-in alphabets
#[derive(Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Alphabets {
    Octal,
    Base8,
    Decimal,
    Base10,
    Duodecimal,
    Base12,
    Hexadecimal,
    Base16,
    /// RFC 4648
    Base32,
    ZBase32,
    Base32crockford,
    Base32hex,
    Base36,
    Base56,
    Base58,
    Base62,
    Base64,
    Base64url,
}

impl Alphabets {
    /// Returns the alphabet itself (rather than it's name)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Octal | Self::Base8 => "01234567",
            Self::Decimal | Self::Base10 => "0123456789",
            Self::Duodecimal | Self::Base12 => "0123456789AB",
            Self::Hexadecimal | Self::Base16 => "0123456789ABCDEF",
            Self::Base32 => "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
            Self::ZBase32 => "ybndrfg8ejkmcpqxot1uwisza345h769",
            Self::Base32crockford => "0123456789ABCDEFGHJKMNPQRSTVWXYZ",
            Self::Base32hex => "0123456789ABCDEFGHIJKLMNOPQRSTUV",
            Self::Base36 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            Self::Base56 => "23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz",
            Self::Base58 => "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            Self::Base62 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            Self::Base64 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/",
            Self::Base64url => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_",
        }
    }

    fn extra_decode_rules(&self) -> Box<[(char, u8)]> {
        match self {
            Self::Duodecimal | Self::Base12 => Box::new([('a', 10), ('b', 11)]),
            Self::Hexadecimal | Self::Base16 => "abcdef"
                .chars()
                .enumerate()
                .map(|(i, c)| (c, u8::try_from(i + 10).unwrap()))
                .collect(),
            Self::Base32crockford => "abcdefghjkmnpqrstvwxyz"
                .chars()
                .enumerate()
                .map(|(i, c)| (c, u8::try_from(i + 10).unwrap()))
                .chain([('o', 0), ('O', 0), ('i', 1), ('I', 1), ('l', 1), ('L', 1)])
                .collect(),
            Self::Base32hex => "abcdefghijklmnopqrstuv"
                .chars()
                .enumerate()
                .map(|(i, c)| (c, u8::try_from(i + 10).unwrap()))
                .collect(),
            Self::Base36 => "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .enumerate()
                .map(|(i, c)| (c, u8::try_from(i + 10).unwrap()))
                .collect(),
            _ => Box::new([]),
        }
    }

    fn pad(&self) -> Option<char> {
        match self {
            Self::Base32 | Self::Base32hex | Self::Base64 | Self::Base64url => Some('='),
            _ => None,
        }
    }
}
