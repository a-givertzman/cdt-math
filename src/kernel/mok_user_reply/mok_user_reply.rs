use std::{sync::{atomic::AtomicBool, Arc, Mutex}, thread};
use sal_sync::services::service::service::Service;
use serde::{de::DeserializeOwned, Serialize};

use crate::kernel::{link::Link, str_err::str_err::StrErr};
use super::query_struct::QueryStruct;
///
/// Struct to imitate user's answer's
pub struct MokUserReply {
    /// recieve and sender channel's
    link: Link,
    /// value to stop thread that await request's
    exit: Arc<AtomicBool<bool>>,
}
//
//
impl MokUserReply {
    ///
    /// Struct constructor
    pub fn new(link: Link) -> Self {
        Self { 
            link,
            exit: Arc::new(AtomicBool::new(false)), 
        }
    }
    ///
    /// Running user service
    pub fn run(self) -> Result<thread::JoinHandle<()>, String> {
        let link = self.link;
        let handle = thread::spawn(move || {
            loop {
                if *self.exit.lock().unwrap() {
                    break;
                }
                let query: Result<QueryStruct, StrErr> = link.req(QueryStruct::new());
                match query {
                    Ok(request) => {
                        let response = QueryStruct {
                            data: format!("Processed: {}", request.data),
                        };
                        let answer: Result<QueryStruct, StrErr> = link.req(response);
                    }
                    Err(err) => {
                        println!("Error receiving request: {:?}", err);
                        break;
                    }
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
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        debug!("{}.run | Exit: {}", self.id, self.exit.load(Ordering::SeqCst));
    }
}

impl Service for MokUserReply {
    
}