
use crate::kernel::str_err::str_err::StrErr;

use super::{context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult}, do_first::EvalZg};
pub struct DoSecond {
    pub value: Option<f64>,
    pub eval: Box<dyn EvalZg>,
}

impl DoSecond {
    pub fn new (ctx: impl EvalZg + 'static) -> Self {
        Self { value: None, eval: Box::new(ctx) }
    }
}

impl EvalZg for DoSecond {
    fn eval(&mut self, input: f64) -> CtxResult<Context, StrErr> {
        match self.eval.eval(input) {
            CtxResult::Ok(ctx) => {
                let zg = ctx.zg;
                ctx.write(
                    DoSecondCtx {
                        result: zg * 2.0,
                    }
                )
            }
            CtxResult::Err(err) => CtxResult::Err(err),
            CtxResult::None => CtxResult::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoSecondCtx {
    pub result: f64
}