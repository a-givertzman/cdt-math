use std::{fmt::Debug, sync::mpsc::{self, Receiver, RecvTimeoutError, Sender}, time::Duration};
use sal_sync::services::entity::{name::Name, point::{point::Point, point_tx_id::PointTxId}};
use serde::{de::DeserializeOwned, Serialize};
use super::str_err::str_err::StrErr;
///
/// Contains local side `send` & `recv` of `channel`
/// - provides simple direct to `send` & `recv`
/// - provides request operation
pub struct Link {
    txid: usize,
    name: Name,
    send: Sender<Point>,
    recv: Receiver<Point>,
    timeout: Duration,
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
            name,
            send, 
            recv,
            timeout: Duration::from_millis(3000),
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
                name: name.clone(),
                send: loc_send, recv: loc_recv,
                timeout: Duration::from_millis(3000),
            },
            Self { 
                txid: PointTxId::from_str(&name.join()),
                name,
                send: rem_send, recv: rem_recv,
                timeout: Duration::from_millis(3000),
            },
        )
    }
    ///
    /// - Sends a request, 
    /// - Await reply,
    /// - Returns parsed reply
    pub fn req<T: DeserializeOwned + Debug>(&self, query: impl Serialize + Debug) -> Result<T, StrErr> {
        match serde_json::to_string(&query) {
            Ok(query) => {
                // let bytes = query.as_bytes();
                let query = Point::new(self.txid, &self.name.join(), query);
                match self.send.send(query) {
                    Ok(_) => {
                        match self.recv.recv_timeout(self.timeout) {
                            Ok(reply) => {
                                let reply = reply.as_string().value;
                                match serde_json::from_str::<T>(reply.as_str()) {
                                    Ok(reply) => {
                                        Ok(reply)
                                    }
                                    Err(err) => Err(StrErr(format!("{}.req | Deserialize error for {:?} in {}, \n\terror: {:#?}", self.name, std::any::type_name::<T>(), reply, err))),
                                }
                            }
                            Err(_) => Err(StrErr(format!("{}.req | Request timeout ({:?})", self.name, self.timeout))),
                        }
                    },
                    Err(err) => Err(StrErr(format!("{}.req | Send request error: {:#?}", self.name, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.req | Serialize query error: {:#?}, \n\tquery: {:#?}", self.name, err, query))),
        }
    }
    ///
    /// Receiving incomong events
    pub fn recv<T: DeserializeOwned + Debug>(&self) -> Result<T, StrErr> {
        loop {
            match self.recv.recv_timeout(self.timeout) {
                Ok(quyru) => {
                    let quyru = quyru.as_string().value;
                    match serde_json::from_str::<T>(quyru.as_str()) {
                        Ok(quyru) => {
                            return Ok(quyru)
                        }
                        Err(err) => return Err(
                            StrErr(
                                format!("{}.req | Deserialize error for {:?} in {}, \n\terror: {:#?}",
                                self.name, std::any::type_name::<T>(), quyru, err),
                            ),
                        ),
                    }
                }
                Err(err) => {
                    match err {
                        RecvTimeoutError::Timeout => {},
                        RecvTimeoutError::Disconnected => return Err(
                            StrErr(format!("{}.req | Recv error: {:#?}", self.name, err)),
                        ),
                    }
                }
            }
        }
    }
    ///
    /// Sending event
    pub fn reply(&self, reply: impl Serialize + Debug) -> Result<(), StrErr> {
        match serde_json::to_string(&reply) {
            Ok(reply) => {
                let reply = Point::new(self.txid, &self.name.join(), reply);
                match self.send.send(reply) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StrErr(format!("{}.teply | Send request error: {:#?}", self.name, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.reply | Serialize reply error: {:#?}, \n\tquery: {:#?}", self.name, err, reply))),
        }
    }
}
//
//
unsafe impl Sync for Link {}