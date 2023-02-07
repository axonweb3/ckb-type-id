// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/

use crate::error::Error;
use crate::type_id::{load_type_id_from_script_args, validate_type_id};

pub fn main() -> Result<(), Error> {
    // remove below examples and write your code here

    let type_id = load_type_id_from_script_args(0)?;

    validate_type_id(type_id)?;

    Ok(())
}
