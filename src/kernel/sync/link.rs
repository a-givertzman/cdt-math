use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc}, time::Duration};
use sal_sync::services::entity::{cot::Cot, name::Name, point::{point::Point, point_hlr::PointHlr, point_tx_id::PointTxId}, status::status::Status};
use serde::{de::DeserializeOwned, Serialize};
use tokio::task::JoinHandle;
use crate::{algorithm::context::ctx_result::CtxResult, kernel::str_err::str_err::StrErr};
///
/// Contains local side `send` & `recv` of `channel`
/// - provides simple direct to `send` & `recv`
/// - provides request operation
pub struct Link {
    txid: usize,
    name: Name,
    send: Sender<Point>,
    recv: Option<Receiver<Point>>,
    timeout: Duration,
    exit: Arc<AtomicBool>,
}
//
//
impl Link {
    ///
    /// Default timeout to await `recv`` operation, 300 ms
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(900);
    ///
    /// Returns [Link] new instance
    /// - `send` - local side of channel.send
    /// - `recv` - local side of channel.recv
    /// - `exit` - exit signal for `recv_query` method
    pub fn new(parent: impl Into<String>, send: Sender<Point>, recv: Receiver<Point>) -> Self {
        let name = Name::new(parent, "Link");
        Self {
            txid: PointTxId::from_str(&name.join()),
            name,
            send, 
            recv: Some(recv),
            timeout: Self::DEFAULT_TIMEOUT,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns it's name
    pub fn name(&self) -> Name {
        self.name.clone()
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
                send: loc_send, recv: Some(loc_recv),
                timeout: Self::DEFAULT_TIMEOUT,
                exit: Arc::new(AtomicBool::new(false)),
            },
            Self { 
                txid: PointTxId::from_str(&name.join()),
                name,
                send: rem_send, recv: Some(rem_recv),
                timeout: Self::DEFAULT_TIMEOUT,
                exit: Arc::new(AtomicBool::new(false)),
            },
        )
    }
    ///
    /// - Sends a request, 
    /// - Await reply,
    /// - Returns parsed reply
    pub async fn req<T: DeserializeOwned + Debug + Send>(&self, query: impl Serialize + Debug) -> Result<T, StrErr> {
        match serde_json::to_string(&query) {
            Ok(query) => {
                let query = Point::String(PointHlr::new(
                    self.txid, &self.name.join(),
                    query, Status::Ok, Cot::Req,
                    chrono::offset::Utc::now(),
                ));
                let timeout = self.timeout;
                let timeout = Duration::from_secs(3);
                match self.send.send(query.clone()) {
                    Ok(_) => {
                        log::debug!("{}.req | Sent request: {:#?}", self.name, query);
                        let h = tokio::task::block_in_place(move|| {
                            let r = match &self.recv {
                                Some(recv) => match recv.recv_timeout(timeout) {
                                    Ok(reply) => {
                                        log::debug!("{}.req | Received reply: {:#?}", self.name, reply);
                                        let reply = reply.as_string().value;
                                        match serde_json::from_str::<T>(reply.as_str()) {
                                            Ok(reply) => {
                                                Ok(reply)
                                            }
                                            Err(err) => Err(StrErr(format!("{}.req | Deserialize error for {:?} in {}, \n\terror: {:#?}", self.name, std::any::type_name::<T>(), reply, err))),
                                        }
                                    }
                                    _ => Err(StrErr(format!("{}.req | Request timeout ({:?})", self.name, timeout))),
                                }
                                None => todo!(),
                            };
                            r
                        });
                        h
                    },
                    Err(err) => Err(StrErr(format!("{}.req | Send request error: {:#?}", self.name, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.req | Serialize query error: {:#?}, \n\tquery: {:#?}", self.name, err, query))),
        }
    }
    ///
    /// Listenning incomong events in the callback
    /// - Callback returns Ok<T> if channel has event
    /// - Callback returns None if channel is empty for now
    /// - Callback returns Err if channel is closed
    pub async fn listen<In: DeserializeOwned + Debug + 'static, Out: Serialize + Debug + 'static>(&mut self, op: Box<dyn Fn(In) -> Out + Send + Sync + 'static>) -> JoinHandle<()> {
        let dbg = self.name.join();
        let txid = self.txid;
        let send = self.send.clone();
        let recv = self.recv.take().unwrap();
        let timeout = self.timeout;
        let exit = self.exit.clone();
        log::debug!("{}.listen | Starting...", dbg);
        let handle = tokio::spawn(async move {
            log::debug!("{}.listen | Start", dbg);
            tokio::task::block_in_place(move|| {
                'main: loop {
                    match recv.recv_timeout(timeout) {
                        Ok(query) => {
                            log::debug!("{}.listen | Received query: {:#?}", dbg, query);
                            let name = query.name();
                            let quyru = query.as_string().value;
                            match serde_json::from_str::<In>(quyru.as_str()) {
                                Ok(query) => {
                                    let query: In = query;
                                    let reply: Out = (op)(query);
                                    match serde_json::to_string(&reply) {
                                        Ok(reply) => {
                                            let reply = Point::String(PointHlr::new(
                                                txid, &name,
                                                reply, Status::Ok, Cot::ReqCon,
                                                chrono::offset::Utc::now(),
                                            ));
                                            match send.send(reply) {
                                                Ok(_) => {}
                                                Err(err) => {
                                                    let err = StrErr(format!("{}.listen | Send request error: {:#?}", dbg, err));
                                                    log::error!("{}", err);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            let err = StrErr(format!("{}.listen | Serialize reply error: {:#?}, \n\tquery: {:#?}", dbg, err, reply));
                                            log::debug!("{}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::warn!("{}.listen | Deserialize error for {:?} in {}, \n\terror: {:#?}", dbg, std::any::type_name::<In>(), quyru, err);
                                }
                            }
                        }
                        Err(err) => {
                            match err {
                                std::sync::mpsc::RecvTimeoutError::Timeout => {}
                                std::sync::mpsc::RecvTimeoutError::Disconnected => {
                                    log::error!("{}.listen | Recv error: {:#?}", dbg, err);
                                }
                            }
                        }
                    }
                    if exit.load(Ordering::SeqCst) {
                        break 'main;
                    }
                }
                log::debug!("{}.listen | Exit", dbg);
            });
        });
        let dbg = self.name.join();
        log::debug!("{}.listen | Starting - Ok", dbg);
        handle
    }
    ///
    /// Receiving incomong events
    /// - Returns Ok<T> if channel has query
    /// - Returns None if channel is empty for now
    /// - Returns Err if channel is closed
    pub async fn recv_query<T: DeserializeOwned + Debug>(&self) -> CtxResult<T, StrErr> {
        let h = tokio::task::block_in_place(move|| {
            match &self.recv {
                Some(recv) => match recv.recv_timeout(self.timeout) {
                    Ok(query) => {
                        log::debug!("{}.recv_query | Received query: {:#?}", self.name, query);
                        let quyru = query.as_string().value;
                        match serde_json::from_str::<T>(quyru.as_str()) {
                            Ok(query) => {
                                return CtxResult::Ok(query)
                            }
                            Err(err) => CtxResult::Err(
                                StrErr(
                                    format!("{}.req | Deserialize error for {:?} in {}, \n\terror: {:#?}",
                                    self.name, std::any::type_name::<T>(), quyru, err),
                                ),
                            ),
                        }
                    }
                    Err(err) => {
                        match err {
                            std::sync::mpsc::RecvTimeoutError::Timeout => CtxResult::None,
                            std::sync::mpsc::RecvTimeoutError::Disconnected => CtxResult::Err(
                                StrErr(format!("{}.req | Recv error: {:#?}", self.name, err)),
                            ),
                        }
                    }
                }
                None => todo!(),
            }
        });
        h
    }
    ///
    /// Receiving incomong events with sender name
    /// - Returns Ok<T> if channel has query
    /// - Returns None if channel is empty for now
    /// - Returns Err if channel is closed
    pub async fn recv_query_from<T: DeserializeOwned + Debug>(&self) -> CtxResult<(String, T), StrErr> {
        let h = tokio::task::block_in_place(move|| {
            match &self.recv {
                Some(recv) => match recv.recv_timeout(self.timeout) {
                    Ok(query) => {
                        log::debug!("{}.recv_query | Received query: {:#?}", self.name, query);
                        let name = query.name();
                        let quyru = query.as_string().value;
                        match serde_json::from_str::<T>(quyru.as_str()) {
                            Ok(query) => {
                                return CtxResult::Ok((name, query))
                            }
                            Err(err) => CtxResult::Err(
                                StrErr(
                                    format!("{}.req | Deserialize error for {:?} in {}, \n\terror: {:#?}",
                                    self.name, std::any::type_name::<T>(), quyru, err),
                                ),
                            ),
                        }
                    }
                    Err(err) => {
                        match err {
                            std::sync::mpsc::RecvTimeoutError::Timeout => CtxResult::None,
                            std::sync::mpsc::RecvTimeoutError::Disconnected => CtxResult::Err(
                                StrErr(format!("{}.req | Recv error: {:#?}", self.name, err)),
                            ),
                        }
                    }
                }
                None => todo!(),
            }
        });
        h
    }
    ///
    /// Sending event
    pub fn send_reply(&self, reply: impl Serialize + Debug) -> Result<(), StrErr> {
        match serde_json::to_string(&reply) {
            Ok(reply) => {
                let reply = Point::new(self.txid, &self.name.join(), reply);
                match self.send.send(reply) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StrErr(format!("{}.reply | Send request error: {:#?}", self.name, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.reply | Serialize reply error: {:#?}, \n\tquery: {:#?}", self.name, err, reply))),
        }
    }
    ///
    /// Sends "exit" signal to the `listen` task
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
unsafe impl Sync for Link {}
//
//
impl Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link")
        .field("txid", &self.txid)
        .field("name", &self.name)
        // .field("send", &self.send)
        // .field("recv", &self.recv)
        .field("timeout", &self.timeout)
        .finish()
    }
}
// ///
// /// Async callback closure
// trait AsyncFn<In, Out> {
//     fn eval(&self, ctx: In) -> BoxFuture<'_, Out>;
// }
// //
// //
// impl<T, F, In, Out> AsyncFn<In, Out> for T
// where
//     T: Fn(In) -> F,
//     F: std::future::Future<Output = Out> + Send + 'static,
// {
//     fn eval(&self, val: In) -> BoxFuture<'_, Out> {
//         Box::pin(self(val))
//     }
// }
