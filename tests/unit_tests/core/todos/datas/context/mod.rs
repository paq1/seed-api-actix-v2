use chrono::{DateTime, TimeZone, Utc};
use framework_cqrs_lib::cqrs::core::context::Context;

pub struct ContextBuilder {
    ctx: Context
}

impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        Self {
            ctx: Context {
                now: Self::default_now(),
                ..Context::empty()
            }
        }
    }

    pub fn with_instant(&self, date: DateTime<Utc>) -> Self {
        Self {
            ctx: Context {
                now: date,
                ..self.ctx.clone()
            }
        }
    }

    pub fn build(&self) -> Context {
        self.ctx.clone()
    }

    pub fn default_now() -> DateTime<Utc> {
        Utc.timestamp_nanos(1431648000000000)
    }
}