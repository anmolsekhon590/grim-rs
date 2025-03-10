use crate::commands::args::FileType;

use super::args;

pub fn set_filetype(_filetype: FileType) -> args::FileType {
    // Add logic to set filetype here 
    return FileType::Png;
}
