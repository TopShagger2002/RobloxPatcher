use rbxlib::enums::BinaryType;
use rbxlib::structs::{Patch, PatchBuilder};
use rbxlib::Result;

use crate::http;

// TelementryUrl and its replacement
const TELEMENTRY_URL: &[u8; 27] = b"client-telemetry.roblox.com";
const NEW_TELEMENTRY_URL: &[u8; 27] = b"example.example.example.com";

const LAST_NON_HYPERION: &[u8] = b"2.592.568";
const last_non_hyp: [u8; 9] = [0x32, 0x2E, 0x35, 0x39, 0x32, 0x2E, 0x35, 0x38, 0x36];

pub fn get_patches() -> Result<Vec<Patch>> {
    /*
    let telementry_patch = PatchBuilder::default()
        .pattern(TELEMENTRY_URL.to_vec())
        .replacement(NEW_TELEMENTRY_URL.to_vec())
        .supported_binarys(vec![BinaryType::Client])
        .replacement_mode(rbxlib::enums::ReplaceMentMode::All)
        .build()?;
    */
    /* Download latest uwp version */
    let str =
        http::download_url("https://raw.githubusercontent.com/Cesare0328/Lx0hTx/main/AndroidRbx")
            .unwrap();
    let str_vec: Vec<&str> = str.lines().collect();
    let version = str_vec[0];

    let version_patch = PatchBuilder::default()
        .pattern(last_non_hyp.to_vec())
        .replacement(version.as_bytes().to_vec())
        .supported_binarys(vec![BinaryType::Client])
        .replacement_mode(rbxlib::enums::ReplaceMentMode::All)
        .build()?;

    Ok(vec![version_patch])
}
