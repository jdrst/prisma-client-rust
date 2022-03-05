pub mod query;
pub mod operator;

pub use chrono;
pub use datamodel;
pub use prisma_models;
pub use query_core;
pub use request_handlers;
pub use serde_json;

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

#[derive(serde::Deserialize)]
pub struct DeleteResult {
    pub count: isize,
}

#[macro_export]
macro_rules! not {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::not(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! and {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::and(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! or {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::or(vec![$($x),+])
    };
}
