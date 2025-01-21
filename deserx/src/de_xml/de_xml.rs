use super::XmlDeserializer;

pub trait DeXml<'de>: Sized {
    fn deserialize_xml<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: XmlDeserializer<'de>;
}
