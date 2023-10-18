use std::{ fmt, error };

#[derive(Debug, Clone, Copy)]
pub struct CouldntFindBytePattern;

impl fmt::Display for CouldntFindBytePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Couldnt find bytes in provided list")
    }
}

impl error::Error for CouldntFindBytePattern {}

#[derive(Debug, Clone, Copy)]
pub struct CouldntConvertToi64;

impl fmt::Display for CouldntConvertToi64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The number is not compatible to be converted to I64")
    }
}

impl error::Error for CouldntConvertToi64 {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidPatch;

impl fmt::Display for InvalidPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The patch provided is not supported on this binary or it is invalid")
    }
}

impl error::Error for InvalidPatch {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidBinaryType;

impl fmt::Display for InvalidBinaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Binary type in patch is invalid")
    }
}

impl error::Error for InvalidBinaryType {}

#[derive(Debug, Clone, Copy)]
pub struct InvalidReplacementMode;

impl fmt::Display for InvalidReplacementMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The Replacement mode is invalid")
    }
}

impl error::Error for InvalidReplacementMode {}

#[derive(Debug, Clone, Copy)]
pub struct BadEncodedPatch;

impl fmt::Display for BadEncodedPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The bytes provided are bad")
    }
}

impl error::Error for BadEncodedPatch {}
