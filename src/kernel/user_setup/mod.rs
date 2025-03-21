//! # User Selection Module
//!
//! This module handles user input for selecting bearings and hooks during the crane design process.
//! It consists of four main components:
//!
//! - `user_bearing_ctx`: Stores the selected user bearing.
//! - `user_bearing`: Handles user interaction for bearing selection.
//! - `user_hoist_rope_ctx`: Stores the selected user hoist rope
//! - `user_hoist_rope`: Handles user interaction for hoist rope selection 
//! - `user_hook_ctx`: Stores the selected user hook.
//! - `user_hook`: Handles user interaction for hook selection.
//!
//! ## Usage
//!
//! This module provides a way for the user to manually choose a bearing and a hook based on available options.
//! The selection is stored in `UserBearingCtx`, `UserHookCtx` and `UserHoistRopeCtx`, which can be accessed within the algorithm's execution context.
//!
//! For more details, refer to the design document:
//! [User Hook](design/docs/algorithm/part02/chapter_01_choose_hook.md)
//! [User Bearing](design/docs/algorithm/part02/chapter_01_choose_hook.md)
//! [User Hoisting Rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
//! 
//!
//! ## Example
//!
//! ```rust
//! use crate::algorithm::context::ctx_result::CtxResult;
//! use crate::kernel::eval::Eval;
//! use crate::algorithm::user_bearing::UserBearing;
//! use crate::algorithm::user_hook::UserHook;
//! use crate::kernel::request::Request;
//! use crate::algorithm::entities::bearing::Bearing;
//! use crate::algorithm::entities::hook::Hook;
//!
//! let bearing_request = Request::<Bearing>::new();
//! let hook_request = Request::<Hook>::new();
//!
//! let mut user_bearing = UserBearing::new(bearing_request, previous_step);
//! let mut user_hook = UserHook::new(hook_request, previous_step);
//! let mut user_hoist_rope = UserHoistRope::new(hoist_rope_request, previous_step);
//!
//! let bearing_result = user_bearing.eval();
//! let hook_result = user_hook.eval();
//! let hoist_rope_result = user_hoist_rope.eval();
//!
//! match (bearing_result, hook_result, hoist_rope_result) {
//!     (CtxResult::Ok(ctx_b), CtxResult::Ok(ctx_h), CtxResult::Ok(ctx_h_r)) => log::debug!("User selections: {:?}, {:?}", ctx_b, ctx_h, ctx_h_r),
//!     _ => log::debug!("Error in user selection."),
//! }
//! ```
//!
//! ## Components
//!
//! - `UserBearingCtx`: Stores the selected bearing chosen by the user.
//! - `UserBearing`: Implements the logic for requesting a bearing from the user.
//! - `UserHoistRopeCtx`: Stores the selected hoisting rope chosen by the user
//! - `UserHoistRope`: Implements the logic for requesting a hoisting rope from the user
//! - `UserHookCtx`: Stores the selected hook chosen by the user.
//! - `UserHook`: Implements the logic for requesting a hook from the user.
//!
//! This module ensures that the user can participate in the selection process while maintaining the structured evaluation pipeline.
pub mod user_bearing_ctx;
pub mod user_bearing;
pub mod user_hoist_rope_ctx;
pub mod user_hoist_rope;
pub mod user_hook_ctx;
pub mod user_hook;
