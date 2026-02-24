//! # Minimum Tequired Breaking Force In Rope Calculation Module
//!
//! This module is responsible for calculating the minimum required breaking force in rope
//! based on the rope [safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md) and [maximum force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md).
//!
//! It consists of two main components:
//!
//! - [`min_break_force`]: Stores the calculated minimum required breaking force in rope value.
//! - [`min_break_force_ctx`]: Implements the calculation logic.
//!
//! ## Usage
//!
//! The calculation process determines the necessary minimum required breaking force in rope based
//! //! based on the rope [safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md) and [maximum force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md).
//! The results are stored in [MinBreakForceCtx] and can be accessed within the algorithm's execution context.
//!
//! For more details, refer to the design document:
//! [Minimum Tequired Breaking Force In Rope Calculation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//! [Table of choice](references\GOST_33710-2015.pdf)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::min_break_force::min_break_force::MinBreakForce;
//!
//! let mut min_break_force = MinBreakForce::new(previous_step);
//! let result = min_break_force.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Calculated rope safety factor: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid minimum required breaking force in rope provided."),
//! }
//! ```
//!
//! ## Components
//!
//! - [MinBreakForceCtx]: Stores the calculated minimum required breaking force in rope value.
//! - [MinBreakForce]: Implements the evaluation logic for determining minimum required breaking force in rope.
//!
//! This module is a part of the crane design algorithm.
pub mod min_break_force_ctx;
pub mod min_break_force;

