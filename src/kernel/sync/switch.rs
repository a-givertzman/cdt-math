use std::{fmt::Debug, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc}, time::{Duration, Instant}};
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
    subscribers: usize,
    subscribers_tx: Sender<(String, Sender<Point>)>,
    subscribers_rx: Option<Receiver<(String, Sender<Point>)>>,
    receivers_tx: Sender<(String, Receiver<Point>)>,
    receivers_rx: Option<Receiver<(String, Receiver<Point>)>>,
    timeout: Duration,
    exit: Arc<AtomicBool>,
}
//
//
impl Switch {
    ///
    /// Default timeout to await `recv`` operation, 300 ms
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);
    ///
    /// Returns [Switch] new instance
    /// - `send` - local side of channel.send
    /// - `recv` - local side of channel.recv
    /// - `exit` - exit signal for `recv_query` method
    pub fn new(parent: impl Into<String>, send: Sender<Point>, recv: Receiver<Point>) -> Self {
        let name = Name::new(parent, "Switch");
        let (receivers_tx, receivers_rx) = mpsc::channel();
        let (subscribers_tx, subscribers_rx) = mpsc::channel();
        Self {
            txid: PointTxId::from_str(&name.join()),
            name,
            send, 
            recv: Some(recv),
            subscribers: 0,
            subscribers_tx,
            subscribers_rx: Some(subscribers_rx),
            receivers_tx,
            receivers_rx: Some(receivers_rx),
            timeout: Self::DEFAULT_TIMEOUT,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns Self and `remote: [Link]` new instances
    pub fn split(parent: impl Into<String>) -> (Self, Link) {
        let name = Name::new(parent, "Link");
        let (loc_send, rem_recv) = mpsc::channel();
        let (rem_send, loc_recv) = mpsc::channel();
        let remote = Link::new(name.join(), rem_send, rem_recv);
        let (receivers_tx, receivers_rx) = mpsc::channel();
        let (subscribers_tx, subscribers_rx) = mpsc::channel();
        (
            Self { 
                txid: PointTxId::from_str(&name.join()),
                name: name.clone(),
                send: loc_send, recv: Some(loc_recv),
                subscribers: 0,
                subscribers_tx,
                subscribers_rx: Some(subscribers_rx),
                receivers_tx,
                receivers_rx: Some(receivers_rx),
                timeout: Self::DEFAULT_TIMEOUT,
                exit: Arc::new(AtomicBool::new(false)),
            },
            remote,
        )
    }
    ///
    /// Returns connected `Link`
    pub fn link(&mut self) -> Link {
        let (loc_send, rem_recv) = mpsc::channel();
        let (rem_send, loc_recv) = mpsc::channel();
        let remote = Link::new(&format!("{}:{}", self.name, self.subscribers), rem_send, rem_recv);
        let key = remote.name().join();
        self.subscribers_tx.send((key.clone(), loc_send)).unwrap();
        self.receivers_tx.send((key, loc_recv)).unwrap();
        remote
    }
    ///
    /// Entry point
    pub async fn run(&mut self) -> Result<JoinSet<()>, StrErr> {
        let dbg = self.name.join();
        log::info!("{}.run | Remote | Starting...", dbg);
        let mut subscribers = IndexMapFxHasher::<String, Sender<Point>>::with_hasher(BuildHasherDefault::default());
        let subscribers_rx = self.subscribers_rx.take().unwrap();
        let exit = self.exit.clone();
        let recv = self.recv.take().unwrap();
        let timeout = self.timeout;
        let mut join_set = JoinSet::new();
        join_set.spawn(async move {
            tokio::task::block_in_place(move|| {
                log::debug!("{}.run | Remote | Start", dbg);
                'main: loop {
                    for (key, subscriber) in subscribers_rx.try_iter() {
                        subscribers.insert(key, subscriber);
                    };
                    log::debug!("{}.run | Locals | Subscriber: {}", dbg, subscribers.len());
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
                                        None => {
                                            log::warn!("{}.run | Subscriber not found: {:?}", dbg, key);
                                        },
                                    }
                                }
                                _ => log::warn!("{}.run | Uncnown message received: {:?}", dbg, event),
                            }
                        },
                        Err(err) => match err {
                            std::sync::mpsc::RecvTimeoutError::Timeout => {
                                log::warn!("{}.run | Remote | Listening...", dbg);
                            },
                            std::sync::mpsc::RecvTimeoutError::Disconnected => panic!("{}.run | Receive error, all receivers has been closed", dbg),
                        },
                    }
                    if exit.load(Ordering::SeqCst) {
                        break 'main;
                    }
                }
                log::info!("{}.run | Remote | Exit", dbg);
            })
        });
        let dbg = self.name.join();
        log::info!("{}.run | Remote | Starting - Ok", dbg);
        log::info!("{}.run | Locals | Starting...", dbg);
        let send = self.send.clone();
        let mut receivers = IndexMapFxHasher::<String, Receiver<Point>>::with_hasher(BuildHasherDefault::default());
        let receivers_rx = self.receivers_rx.take().unwrap();
        let timeout = self.timeout;
        let interval = self.timeout;    //Duration::from_millis(1000);
        let exit = self.exit.clone();
        join_set.spawn(async move {
            tokio::task::spawn_blocking(move|| {
                log::debug!("{}.run | Locals | Start", dbg);
                'main: loop {
                    let cycle = Instant::now();
                    for (key, receiver) in receivers_rx.try_iter() {
                        receivers.insert(key, receiver);
                    };
                    log::debug!("{}.run | Locals | Receivers: {}", dbg, receivers.len());
                    for (_key, receiver) in receivers.iter() {
                        match receiver.recv_timeout(timeout) {
                            Ok(event) => {
                                log::debug!("{}.run | Received from locals: {:?}", dbg, event);
                                if let Err(err) = send.send(event) {
                                    log::warn!("{}.run | Send error: {:?}", dbg, err);
                                }
                            }
                            Err(err) => match err {
                                mpsc::RecvTimeoutError::Timeout => {
                                    log::warn!("{}.run | Locals | Listening...", dbg);
                                }
                                mpsc::RecvTimeoutError::Disconnected => {
                                    log::error!("{}.run | Receive error, all senders has been closed", dbg);
                                }
                            }
                        }
                        if exit.load(Ordering::SeqCst) {
                            break 'main;
                        }
                    }
                    if exit.load(Ordering::SeqCst) {
                        break 'main;
                    }
                    if receivers.len() == 0 {
                        let elapsed = cycle.elapsed();
                        if elapsed < interval {
                            std::thread::sleep(interval - elapsed);
                        }
                    }
                }
                log::info!("{}.run | Locals | Exit", dbg);
            });
        });
        let dbg = self.name.join();
        log::info!("{}.run | Locals | Starting - Ok", dbg);
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