//! # Hoisting Tackle Effiency Coefficient Calculation Module
//!
//! This module is responsible for calculating the hoisting tackle effiency coefficient
//! during the crane configuration process.
//!
//! ## Structure
//!
//! It consists of two main components:
//!
//! - `hoist_tackle_eff_coeff_ctx`: Stores the calculated hoisting tackle effiency coefficient value.
//! - `hoist_tackle_eff_coeff`: Implements the logic for evaluating the hoisting tackle effiency coefficient.
//!
//! ## Usage
//!
//! The hoisting tackle effiency coefficient calculation process determines the appropriate configuration
//! based on user input or predefined parameters.
//!
//! For more details, refer to the design document:
//! [Hoisting Tackle Effiency Coefficient](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::hoist_tackle_eff_coeff::HoistTackleEffCoeff;
//!
//! let mut hoist_tackle_eff_coeff = HoistTackleEffCoeff::new(link, request, previous_step);
//! let result = hoist_tackle_eff_coeff.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Calculated hoisting tackle effiency coefficient: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid selection provided."),
//! }
//! ```
//!
//! ## Components
//!
//! - `HoistTackleEffCoeffCtx`: Stores the selected hoisting tackle effiency coefficient.
//! - `HoistTackleEffCoeff`: Implements the evaluating logic.
//!
//! This module is an essential part of the crane design algorithm,
//! that calculate hoisting tackle effiency coefficient.
pub mod hoist_tackle_eff_coeff_ctx;
pub mod hoist_tackle_eff_coeff;