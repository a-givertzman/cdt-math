//! # Module of calculating dynamic coefficient
//!
//! This module constist method of calculating steady state lifting speed
//! and their processing in the system. 
//! [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
//! It includes:
//! - calculating dynamic coefficient (`dynamic_coefficient`)
//!
//! ## Example of using
//! ```rust
//! use crate::algorithm::lifting_speed::lifting_speed::LiftingSpeed;
//! use crate::kernel::initial_data::initial_data::InitialData;
//! let dynamic_coefficient = DynamicCoefficient::new(SelectBettaPhi::new(InitialData::new()), LiftingSpeed::new(InitialData::new())).eval();
//! println!("dynamic coefficient: {}", dynamic_coefficient);
//! ```
pub mod dynamic_coefficient;