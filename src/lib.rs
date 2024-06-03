//! It being a cross-platform library to provides easy to use beep functions
//! that actually beep.
//!
//! # What is it?
//!
//! As mentioned above, it's a cross-platform library that provides very basic audio tones (beeps)
//! for whatever use you may have for that. It's built on the _AMAZING_
//! [CPAL crate](https://crates.io/crates/cpal), which provides the low level audio support,
//!& also provides excellent examples from which this crate stole heavily.
//!
//! # Why does this Exist?
//!
//! There are a variety of platform dependent crates that have a similar function, including:
//!
//! * [beep](https://crates.io/crates/beep)
//! * [beep-evdev](https://crates.io/crates/beep-evdev)
//! * [win-beep](https://crates.io/crates/win-beep)
//!
//! None of these are cross-platform, & they are all different. This crate fixes that by providing
//! a general beeping interface that works on all platforms.
//!
//! # Quick Start
//!
//! That's the only kind of start that you can have with this library:
//!
//! ```
//! use actually_beep::beep_with_hz_and_millis;
//!
//! let middle_e_hz = 329;
//! let a_bit_more_than_a_second_and_a_half_ms = 1600;
//!
//! beep_with_hz_and_millis(middle_e_hz, a_bit_more_than_a_second_and_a_half_ms).unwrap();
//! ```
//!
//! # Futures
//!
//! Currently, there is one call that blocks. There is no "beep indefinitely" call as
//! the `beep` crate provides, & it doesn't provide any way to adjust things like the
//! audio device, host audio system, or anything else.
//!
//! These things are potentially, possibly, something that may be worked on.

mod beep;
mod error;

pub use beep::beep_with_hz_and_millis;
pub use error::{Error, Result};
