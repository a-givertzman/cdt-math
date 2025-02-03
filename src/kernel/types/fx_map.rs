use indexmap::IndexMap;
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};
///
/// Lightweght faster HashMap
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
///
/// Lightweght faster HashMap
pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
