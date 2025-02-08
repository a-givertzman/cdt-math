use std::sync::mpsc::{self, Receiver, Sender};
use sal_sync::services::entity::{name::Name, point::{point::Point, point_tx_id::PointTxId}};
use serde::Serialize;

use super::str_err::str_err::StrErr;
///
/// Contains local side `send` & `recv` of `channel`
/// - provides simple direct to `send` & `recv`
/// - provides request operation
pub struct Link {
    txid: usize,
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
    pub fn new(parent: impl Into<String>, send: Sender<Point>, recv: Receiver<Point>,) -> Self {
        let name = Name::new(parent, "Link");
        Self {
            txid: PointTxId::from_str(&name.join()),
            send, 
            recv,
        }
    }
    ///
    /// Returns `local: [Link] remote: [Link]` new instance
    pub fn split(parent: impl Into<String>) -> (Self, Self) {
        let name = Name::new(parent, "Link");
        let (loc_send, rem_recv) = mpsc::channel();
        let (rem_send, loc_recv) = mpsc::channel();
        (
            Self { 
                txid: PointTxId::from_str(&name.join()),
                send: loc_send, recv: loc_recv
            },
            Self { 
                txid: PointTxId::from_str(&name.join()),
                send: rem_send, recv: rem_recv
            },
        )
    }
    ///
    /// - Sends a request, 
    /// - Await reply,
    /// - Returns parsed reply
    pub fn req<T>(&self, query: impl Serialize) -> Result<T, StrErr> {
        match serde_json::to_string(&query) {
            Ok(query) => {
                // let bytes = query.as_bytes();
                let query = Point::new(self.tx_id, self.name, query);
                self.
            }
            Err(err) => Err(StrErr(format!("..."))),
        }
    }
}