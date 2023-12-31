use crate::model;
use crate::object::Object;
use teo_result::Error;

impl<'a> TryFrom<&'a Object> for &'a model::Object {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl TryFrom<&Object> for model::Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl TryFrom<Object> for model::Object {

    type Error = Error;

    fn try_from(value: Object) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}