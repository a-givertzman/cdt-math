//! # Rope Safety Factor Calculation Module
//!
//! This module is responsible for calculating the rope safety factor
//! based on the user's mechanism work type, type of rope and type of winding.
//!
//! It consists of two main components:
//!
//! - [`safety_factor_ctx`]: Stores the calculated rope safety factor value.
//! - [`safety_factor`]: Implements the calculation logic.
//!
//! ## Usage
//!
//! The calculation process determines the necessary rope safety factor based
//! on user's mechanism work type, type of rope and type of winding. The results are stored
//! in [`SafetyFactorCtx`] and can be accessed within the algorithm's execution context.
//!
//! For more details, refer to the design document:
//! [Rope Safety Factor Calculation](design/docs/algorithm/part02/chapter_03_choose_hoisting_tackle.md)
//! [Table of choice](references\GOST_33710-2015.pdf)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::rope_safety_factor::safety_factor::SafetyFactor;
//!
//! let mut safety_factor = SafetyFactor::new(previous_step);
//! let result = safety_factor.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Calculated rope safety factor: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid rope safety factor provided."),
//! }
//! ```
//!
//! ## Components
//!
//! - [SafetyFactorCtx]: Stores the calculated rope safety factor value.
//! - [SafetyFactor]: Implements the evaluation logic for determining rope safety factor.
//!
//! This module is a part of the crane design algorithm.
pub mod safety_factor_ctx;
pub mod safety_factor;
pub mod select_coeff;
