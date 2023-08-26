//! Status line for shells with ANSI escape sequences support
//!
//! This is a documentation for statusline API, use `README.md` for executable documentation
//!
//! # Example
//!
//! ```
//! use statusline::{Bottom, CommandLineArgs, Icons, Pretty, Top};
//!
//! let icons = Icons::MinimalIcons;
//! let args = CommandLineArgs::from_env::<&str>(&[]);
//! let top = Top::from_env(&args);
//! let bottom = Bottom::from_env(&args);
//! println!("{}", top.to_title(Some("test")));
//! println!("{}", top.pretty(&icons));
//! print!("{}", bottom.pretty(&icons));  // Or you can use readline with result as prompt
//!
//! // And, additionally, you can start a separate thread for getting more info
//! // which should be outputed "over" the first top line
//! ```

#![feature(byte_slice_trim_ascii)]
#![feature(io_error_more)]
#![feature(iter_next_chunk)]
#![feature(fn_traits)]
#![feature(fs_try_exists)]
#![feature(let_chains)]
#![feature(slice_first_last_chunk)]
#![feature(stdsimd)]
#![feature(unboxed_closures)]

mod args;
mod block;
mod bottom;
mod chassis;
mod git;
mod icon;
mod style;
mod time;
mod top;
mod venv;

/// Filesystem-related operations
pub mod file;

/// Virtualization detector (not tested tho)
pub mod virt;

pub use crate::{
    args::{Environment, FromEnv},
    block::{autopretty, Block, BlockType},
    bottom::Bottom,
    chassis::Chassis,
    git::{GitStatus, GitStatusExtended},
    icon::{Icon, Icons, Pretty},
    style::{Style, Styled},
    top::Top,
};
