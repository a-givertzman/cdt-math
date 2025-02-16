use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
///
/// Enum to structurize [types of hoisting tope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub enum RopeType {
    Metal,
    Synthetic
}