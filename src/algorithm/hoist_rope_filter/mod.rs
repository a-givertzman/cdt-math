//! # Hoisting Ropes Filtering Module
//! 
//! This module is responsible for filtering hoisting ropes based on predefined criteria.
//! It consists of two main components:
//! 
//! - `[hoist_rope_filter_ctx]`: Stores the filtered hoisting ropes.
//! - `[hoist_rope_filter]`: Implements the filtering logic.
//! 
//! ## Usage
//! 
//! The filtering process selects hoisting ropes that meet specific conditions related to [minimum breaking force](src\algorithm\min_break_force).
//! The results are stored in `HoistRopeFilterCtx` and can be accessed within the algorithm's execution context.
//! 
//! For more details, refer to the design document:
//! [Filtering Bearings](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//! 
//! ## Example
//! 
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::entities::hoisting_rope::HoistingRope;
//! use crate::algorithm::hoist_rope_filter::HoistRopeFilter;
//! 
//! let mut hoist_rope_filter = HoistRopeFilter::new(previous_step);
//! let result = hoist_rope_filter.eval();
//! 
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Filtered hoisting ropes: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No hoisting ropes matched the criteria."),
//! }
//! ```
//! 
//! ## Components
//! 
//! - `HoistRopeFilterCtx`: Stores the list of hoisting ropes.
//! - `HoistRopeFilter`: Implements the evaluation logic for filtering hoisting ropes.
//! 
//! This module is a part of the crane design algorithm and is used to refine component selection.
pub mod hoist_rope_filter_ctx;
pub mod hoist_rope_filter;