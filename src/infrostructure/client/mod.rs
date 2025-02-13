//! 
//! # Client - Server event interface description
//! 
//! ## ChooseDriverType Requiest
//! 
//! Ascs user for type of the ... drive
//! 
//! - Cot::Req
//! 
//! ```json
//! {
//!     "req-name": "ChooseDriverType",
//!     "variant": []
//! }
//! ```
//! 
//! - Cot::ReqCon
//! 
//! ```json
//! {
//!     "req-name": "ChooseDriverType",
//!     "selection": "Selected Variant Id"
//! }
//! ```
//! 
//! - Cot::ReqErr
//! 
//! ```json
//! {
//!     "req-name": "ChooseDriverType",
//!     "error": "Error description"
//! }
//! ```
