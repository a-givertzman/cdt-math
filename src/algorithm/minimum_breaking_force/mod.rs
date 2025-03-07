//! # Minimum Breaking Force Calculation Module
//!
//! This module is responsible for calculating the minimum breaking force
//! based on the [maximum force in hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md) and [safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md).
//!
//! It consists of two main components:
//!
//! - `min_break_force_ctx`: Stores the minimum breaking force value.
//! - `min_break_force`: Implements the calculation logic.
//!
//! ## Usage
//!
//! The calculation process determines the minimum breaking force
//! based on the [maximum force in hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md) and [safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md).
//! The results are stored in `MinBreakForceCtx` and can be accessed
//! within the algorithm's execution context.
//!
//! For more details, refer to the design document:
//! [Minimum Breaking Force](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::minimum_breaking_force::MinBreakForce;
//!
//! let mut min_break_force = MinBreakForce::new(previous_step);
//! let result = min_break_force.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Calculated minimum breaking force: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid minimum breaking force could be determined."),
//! }
//! ```
//!
//! ## Components
//!
//! - `MinBreakForceCtx`: Stores the calculated minimum breaking force.
//! - `MinBreakForce`: Implements the evaluation logic for determining the minimum breaking force.
//!
//! This module is a part of the crane design algorithm.
pub mod min_break_force_ctx;
pub mod min_break_force;
