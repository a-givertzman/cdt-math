///
/// Complete structured list of Application Events
#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Query(Query),
    /// Information event
    Inf(Info),
    /// Diagnostoc event
    Diag(Diag),
}
