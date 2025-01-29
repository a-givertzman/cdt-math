use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc::RecvTimeoutError, Arc, RwLock}, time::Duration};

use sal_sync::services::{entity::{dbg_id::DbgId, name::Name, object::Object, point::point_tx_id::PointTxId}, service::{service::Service, service_handles::ServiceHandles}};

use super::math_service_conf::MathServiceConf;

///
/// 
struct MathService {
    id: String,
    dbg: DbgId,
    name: Name,
    txid: usize,
    conf: MathServiceConf,
    services: Arc<RwLock<Services>>,
    exit: Arc<AtomicBool>,
}
//
//
impl MathService {
    pub fn new(conf: MathServiceConf, services: Arc<RwLock<Services>>) -> Self {
        Self {
             id: conf.name.join(),
             dbg: DbgId(conf.name.join()),
             name: conf.name.clone(),
             txid: PointTxId::from_str(&conf.name.join()),
             services,
             conf,
             exit: Arc::new(AtomicBool::default())
        }
    }
}
//
//
impl Object for MathService {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
//
impl std::fmt::Debug for MathService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MathService")
            .field("id", &self.id)
            .finish()
    }
}
//
//
impl Service for MathService {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles<()>, String> {
        log::info!("{}.run | Starting...", self.id);
        log::trace!("{}.run | Self tx_id: {}", self.txid);
        let self_id = self.id.clone();
        let self_name = self.name.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        let rx_recv = self.subscribe(&subscriptions, &services);
        let handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            'main: loop {
                log::trace!("{}.run | calculation step...", self_id);
                match rx_recv.recv_timeout(recv_timeout) {
                    Ok(point) => {
                        log::debug!("{}.run | point: {:?}", self_id, &point);
                        task_nodes.eval(point);
                        log::debug!("{}.run | calculation step - done ({:?})", self_id, cycle.elapsed());
                        cycle.wait();
                    }
                    Err(err) => {
                        match err {
                            RecvTimeoutError::Timeout => log::trace!("{}.run | Receive error: {:?}", self_id, err),
                            RecvTimeoutError::Disconnected => {
                                log::error!("{}.run | Error receiving from queue: {:?}", self_id, err);
                                break 'main;
                            }
                        }
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            };
            if let Some((service_name, points)) = subscriptions {
                if let Err(err) = services.wlock(&self_id).unsubscribe(&service_name, &self_name.join(), &points) {
                    log::error!("{}.run | Unsubscribe error: {:#?}", self_id, err);
                }
            }
            log::info!("{}.run | Exit", self_id);
        });
        match handle {
            Ok(handle) => {
                log::info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id, err);
                log::warn!("{}", message);
                Err(message)
            }
        }
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        log::debug!("{}.exit | Exit: {}", self.id, self.exit.load(Ordering::SeqCst));
    }
}