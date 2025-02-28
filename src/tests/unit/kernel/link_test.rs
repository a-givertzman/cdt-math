#[cfg(test)]

mod link {
    use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, Arc, Once}, time::Duration};
    use sal_sync::services::entity::name::Name;
    use serde::{Deserialize, Serialize};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::context::ctx_result::CtxResult, kernel::{str_err::str_err::StrErr, sync::link::Link}};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing 'Request::fetch'
    #[tokio::test]
    async fn req() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.spawn(async {
            init_once();
            init_each();
            log::debug!("");
            let dbg = "fetch";
            log::debug!("\n{}", dbg);
            let test_duration = TestDuration::new(dbg, Duration::from_secs(5));
            test_duration.run().unwrap();
            let test_data: [(i32, Query, Result<Query, StrErr>); 2] = [
                (1, Query("Query-1".into()), Ok(Query("Reply-1".into()))),
                (2, Query("Query-2".into()), Ok(Query("Reply-2".into()))),
            ];
            let (local, remote) = Link::split(dbg);
            let mut listener = Listener::new(dbg, remote);
            log::debug!("{} | Starting Listener", dbg);
            let listener_handle = listener.run().await.unwrap();
            log::debug!("{} | Starting Listener - Ok", dbg);
            for (step, query, target) in test_data {
                let result = local.req(query).await;
                log::debug!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            }
            listener.exit();
            listener_handle.await.unwrap();
            test_duration.exit();
        }).await.unwrap();
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Query(pub String);
    struct Listener {
        name: Name,
        /// recieve and sender channel's
        link: Option<Link>,
        /// value to stop thread that await request's
        exit: Arc<AtomicBool>,
    }
    impl Listener {
        ///
        /// Struct constructor
        pub fn new(parent: impl Into<String>, link: Link) -> Self {
            let name = Name::new(parent, "Listener");
            Self { 
                name: name,
                link: Some(link),
                exit: Arc::new(AtomicBool::new(false)),
            }
        }
        ///
        /// Starts service's main loop in the individual task
        pub async fn run(&mut self) -> Result<tokio::task::JoinHandle<()>, String> {
            let mut link = self.link.take().unwrap_or_else(|| panic!("{}.run | Link not found", self.name));
            let dbg = self.name.join();
            log::info!("{}.run | Starting...", dbg);
            let exit = self.exit.clone();
            let listener_handle = tokio::task::block_in_place( move|| {
                tokio::runtime::Handle::current().spawn(async move {
                    log::info!("{}.run | Start", dbg);
                    async fn send_reply(dbg: &str, link: &mut Link, reply: impl Serialize + Debug) {
                        if let Err(err) = link.send_reply(reply) {
                            log::debug!("{}.run | Send reply error: {:?}", dbg, err);
                        };
                    }
                    'main: loop {
                        match link.recv_query::<String>().await {
                            CtxResult::Ok(query) => match query.as_str() {
                                "Query-1" => send_reply(&dbg, &mut link, "Reply-1").await,
                                "Query-2" => send_reply(&dbg, &mut link, "Reply-2").await,
                                "Query-3" => send_reply(&dbg, &mut link, "Reply-3").await,
                                _ => panic!("Unknown Query: {:?}", query)
                            }
                            CtxResult::Err(err) => {
                                log::warn!("{}.run | Error: {:?}", dbg, err);
                                break;
                            }
                            CtxResult::None => {},
                        }
                        if exit.load(Ordering::SeqCst) {
                            break 'main;
                        }
                    }
                    log::debug!("{}.run | Exit", dbg);
                })
            });
            Ok(listener_handle)
        }
        ///
        /// Sends "exit" signal to the service's thread
        pub fn exit(&self) {
            self.exit.store(true, Ordering::SeqCst);
        }
    }
}
