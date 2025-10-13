use crate::
    algorithm::entities::usage_class::UsageClass
;
///
/// Calculation context store: [usage class](design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter01_loadUsageClass.md)
#[derive(Debug, Clone, Default)]
pub struct ChoiceUsageClassCtx {
    /// value of [usage class](design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter01_loadUsageClass.md)
    pub result: UsageClass,
}
