use curl::easy::Easy;
use rbxlib::Result;
use std::{fs, io::Write, path::PathBuf, sync::mpsc, thread};

use crate::errors::FileReadError;
pub fn download_url(url: &str) -> Result<String> {
    let mut result_vec: Vec<u8> = vec![];
    let mut easy = Easy::new();
    easy.url(url)?;
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        result_vec.extend_from_slice(data);
        Ok(data.len())
    })?;
    transfer.perform()?;
    drop(transfer);

    let converted = String::from_utf8(result_vec.clone())?;

    Ok(converted)
}
