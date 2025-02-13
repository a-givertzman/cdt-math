use std::{fmt::Debug, str::FromStr, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use log::{info, trace, warn};
use sal_sync::services::{
    entity::{error::str_err::StrErr, name::Name, object::Object, point::point_tx_id::PointTxId}, service::{service::Service, service_handles::ServiceHandles}
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::{infrostructure::client::{change_hoisting_tackle::{ChangeHoistingTackleQuery, ChangeHoistingTackleReply}, choose_hoisting_rope::{ChooseHoistingRopeQuery, ChooseHoistingRopeReply}, choose_user_bearing::{ChooseUserBearingQuery, ChooseUserBearingReply}, choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query_kind::QueryKind}, kernel::link::Link};
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
    /// Match exact kind of query
    /// Returns corresponding reply as [Serialize]
    fn build_reply(dbg:&str, kind: &str, query: impl DeserializeOwned) -> Result<impl Serialize + Debug, StrErr> {
        match QueryKind::from_str(&kind) {
            Ok(kind) => {
                match kind {
                    QueryKind::ChooseUserHook => {
                        let query: ChooseUserHookQuery = query;
                        Ok(ChooseUserHookReply::new())
                    },
                    QueryKind::ChooseUserBearing => {
                        let query: ChooseUserBearingQuery = query;
                        Ok(ChooseUserBearingReply::new())
                    },
                    QueryKind::ChooseHoistingRope => {
                        let query: ChooseHoistingRopeQuery = query;
                        Ok(ChooseHoistingRopeReply::new())
                    },
                    QueryKind::ChangeHoistingTackle => {
                        let query: ChangeHoistingTackleQuery = query;
                        Ok(ChangeHoistingTackleReply::new())
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
        let link = self.link.take().unwrap_or_else(|| panic!("{}.run | Link not found", self.name));
        info!("{}.run | Starting...", self.id().clone());
        trace!("{}.run | Self tx_id: {}", self.id().clone(), PointTxId::from_str(self.id().clone()));
        let self_id = self.id().clone();
        let exit = self.exit.clone();
        let dbg = self.name.join().clone();
        let handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move ||{
            'main: loop {
                match link.recv_query::<QueryKind>() {
                    Ok((kind, query)) => {
                        match Self::build_reply(&dbg.clone(), &kind, query) {
                            Ok(reply) => {
                                if let Err(err) = link.send_reply(reply) {
                                    log::debug!("{}.run | Send reply error: {:?}", dbg.clone(), err);
                                };
                            }
                            Err(_) => todo!(),
                        };
                    }
                    Err(err) => {
                        log::warn!("{}.run | Error: {:?}", dbg.clone(), err);
                        break;
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id().clone());
                return Ok(ServiceHandles::new(vec![(self.id().to_string(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id(), err);
                warn!("{}", message);
                return Err(message)
            }
        }
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
