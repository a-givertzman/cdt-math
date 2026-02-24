//! # Module of Constants
//!
//! This module represent's constant's for calculating algorithm's
//! during the crane configuration process.
//!
//! ## Structure
//!
//! It consists of one main components:
//!
//! - `common`: Stores common constants.
//!
//! ## Usage
//!
//! The constants are using by alone algorithm's step.
//! 
//! More information about each constant describe in components file
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::constants::common;
//!
//! log::debug!("Value PI: {}", common::PI);
//! log::debug!("Value G: {}", common::G);
//! 
//! ```
//!
//! ## Components
//!
//! - `common`: Stores common constants.
//!
//! This module is an essential part of the crane design,
//! by stores constants, for algorithms.
pub mod common;