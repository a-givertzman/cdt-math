use std::{sync::{Arc, Mutex}, thread};
use serde::{de::DeserializeOwned, Serialize};

use crate::kernel::{link::Link, str_err::str_err::StrErr};
use super::query_struct::QueryStruct;
///
/// Struct to imitate user's answer's
pub struct MokUserReply {
    /// where store info about recieve and sender channel's
    link: Link,
    /// value to stop thread that await request's
    stop_signal: Arc<Mutex<bool>>,
}
//
//
impl MokUserReply {
    ///
    /// Struct constructor
    pub fn new(link: Link) -> Self {
        Self { 
            link,
            stop_signal: Arc::new(Mutex::new(false)), 
        }
    }
    ///
    /// Running user service
    pub fn run(self) -> Result<thread::JoinHandle<()>, String> {
        let link = self.link;
        let handle = thread::spawn(move || {
            loop {
                if *self.stop_signal.lock().unwrap() {
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
    ///
    /// Stoping thread
    pub fn exit(&self) {
        let mut stop = self.stop_signal.lock().unwrap();
        *stop = true;
    }
}
