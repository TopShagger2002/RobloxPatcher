use super::*;
use super::constants::enum_bytes::{ CLIENT, ALL, /* CLOSEST */ FIRST, LAST };
use super::errors::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BinaryType {
    Client,

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ReplaceMentMode {
    All,
    First,
    Last,
    //Closest,
}
impl BinaryType {
    pub fn to_bytes(&self) -> u8 {
        match self {
            BinaryType::Client => CLIENT
        }
    }
    pub fn from_byte(byte: u8) -> Option<BinaryType> {
        match byte {
            CLIENT => Some(BinaryType::Client),
            _ => None,
        }
    }
    pub fn to_string(&self) -> Result<String> {
        match &self {
            BinaryType::Client => Ok("PLAYER".to_owned()),
            _ => { Err(InvalidBinaryType.into()) }
        }
    }
    pub fn from_string(check: &str) -> Result<BinaryType> {
        match check {
            "PLAYER" => { Ok(BinaryType::Client) }
            _ => Err(InvalidBinaryType.into()),
        }
    }
}

impl ReplaceMentMode {
    pub fn to_bytes(&self) -> u8 {
        match self {
            ReplaceMentMode::All => ALL,
            //ReplaceMentMode::Closest => CLOSEST,
            ReplaceMentMode::First => FIRST,
            ReplaceMentMode::Last => LAST,
        }
    }
    pub fn from_byte(byte: u8) -> Result<ReplaceMentMode> {
        match byte {
            ALL => { Ok(ReplaceMentMode::All) }
            //CLOSEST => { Ok(ReplaceMentMode::Closest) }
            FIRST => { Ok(ReplaceMentMode::First) }
            LAST => { Ok(ReplaceMentMode::Last) }
            _ => Err(InvalidReplacementMode.into()),
        }
    }
    pub fn from_string(check: &str) -> Result<ReplaceMentMode> {
        match check {
            "ALL" => { Ok(ReplaceMentMode::All) }
            //"CLOSEST" => { Ok(ReplaceMentMode::Closest) }
            "FIRST" => { Ok(ReplaceMentMode::First) }
            "LAST" => { Ok(ReplaceMentMode::Last) }
            _ => Err(InvalidReplacementMode.into()),
        }
    }
}
