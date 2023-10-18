//use serde::/* Deserialize */;
use derive_builder::Builder;
use super::errors::*;
use super::enums::*;
use super::Result;

use super::{
    errors::{ InvalidPatch, InvalidBinaryType, InvalidReplacementMode },
    functions::{
        replace_all_instances_of_bytes,
        replace_first_instance_of_bytes,
        replace_last_instance_of_bytes,
        replace_closest_to_index,
    },
};

#[derive(Clone, Debug, /* Deserialize */)]
pub struct JsonPatch {
    pub supported_binarys: Vec<String>,
    pub replacement_mode: String /* 1 - 3 */,
    pub pattern: Vec<u8>,
    pub replacement: Vec<u8>,
    pub closest_to: Option<usize>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct Patch {
    pub supported_binarys: Vec<BinaryType>,
    pub replacement_mode: ReplaceMentMode /* 1 - 3 */,
    pub pattern: Vec<u8>,
    pub replacement: Vec<u8>,
    //pub closest_to: Option<usize>,
}

#[derive(Builder, Clone, Debug)]
pub struct Binary {
    pub client_bytes: Vec<u8>,
    pub binary_type: BinaryType,
}

impl JsonPatch {
    pub fn as_patch(&self) -> Result<Patch> {
        let mut supported_binarys = vec![];
        for i in &self.supported_binarys {
            let converted = BinaryType::from_string(i)?;
            supported_binarys.push(converted);
        }
        let replacement_mode = ReplaceMentMode::from_string(&self.replacement_mode)?;

        let patch_builder = PatchBuilder::default()
            .supported_binarys(supported_binarys)
            .replacement_mode(replacement_mode)
            .pattern(self.pattern.to_owned())
            .replacement(self.replacement.to_owned())
            //.closest_to(self.closest_to.to_owned())
            .build();

        let patch = patch_builder?;
        Ok(patch)
    }
}

impl Binary {
    pub fn apply_patch(&mut self, patch: &Patch) -> Result<()> {
        if !patch.supported_binarys.contains(&self.binary_type) {
            return Err(InvalidPatch.into());
        }

        let mut self_mut = self;

        match patch.replacement_mode {
            ReplaceMentMode::All => {
                replace_all_instances_of_bytes(
                    &mut self_mut.client_bytes,
                    &patch.pattern,
                    &patch.replacement
                )?;
            }
            ReplaceMentMode::First => {
                replace_first_instance_of_bytes(
                    &mut self_mut.client_bytes,
                    &patch.pattern,
                    &patch.replacement
                )?;
            }
            ReplaceMentMode::Last => {
                replace_last_instance_of_bytes(
                    &mut self_mut.client_bytes,
                    &patch.pattern,
                    &patch.replacement
                )?;
            }
            /*  ReplaceMentMode::Closest => {
                let res: Result<usize> = patch.closest_to.ok_or(InvalidPatch.into());
                let index = res?;

                replace_closest_to_index(
                    &mut self_mut.client_bytes,
                    &index,
                    &patch.pattern,
                    &patch.replacement
                )?;
            }
            */
        }

        Ok(())
    }
}
