use super::{DeXmlError, Visitor};

pub trait XmlDeserializer<'de>: Sized {
    type Error: DeXmlError;

    fn deserialize_element<V>(self, tag: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}
