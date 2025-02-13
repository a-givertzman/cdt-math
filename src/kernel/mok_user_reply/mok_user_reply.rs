use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, RwLock}, thread};
use sal_sync::services::{
    entity::{name::Name, object::Object}, service::{service::Service, service_handles::ServiceHandles}
};
use serde::{de::DeserializeOwned, Serialize};
use crate::kernel::{link::Link, str_err::str_err::StrErr};
use super::query_struct::QueryStruct;
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
    pub fn run(self) -> Result<thread::JoinHandle<()>, String> {
        let link = self.link.take().unwrap_or_else(|| {});
        let handle = thread::spawn(move || {
            loop {
                match link.recv() {
                    Ok(request) => {
                        let response = QueryStruct {
                            data: format!("Processed: {}", request.data),
                        };
                        let answer: Result<QueryStruct, StrErr> = link.req(response);
                    }
                    Err(err) => {
                        log::warn!("{}.run | Error: {:?}", self.dbg, err);
                        break;
                    }
                }
                if self.exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        Ok(handle)
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
