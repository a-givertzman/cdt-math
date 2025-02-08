use std::sync::mpsc::{self, Receiver, Sender};
use sal_sync::services::entity::point::point::Point;
///
/// Contains local side `send` & `recv` of `channel`
/// - provides simple direct to `send` & `recv`
/// - provides request operation
pub struct Link {
    send: Sender<Point>,
    recv: Receiver<Point>,
}
//
//
impl Link {
    ///
    /// Returns [Link] new instance
    /// - send - local side of channel.send
    /// - recv - local side of channel.recv
    pub fn new(send: Sender<Point>, recv: Receiver<Point>,) -> Self {
        Self {
            send, 
            recv,
        }
    }
    ///
    /// Returns `local: [Link] remote: [Link]` new instance
    pub fn split() -> (Self, Self) {
        let (loc_send, rem_recv) = mpsc::channel();
        let (rem_send, loc_recv) = mpsc::channel();
        (
            Self { send: loc_send, recv: loc_recv },
            Self { send: rem_send, recv: rem_recv },
        )
    }
    ///
    /// - Sends a request, 
    /// - Await reply,
    /// - Returns parsed reply
    pub fn req<T>(query: Serialize) -> Result<T, StrErr> {
        match serde_json::to_string(query) {
            Ok(query) => {
                let bytes = query.as_bytes()

            }
            Err(err) => Err(StrErr(...)),
        };
    }
}