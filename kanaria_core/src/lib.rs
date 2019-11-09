#[macro_use]
extern crate bitflags;

pub use self::char::UCSChar;
pub use self::str::UCSStr;

pub mod constants;
pub mod utils;
pub mod char;
pub mod str;
