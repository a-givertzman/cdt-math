//! # Module of calculating steady state lifting speed
//!
//! This module constist method of calculating steady state lifting speed
//! and their processing in the system. 
//! [reference to steady-state lifting speed documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
//! It includes:
//! - calculating steady state lifting speed (`steady_state_lifting_speed`)
//! - calculating half of nominal lifting speed of the mechanism (`payload_weight`)
//!
//! ## Example of using
//! ```rust
//! use crate::algorithm::lifting_speed::lifting_speed::LiftingSpeed;
//! use crate::kernel::initial_data::initial_data::InitialData;
//! let lifting_speed = LiftingSpeed::new(InitialData::new()).eval();
//! println!("Steady state lifiting speed: {}", total);
//! ```
pub mod lifting_speed;