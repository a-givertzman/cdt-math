use sal_sync::services::entity::dbg_id::DbgId;
use crate::kernel::eval::Eval;
use super::context::{context::Context, ctx_result::CtxResult};
///
/// Just pass context for now
/// to be removed or filled with functionality
#[derive(Debug)]
pub struct Initial {
    dbg: DbgId,
    pub ctx:Option<Context>,
}
//
//
impl Initial {
    /// 
    /// Returns [Initial] new instance
    /// - 'ctx' - Application [Context]
    pub fn new(ctx: Context) -> Self {
        Self {
            dbg: DbgId("Initial".to_string()),
            ctx: Some(ctx),
        }
    }
}
//
//
impl Eval<Context> for Initial {
    //
    //
    fn eval(&mut self) -> CtxResult<Context, crate::kernel::str_err::str_err::StrErr> {
        let ctx = self.ctx.take().unwrap();
        log::debug!("{}.eval | Start with contect: {:#?}", self.dbg, ctx);
        CtxResult::Ok(ctx)
    }
}
