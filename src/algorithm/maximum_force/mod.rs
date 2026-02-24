//! # Maximum Force In Hoisting Rope Calculation Module
//!
//! This module is responsible for calculating the maximum force in hoisting rope
//! during the crane configuration process.
//!
//! ## Structure
//!
//! It consists of two main components:
//!
//! - `max_force_ctx`: Stores the calculated maximum force in hoisting rope value.
//! - `max_force`: Implements the logic for evaluating the maximum force in hoisting rope.
//!
//! ## Usage
//!
//! The  maximum force in hoisting rope calculation process determines the appropriate configuration
//! based on user input or predefined parameters.
//!
//! For more details, refer to the design document:
//! [Maximum Force In Hoisting Rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::max_force::MaxForce;
//!
//! let mut hoist_tackle_eff_coeff = MaxForce::new(link, request, previous_step);
//! let result = hoist_tackle_eff_coeff.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Calculated maximum force in hoisting rope: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid selection provided."),
//! }
//! ```
//!
//! ## Components
//!
//! - `MaxForceCtx`: Stores the calculated maximum force in hoisting rope.
//! - `MaxForce`: Implements the evaluating logic.
//!
//! This module is an essential part of the crane design algorithm,
//! that calculate maximum force in hoisting rope.
pub mod max_force_ctx;
pub mod max_force;