extern crate rbxlib;
use rbxlib::{
    enums,
    enums::BinaryType,
    structs::{BinaryBuilder, Patch, PatchBuilder},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
    thread::sleep,
    time::Duration,
};
use sysinfo::{ProcessExt, System, SystemExt};

use crate::{errors::CantFindRobloxUwp, patches::get_patches};
mod errors;
mod http;
mod patches;

const RBX_BYTES: &[u8] = b"roblox.com";
const KORTEX_BYTES: &[u8] = b"roblox.lol";

fn main() -> rbxlib::Result<()> {
    println!(
        "
MADE BY SOS DISCORD ON V3RMILLION
"
    );

    println!("Waiting for roblox uwp ...");

    let mut uwp_roblox_path: Option<PathBuf> = None;

    while uwp_roblox_path == None {
        let system = System::new_all();
        for process in system.processes_by_exact_name("Windows10Universal.exe") {
            if uwp_roblox_path.is_some() {
                break;
            }
            let proces_path = process.exe();
            uwp_roblox_path = Some(proces_path.to_path_buf());
        }
    }

    let Some(uwp_roblox_path) = uwp_roblox_path else {
        return Err(CantFindRobloxUwp.into());
    };

    let can_read = std::fs::metadata(&uwp_roblox_path)?
        .permissions()
        .readonly();

    /*
    if !can_read {
        println!("Please run uwp roblox from a non microsoft store build");
        return Ok(());
    }
    */
    let bytes = fs::read(&uwp_roblox_path)?;

    let mut binary = BinaryBuilder::default()
        .binary_type(BinaryType::Client)
        .client_bytes(bytes)
        .build()?;

    let patches = get_patches()?;

    println!("Applying patches");
    for patch in patches {
        binary.apply_patch(&patch)?;
    }
    println!("Patches applied");

    println!("Killing running process");

    let system = System::new_all();
    for process in system.processes_by_exact_name("Windows10Universal.exe") {
        process.kill();
    }

    println!("Saving exe to {:?}", uwp_roblox_path.display());

    let mut res = fs::write(&uwp_roblox_path, &binary.client_bytes);

    if res.is_err() {
        println!("Failed to save program");
        println!("Saving to folder");

        fs::write(
            uwp_roblox_path
                .parent()
                .unwrap()
                .join("./Windows10Patched.exe"),
            &binary.client_bytes,
        )?;

        println!("Retrying modifying main program if this last too long press control c");
        while res.is_err() {
            res = fs::write(&uwp_roblox_path, &binary.client_bytes);
            sleep(Duration::from_secs_f64(0.5));
        }
    }

    println!("Patched to latest version and disabled client telementry");

    println!("Press control C to exit (IGNORE error msg on exit)");
    loop {}

    Ok(())
}
