#[cfg(test)]
mod select_safety_coeff {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::algorithm::{entities::{crane_work_area_type::CraneWorkArea, mechanism_work_type::MechanismWorkType, winding_type::WindingType}, rope_safety_factor::select_safety_coeff::SelectSafetyCoeff};
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
    /// Testing 'eval'
    #[tokio::test(flavor = "multi_thread")]
    async fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "select_safety_coeff";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                WindingType::MultiLayer,
                false,
                CraneWorkArea::Aggressive,
                MechanismWorkType::M1,
                4.5
            ),
            (
                2,
                WindingType::SingleLayer,
                false,
                CraneWorkArea::Default,
                MechanismWorkType::M3,
                3.55
            ),
            (
                3,
                WindingType::MultiLayer,
                true,
                CraneWorkArea::Default,
                MechanismWorkType::M4,
                4.5
            ),
            (
                4,
                WindingType::MultiLayer,
                false,
                CraneWorkArea::StrongAggressive,
                MechanismWorkType::M2,
                4.5
            )
        ];
        for (step,winding_type, mark_fire_exp_env, crane_work_area, mechanism_work_type,target) in test_data {
            match SelectSafetyCoeff::new("Test", winding_type, mark_fire_exp_env, crane_work_area, mechanism_work_type).eval() {
                Ok(result) => {
                    assert!(
                        result.result == target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result.result,
                        target
                    );
                },
                Err(err) => panic!("step {} \nerror: {:#?}", step, err),
            }
        }
        test_duration.exit();
    }
}
