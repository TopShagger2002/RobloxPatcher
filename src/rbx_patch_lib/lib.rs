use std::mem::transmute;

use self::errors::BadEncodedPatch;

use self::{
    enums::{ BinaryType, ReplaceMentMode },
    functions::{ bytes_to_client_type, client_types_to_bytes },
    structs::{ PatchBuilder, Patch },
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod structs;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod constants;

fn to_bytes(number: usize) -> [u8; 8] {
    unsafe { transmute(number.to_be()) }
}

fn from_bytes(bytes: &[u8; 8]) -> usize {
    usize::from_be_bytes(*bytes)
}

pub fn serialize(patch: Patch) -> Vec<u8> {
    if patch.pattern.len() != patch.replacement.len() {
        return vec![];
    }
    let capacity: usize = 2 + 8 + patch.pattern.len() * 2;
    let mut bytes: Vec<u8> = Vec::with_capacity(capacity);

    bytes.push(client_types_to_bytes(patch.supported_binarys));
    bytes.push(patch.replacement_mode.to_bytes());

    for byte in to_bytes(patch.pattern.len()) {
        bytes.push(byte);
    }

    for byte in patch.pattern {
        bytes.push(byte);
    }
    for byte in patch.replacement {
        bytes.push(byte);
    }

    return bytes;
}

pub fn deserialize(bytes: &[u8]) -> Result<Patch> {
    if bytes.len() < 14 {
        println!("{} bad len", bytes.len());
        return Err(errors::BadEncodedPatch.into());
    }
    dbg!("{:?}", bytes);
    let binary_byte = bytes[0];
    let replacement_type = bytes[1];
    let capacity_bytes: &[u8] = &bytes[2..10];
    if capacity_bytes.len() != 8 {
        return Err(BadEncodedPatch.into());
    }
    let mut new_capacity_bytes: [u8; 8] = [0; 8];
    for (index, byte) in capacity_bytes.iter().enumerate() {
        new_capacity_bytes[index] = byte.clone();
    }
    let capacity_bytes = new_capacity_bytes;

    let capacity = from_bytes(&capacity_bytes);

    let replacement_type = ReplaceMentMode::from_byte(replacement_type)?;

    let to_replace = &bytes[10..10 + capacity];
    let replacement = &bytes[10 + capacity..10 + capacity * 2];

    let patch = PatchBuilder::default()
        .supported_binarys(bytes_to_client_type(binary_byte))
        .replacement_mode(replacement_type)
        .pattern(to_replace.to_vec())
        .replacement(replacement.to_vec())
        .build()
        .unwrap();

    Ok(patch)
}
