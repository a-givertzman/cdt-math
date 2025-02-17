
use crate::kernel::str_err::str_err::StrErr;

use crate::kernel::eval::Eval;

use super::context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult};

pub trait EvalZg {
    fn eval(&mut self, input: f64) -> CtxResult<Context, StrErr>;
}
pub struct DoFirst {
    pub value: Option<f64>,
    pub eval: Box<dyn Eval>,
    pub ctx: Option<Context>,
}

impl DoFirst {
    pub fn new (ctx: impl Eval + 'static) -> Self {
        Self { value: None, eval: Box::new(ctx), ctx: None }
    }
}

impl EvalZg for DoFirst {
    fn eval(&mut self, input: f64) -> CtxResult<Context, StrErr> {
        let zg = if self.ctx.is_none() {
            self.ctx = Some(self.eval.eval().unwrap());
            self.ctx.clone().unwrap().zg
        } else {
            input
        };
        self.ctx.clone().unwrap().write(
            DoFirstCtx {
                zg: zg ,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct DoFirstCtx {
    pub zg: f64
}