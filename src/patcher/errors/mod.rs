use std::{error, fmt};

#[derive(Debug, Clone, Copy)]
pub struct FileReadError;

impl fmt::Display for FileReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Couldnt read file")
    }
}

impl error::Error for FileReadError {}

#[derive(Debug, Clone, Copy)]
pub struct CantFindRobloxUwp;

impl fmt::Display for CantFindRobloxUwp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Couldnt find roblox uwp")
    }
}

impl error::Error for CantFindRobloxUwp {}
