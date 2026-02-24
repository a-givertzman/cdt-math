//! # Hoisting Tackle Multiplicity Selection Module
//!
//! This module is responsible for calculating the hoisting tackle multiplicity
//! during the crane configuration process.
//!
//! ## Structure
//!
//! It consists of two main components:
//!
//! - `hoist_tackle_multi_ctx`: Stores the selected hoisting tackle multiplicity value.
//! - `hoist_tackle_multi`: Implements the logic for evaluating and adjusting the hoisting tackle multiplicity.
//!
//! ## Usage
//!
//! The hoisting tackle multiplicity calculation process determines the appropriate configuration
//! based on user input or predefined parameters.
//!
//! For more details, refer to the design document:
//! [Hoisting Tackle Multiplicity](design/docs/algorithm/part02/chapter_03_choose_hoisting_tackle.md)
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::hoisting_tackle_multiplicity::HoistTackleMulti;
//!
//! let mut hoist_tackle_multi = HoistTackleMulti::new(link, request, previous_step);
//! let result = hoisting_tackle.eval();
//!
//! match result {
//!     CtxResult::Ok(ctx) => log::debug!("Selected hoisting tackle multiplicity: {:?}", ctx),
//!     CtxResult::Err(err) => log::debug!("Error: {}", err),
//!     CtxResult::None => log::debug!("No valid selection provided."),
//! }
//! ```
//!
//! ## Components
//!
//! - `HoistTackleMultiCtx`: Stores the selected hoisting tackle multiplicity value.
//! - `HoistTackleMulti`: Implements the evaluating logic.
//!
//! This module is an essential part of the crane design algorithm,
//! that calculate hoisting tackle multiplicity.
pub mod hoist_tackle_multi_ctx;
pub mod hoist_tackle_multi;