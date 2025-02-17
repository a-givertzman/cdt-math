use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use log::{info, trace, warn};
use sal_sync::services::{
    entity::{name::Name, object::Object, point::point_tx_id::PointTxId}, service::{service::Service, service_handles::ServiceHandles}
};
use serde::{de::DeserializeOwned, Serialize};
use crate::{algorithm::entities::{bearing::Bearing, hoisting_rope::hoisting_rope::HoistingRope, hook::Hook}, infrostructure::client::{change_hoisting_tackle::{ChangeHoistingTackleQuery, ChangeHoistingTackleReply}, choose_hoisting_rope::{ChooseHoistingRopeQuery, ChooseHoistingRopeReply}, choose_user_bearing::{ChooseUserBearingQuery, ChooseUserBearingReply}, choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query}, kernel::link::Link};
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
            fn send_reply(dbg: &str, link: &Link, reply: impl Serialize + Debug) {
                if let Err(err) = link.send_reply(reply) {
                    log::debug!("{}.run | Send reply error: {:?}", dbg, err);
                };
            }
            'main: loop {
                match link.recv_query::<Query>() {
                    Ok(query) => match query {
                        //
                        // all possible kinds jof queries to be matched...
                        // corresponding reply to have to be returned
                        //
                        Query::ChooseUserHook(query) => {
                            let query: ChooseUserHookQuery = query;
                            send_reply(&dbg, &link, ChooseUserHookReply::new(Hook {
                                gost: todo!(),
                                r#type: todo!(),
                                load_capacity_m13: todo!(),
                                load_capacity_m46: todo!(),
                                load_capacity_m78: todo!(),
                                shank_diameter: todo!(),
                            }));
                        },
                        Query::ChooseUserBearing(query) => {
                            let query: ChooseUserBearingQuery = query;
                            send_reply(&dbg, &link, ChooseUserBearingReply::new(Bearing {
                                name: todo!(),
                                outer_diameter: todo!(),
                                inner_diameter: todo!(),
                                static_load_capacity: todo!(),
                                height: todo!(),
                            }))
                        },
                        Query::ChooseHoistingRope(query) => {
                            let query: ChooseHoistingRopeQuery = query;
                            send_reply(&dbg, &link, ChooseHoistingRopeReply::new(HoistingRope {
                                name: todo!(),
                                rope_diameter: todo!(),
                                r#type: todo!(),
                                rope_durability: todo!(),
                                rope_force: todo!(),
                                s: todo!(),
                                m: todo!(),
                            }))
                        },
                        Query::ChangeHoistingTackle(query) => {
                            let query: ChangeHoistingTackleQuery = query;
                            send_reply(&dbg, &link, ChangeHoistingTackleReply::new(
                                0,
                            ))
                        },
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
