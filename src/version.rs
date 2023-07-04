use std::str::FromStr;

use crate::{Error, Result};

#[derive(Debug)]
pub struct MCVersion {
    pub major: u8,
    pub minor: u8,

    pub is_release: bool,
    pub extra: String,
}

#[derive(Debug)]
pub struct MCVersionReq {
    pub comparators: Vec<Comparator>,
}

impl MCVersionReq {
    pub const ANY: Self = MCVersionReq {
        comparators: Vec::new(),
    };

    pub fn matches(&self, version: &MCVersion) -> bool {
        self.comparators.iter().all(|cmp| cmp.matches(version))
    }
}

impl Comparator {
    pub fn matches(&self, version: &MCVersion) -> bool {
        match self.op {
            Op::Exact => self.major == version.major && self.minor == version.minor,
            Op::Greater => self.major < version.major || (self.major == version.major && self.minor < version.minor),
            Op::GreaterEq => self.major <= version.major || (self.major == version.major && self.minor <= version.minor),
            Op::Less => self.major > version.major || (self.major == version.major && self.minor > version.minor),
            Op::LessEq => self.major >= version.major || (self.major == version.major && self.minor >= version.minor),
            Op::Wildcard => true,
        }
    }
}

impl Default for MCVersionReq {
    fn default() -> Self {
        MCVersionReq::ANY
    }
}

#[derive(Debug)]
pub struct Comparator {
    pub op: Op,
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug)]
pub enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Wildcard,
}

impl MCVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor, is_release: true, extra: String::new() }
    }
}

impl FromStr for MCVersion {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        if !text.contains(".") || text.contains("-") || text.contains(" ") {
            Ok(Self {
                major: 0,
                minor: 0,
                is_release: false,
                extra: text.to_owned(),
            })
        } else {
            let vec: Vec<&str> = text.split(".").collect();
            if vec.len() < 2 || vec[0] != "1" {
                return Err(Error::InvalidVersion(text.to_owned()));
            }
            Ok(Self {
                major: vec[1].parse().unwrap_or(0),
                minor: vec[2].parse().unwrap_or(0),
                is_release: true,
                extra: String::new(),
            })
        }
    }
}


impl FromStr for MCVersionReq {
    type Err = Error;

    fn from_str(_text: &str) -> Result<Self> {
        todo!()
    }
}
