use std::sync::Arc;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<i64> for Object {

    fn from(value: i64) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Int64(value)))
        }
    }
}