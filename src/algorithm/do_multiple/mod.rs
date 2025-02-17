
use crate::kernel::{eval::Eval, str_err::str_err::StrErr};

use super::{context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult}, do_first::EvalZg};
pub struct DoMultiple {
    pub value: Option<f64>,
    pub eval: Box<dyn EvalZg>,
}

impl DoMultiple {
    pub fn new (ctx: impl EvalZg + 'static) -> Self {
        Self { value: None, eval: Box::new(ctx) }
    }
}

impl DoMultiple {
    fn eval(&mut self) -> CtxResult<Vec<Context>, StrErr> {
        let zg = vec![-0.0, 0.0, 0.5, 0.9];
        let mut result = vec![];
        for input in zg {
            match self.eval.eval(input) {
                CtxResult::Ok(ctx) => {
                    result.push(ctx);
                }
                CtxResult::Err(err) => return CtxResult::Err(err),
                CtxResult::None => return CtxResult::None,
            }
        }
        CtxResult::Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct DoSecondCtx {
    pub result: f64
}