use std::{str::FromStr, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use sal_sync::services::{
    entity::{error::str_err::StrErr, name::Name, object::Object}, service::{service::Service, service_handles::ServiceHandles}
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::{infrostructure::client::{query_kind::QueryKind, test_user_query1::{TestUserQuery1, TestUserReply1}, test_user_query2::{TestUserQuery2, TestUserReply2}}, kernel::link::Link};
///
/// Struct to imitate user's answer's
pub struct MokUserReply {
    dbg: String,
    name: Name,
    /// recieve and sender channel's
    link: Option<Link>,
    /// value to stop thread that await request's
    exit: Arc<AtomicBool>,
}
//
//
impl MokUserReply {
    ///
    /// Struct constructor
    pub fn new(parent: impl Into<String>, link: Link) -> Self {
        let name = Name::new(parent, "MokUserReply");
        Self { 
            dbg: name.join(),
            name: name,
            link: Some(link),
            exit: Arc::new(AtomicBool::new(false)), 
        }
    }
    ///
    /// Running user service
    pub fn run(&mut self) -> Result<thread::JoinHandle<()>, String> {
        let dbg = self.name.join();
        let link = self.link.take().unwrap_or_else(|| panic!("{}.run | Link not found", self.name));
        let exit = self.exit.clone();
        let handle = thread::spawn(move || {
            loop {
                match link.recv_query() {
                    Ok((kind, query)) => {
                        match QueryKind::from_str(&kind) {
                            Ok(kind) => {
                                match Self::build_reply(kind, query) {
                                    Ok(reply) => {
                                        if let Err(err) = link.send_reply(reply) {
                                            log::debug!("{}.run | Send reply error: {:?}", dbg, err);
                                        };
                                    }
                                    Err(_) => todo!(),
                                };
                            }
                            Err(err) => log::warn!("{}.run | Error: {:?}", dbg, err),
                        }
                    }
                    Err(err) => {
                        log::warn!("{}.run | Error: {:?}", dbg, err);
                        break;
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        Ok(handle)
    }
    ///
    /// Match exact kind of query
    /// Returns corresponding reply as [Serialize]
    fn build_reply(dbg:&str, kind: &str, query: impl DeserializeOwned) -> Result<impl Serialize, StrErr> {
        match QueryKind::from_str(&kind) {
            Ok(kind) => {
                match kind {
                    QueryKind::TestUserQuery1 => {
                        let query: TestUserQuery1 = query;
                        Ok(TestUserReply1::new())
                    },
                    QueryKind::TestUserQuery2 => {
                        let query: TestUserQuery2 = query;
                        Ok(TestUserReply2::new())
                    },
                    //
                    // all possible kinds jof queries to be matched...
                    // corresponding reply to have to be returned
                    //
                }
            }
            Err(err) => Err(StrErr(format!("{}.build_reply | Error: {:?}", dbg, err))),
        }
    }
    ///
    /// Processesing request
    fn handle_request<T: Serialize + DeserializeOwned>(request: T) -> T {
        request // just echo-answer
    }
}
//
//
impl Object for MokUserReply {
    fn id(&self) -> &str {
        &self.dbg
    }

    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
//
impl Service for MokUserReply {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles<()>, String> {
        todo!()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        log::debug!("{}.run | Exit: {}", self.name, self.exit.load(Ordering::SeqCst));
    }
}
//
//
impl std::fmt::Debug for MokUserReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MokUserReply")
        .field("name", &self.name)
        // .field("link", &self.link)
        // .field("exit", &self.exit)
        .finish()
    }
}
