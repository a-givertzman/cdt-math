use std::{hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use sal_sync::{collections::map::IndexMapFxHasher, services::entity::{cot::Cot, error::str_err::StrErr, name::Name, point::{point::Point, point_tx_id::PointTxId}}};
use tokio::{sync::mpsc::{self, Receiver, Sender}, task::{JoinHandle, JoinSet}};
use tokio_stream::{wrappers::ReceiverStream, StreamExt, StreamMap};
use super::{link::Link, recv_timeout::RecvTimeout};
trait SizedStream: tokio_stream::Stream<Item = Point> + Sized {}
///
/// 
pub struct Switch {
    txid: usize,
    name: Name,
    send: Sender<Point>,
    recv: Option<Receiver<Point>>,
    subscribers: IndexMapFxHasher<String, Sender<Point>>,
    stream: Option<StreamMap<String, ReceiverStream<Point>>>,
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
            stream: None,
            timeout: Self::DEFAULT_TIMEOUT,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns connected `Link`
    pub fn link(&mut self) -> Link {
        let (loc_send, rem_recv) = mpsc::channel(10_000);
        let (rem_send, loc_recv) = mpsc::channel(10_000);
        let remote = Link::new(&self.name, rem_send, rem_recv);
        let key = remote.name().join();
        self.subscribers.insert(key.clone(), loc_send);
        let stream = ReceiverStream::new(loc_recv);
        match &mut self.stream {
            Some(self_stream) => {
                self_stream.insert(key, stream);
            },
            None => {
                self.stream = Some(tokio_stream::StreamMap::new())
            },
        };
        remote
    }
    ///
    /// Entry point
    pub fn run(&mut self) -> Result<JoinSet<()>, StrErr> {
        let dbg = self.name.join();
        let subscribers: IndexMapFxHasher<String, Sender<Point>> = self.subscribers.drain(0..).collect();
        let exit = self.exit.clone();
        let mut recv = self.recv.take().unwrap();
        let timeout = self.timeout.clone();
        let mut join_set = JoinSet::new();
        join_set.spawn(async move {
            'main: loop {
                match recv.recv_timeout(timeout).await {
                    Ok(event) => {
                        match event.cot() {
                            Cot::Inf | Cot::Act | Cot::Req => {
                                for (_key, subscriber) in &subscribers {
                                    if let Err(err) = subscriber.send(event.clone()).await {
                                        log::warn!("{}.run | Send error: {:?}", dbg, err);
                                    }
                                }
                            }
                            Cot::ReqCon | Cot::ReqErr => {
                                let key = event.name();
                                match subscribers.get(&key) {
                                    Some(subscriber) => {
                                        if let Err(err) = subscriber.send(event.clone()).await {
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
        let mut recv_stream = self.stream.take().unwrap();
        let exit = self.exit.clone();
        join_set.spawn(async move {
            'main: loop {
                match recv_stream.next().await {
                    Some((_key, event)) => {
                        if let Err(err) = send.send(event).await {
                            log::warn!("{}.run | Send error: {:?}", dbg, err)
                        }
                    },
                    None => {
                        panic!("{}.run | Receive error, all senders has been closed", dbg)
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
        });
        Ok(join_set)
    }
    ///
    /// Sends "exit" signal to the service's task
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}