use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use sal_sync::services::{
    entity::{name::Name, object::Object, point::point_tx_id::PointTxId},
    service::{service::Service, service_handles::ServiceHandles},
};
use serde::Serialize;
use crate::{
    algorithm::{context::ctx_result::CtxResult, entities::{bearing::Bearing, hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType}, hook::Hook}}, 
    infrostructure::client::{
        change_hoisting_tackle::{ChangeHoistingTackleQuery, ChangeHoistingTackleReply},
        choose_hoisting_rope::{ChooseHoistingRopeQuery, ChooseHoistingRopeReply},
        choose_user_bearing::{ChooseUserBearingQuery, ChooseUserBearingReply},
        choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply},
        query::Query
    },
    kernel::link::Link,
};
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
        let dbg = self.name.join().clone();
        log::info!("{}.run | Starting...", dbg);
        log::trace!("{}.run | Self tx_id: {}", dbg, PointTxId::from_str(self.id()));
        let exit = self.exit.clone();
        let handle = thread::Builder::new().name(format!("{} - main", dbg)).spawn(move || {
            fn send_reply(dbg: &str, link: &Link, reply: impl Serialize + Debug) {
                if let Err(err) = link.send_reply(reply) {
                    log::debug!("{}.run | Send reply error: {:?}", dbg, err);
                };
            }
            'main: loop {
                match link.recv_query::<Query>() {
                    CtxResult::Ok(query) => match query {
                        Query::ChooseUserHook(query) => {
                            let query: ChooseUserHookQuery = query;
                            let reply = match query.testing {
                                true => ChooseUserHookReply::new(Hook {
                                    gost: "GOST 34567-85".to_string(),
                                    r#type: "Forged".to_string(),
                                    load_capacity_m13: 25.0,
                                    load_capacity_m46: 23.0,
                                    load_capacity_m78: 21.0,
                                    shank_diameter: 85.0,
                                }),
                                false => ChooseUserHookReply::new(Hook {
                                    gost: "GOST 34567-85".to_string(),
                                    r#type: "Forged".to_string(),
                                    load_capacity_m13: 25.0,
                                    load_capacity_m46: 23.0,
                                    load_capacity_m78: 21.0,
                                    shank_diameter: 85.0,
                                }),
                            };
                            send_reply(&dbg, &link, reply.choosen);
                        }
                        Query::ChooseUserBearing(query) => {
                            let _query: ChooseUserBearingQuery = query;
                            send_reply(&dbg, &link, ChooseUserBearingReply::new(Bearing {
                                name: "8100H".to_owned(),
                                outer_diameter: 24.0,
                                inner_diameter: 10.0,
                                static_load_capacity: 11800.0,
                                height: 9.0,
                            }))
                        }
                        Query::ChooseHoistingRope(query) => {
                            let _query: ChooseHoistingRopeQuery = query;
                            send_reply(&dbg, &link, ChooseHoistingRopeReply::new(HoistingRope {
                                name: "STO 71915393-TU 051-2014 Octopus 826K".to_owned(),
                                rope_diameter: 12.0,
                                r#type: RopeType::Metal,
                                rope_durability: RopeDurabilityClass::C1770,
                                rope_force: 112.0,
                                s: 67.824,
                                m: 0.688,
                            }))
                        }
                        Query::ChangeHoistingTackle(query) => {
                            let _query: ChangeHoistingTackleQuery = query;
                            send_reply(&dbg, &link, ChangeHoistingTackleReply::new(1))
                        }
                    }
                    CtxResult::Err(err) => {
                        log::warn!("{}.run | Error: {:?}", dbg.clone(), err);
                        break;
                    }
                    CtxResult::None => {},
                }
                // std::thread::sleep(Duration::from_millis(100));
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
            log::debug!("{}.run | Exit", dbg);
        });
        match handle {
            Ok(handle) => {
                log::info!("{}.run | Starting - ok", self.id());
                Ok(ServiceHandles::new(vec![(self.id().to_string(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id(), err);
                log::warn!("{}", message);
                Err(message)
            }
        }
    }    
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
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
