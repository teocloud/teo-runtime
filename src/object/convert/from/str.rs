use std::sync::Arc;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<&str> for Object {

    fn from(value: &str) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::String(value.to_string())))
        }
    }
}