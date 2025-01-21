use deserx::de_xml::{DeXmlError, ElemAccess, XmlDeserializer};
use quick_xml::{events::BytesStart, Reader};
use xml_reader::{DeEvent, SliceReader, StartTrimmer, XmlRead, XmlReader};

use crate::errors::DeError;

mod xml_reader;

pub struct Dezerializer<'de, R>
where
    R: XmlRead<'de>,
{
    reader: XmlReader<'de, R>,
    peek: Option<DeEvent<'de>>,
}

impl<'de, R> Dezerializer<'de, R>
where
    R: XmlRead<'de>,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader: XmlReader::new(reader),
            peek: None,
        }
    }
    fn next(&mut self) -> Result<DeEvent<'de>, DeError> {
        if let Some(e) = self.peek.take() {
            return Ok(e);
        }
        self.reader.next()
    }
}
impl<'de> Dezerializer<'de, SliceReader<'de>> {
    pub fn from_str(s: &'de str) -> Self {
        let mut reader = Reader::from_str(s);
        // let config = reader.config_mut();
        // config.expand_empty_elements = true;
        Self::new(SliceReader {
            reader,
            start_trimmer: StartTrimmer::default(),
        })
    }
}

impl<'de, 'a, R> XmlDeserializer<'de> for &'a mut Dezerializer<'de, R>
where
    R: XmlRead<'de>,
{
    type Error = DeError;

    fn deserialize_element<V>(self, tag: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: deserx::de_xml::Visitor<'de>,
    {
        match self.next()? {
            DeEvent::Start(e) => visitor.visit_element(XmlElemAccess::new(self, e)?),
            e => todo!("handle {:?}", e),
        }
    }
}

struct XmlElemAccess<'de, 'd, R>
where
    R: XmlRead<'de>,
{
    start: BytesStart<'de>,
    de: &'d mut Dezerializer<'de, R>,
}

impl<'de, 'd, R> XmlElemAccess<'de, 'd, R>
where
    R: XmlRead<'de>,
{
    pub fn new(de: &'d mut Dezerializer<'de, R>, start: BytesStart<'de>) -> Result<Self, DeError> {
        Ok(Self { start, de })
    }
}
impl<'de, 'd, R> ElemAccess<'de> for XmlElemAccess<'de, 'd, R>
where
    R: XmlRead<'de>,
{
    type Error = DeError;
}
