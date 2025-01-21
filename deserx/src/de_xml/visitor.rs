use std::fmt;

use super::DeXmlError;

pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    fn visit_element<A>(self, elem: A) -> Result<Self::Value, A::Error>
    where
        A: ElemAccess<'de>,
    {
        let _ = elem;
        Err(DeXmlError::custom("`visit_element` not implemented"))
    }
}

pub trait ElemAccess<'de> {
    type Error: DeXmlError;

    // fn next_attribute
}
