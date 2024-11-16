#[cfg(test)]

mod ForceOfGravity {
    use std::{env::var, str::FromStr, sync::Once, time::{Duration, Instant}};
    use env_logger::Target;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, force_of_gravity::force_of_gravity::ForceOfGravity, lifting_speed::lifting_speed::LiftingSpeed, select_bet_phi::select_bet_phi::{BetPhi, SelectBetPhi}}, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, liftclass::LiftClass, load_combination::LoadCombination}}};
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
    /// Testing [Dk].eval
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                ForceOfGravity{
                    dbgid,
                    dynamic_coefficient: DynamicCoefficient{
                        dbgid: DbgId(format!("ForceOfGravity")),
                        select_bet_phi: SelectBetPhi{
                            dbgid: DbgId(format!("DynamicCoefficient")),
                            lift_class: LiftClass::Hc1,
                            value: Some(BetPhi{
                                bet: 0.17,
                                phi: 0.15,
                            }),
                        },
                        lifting_speed: LiftingSpeed{
                            dbgid: DbgId(format!("LiftingSpeed")),
                            driver_type: DriverType::Hd1,
                            load_comb: LoadCombination::A1,
                            vhmax: 20.0,
                            vhcs: 20.0,
                            value: Some(20.0),
                        },
                        value: Some(20.0),
                    },
                    m_to_lift: 20.0,
                    g: 9.81,
                    value: None,
                }, 3924.0
            ),
        ];
        for (mut value,target) in test_data {
            let result = value.eval();
            match result {
                Ok(result) => {
                    assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
                }
                Err(err) =>println!("error"),
            }
        }
        test_duration.exit();
    }
}
