use std::{fmt::Debug, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc}, time::Duration};
use sal_sync::{collections::map::IndexMapFxHasher, services::entity::{cot::Cot, error::str_err::StrErr, name::Name, point::{point::Point, point_tx_id::PointTxId}}};
use tokio::task::JoinSet;
use super::link::Link;
///
/// 
pub struct Switch {
    txid: usize,
    name: Name,
    send: Sender<Point>,
    recv: Option<Receiver<Point>>,
    subscribers: IndexMapFxHasher<String, Sender<Point>>,
    receivers: IndexMapFxHasher<String, Receiver<Point>>,
    timeout: Duration,
    exit: Arc<AtomicBool>,
}
//
//
impl Switch {
    ///
    /// Default timeout to await `recv`` operation, 300 ms
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(300);
    ///
    /// Returns [Switch] new instance
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
            subscribers: IndexMapFxHasher::with_hasher(BuildHasherDefault::default()),
            receivers: IndexMapFxHasher::with_hasher(BuildHasherDefault::default()),
            timeout: Self::DEFAULT_TIMEOUT,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns connected `Link`
    pub fn link(&mut self) -> Link {
        let (loc_send, rem_recv) = mpsc::channel();
        let (rem_send, loc_recv) = mpsc::channel();
        let remote = Link::new(&self.name, rem_send, rem_recv);
        let key = remote.name().join();
        self.subscribers.insert(key.clone(), loc_send);
        self.receivers.insert(key, loc_recv);
        remote
    }
    ///
    /// Entry point
    pub async fn run(&mut self) -> Result<JoinSet<()>, StrErr> {
        let dbg = self.name.join();
        let subscribers: IndexMapFxHasher<String, Sender<Point>> = self.subscribers.drain(0..).collect();
        let exit = self.exit.clone();
        let recv = self.recv.take().unwrap();
        let timeout = self.timeout.clone();
        let mut join_set = JoinSet::new();
        join_set.spawn(async move {
            'main: loop {
                match recv.recv_timeout(timeout) {
                    Ok(event) => {
                        log::debug!("{}.run | Request: {:?}", dbg, event);
                        match event.cot() {
                            Cot::Inf | Cot::Act | Cot::Req => {
                                for (_key, subscriber) in &subscribers {
                                    if let Err(err) = subscriber.send(event.clone()) {
                                        log::warn!("{}.run | Send error: {:?}", dbg, err);
                                    }
                                }
                            }
                            Cot::ReqCon | Cot::ReqErr => {
                                let key = event.name();
                                match subscribers.get(&key) {
                                    Some(subscriber) => {
                                        if let Err(err) = subscriber.send(event.clone()) {
                                            log::warn!("{}.run | Send error: {:?}", dbg, err);
                                        }
                                    },
                                    None => {},
                                }
                            }
                            _ => log::warn!("{}.run | Uncnown message received: {:?}", dbg, event),
                        }
                    },
                    Err(err) => match err {
                        std::sync::mpsc::RecvTimeoutError::Timeout => {},
                        std::sync::mpsc::RecvTimeoutError::Disconnected => panic!("{}.run | Receive error, all receivers has been closed", dbg),
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
        });
        let dbg = self.name.join();
        let send = self.send.clone();
        let receivers: IndexMapFxHasher<String, Receiver<Point>> = self.receivers.drain(0..).collect();
        let exit = self.exit.clone();
        join_set.spawn(async move {
            'main: loop {
                for (_key, receiver) in receivers.iter() {
                    match receiver.try_recv() {
                        Ok(event) => {
                            if let Err(err) = send.send(event) {
                                log::warn!("{}.run | Send error: {:?}", dbg, err)
                            }
                        }
                        Err(err) => match err {
                            mpsc::TryRecvError::Empty => {} //tokio::time::sleep(Duration::from_millis(1)).await,
                            mpsc::TryRecvError::Disconnected => panic!("{}.run | Receive error, all senders has been closed", dbg),
                        }
                    }
                    if exit.load(Ordering::SeqCst) {
                        break 'main;
                    }
                }
            }
        });
        Ok(join_set)
    }
    ///
    /// Sends "exit" signal to the service's task
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
impl Debug for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Switch")
            .field("txid", &self.txid)
            .field("name", &self.name)
            // .field("send", &self.send)
            // .field("recv", &self.recv)
            // .field("subscribers", &self.subscribers)
            // .field("receivers", &self.receivers)
            .field("timeout", &self.timeout)
            .field("exit", &self.exit)
            .finish()
    }
}