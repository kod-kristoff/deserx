mod deserializer;

pub use deserializer::Dezerializer;
use deserx::de_xml::DeXml;

use crate::errors::DeError;

pub fn from_str<'de, T>(s: &'de str) -> Result<T, DeError>
where
    T: DeXml<'de>,
{
    let mut de = Dezerializer::from_str(s);
    T::deserialize_xml(&mut de)
}
