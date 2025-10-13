use crate::algorithm::entities::hoist::hoist::Hoist;
///
/// Calculation context store: [hoist filter](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter03_chooseHoist.md)
#[derive(Debug, Clone, Default)]
pub struct HoistFilterCtx {
    /// value of [hoist filter](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
    pub result: Vec<Hoist>,
}
