use std::{borrow::Cow, io::BufRead};

mod de_xml_impl;
mod ser_xml_impl;

use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    ElementWriter, NsReader, Writer,
};

use crate::DeXmlError;

pub trait SerXml {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error>; // Result<S::Ok, S::Error>
                                       // where
                                       //     S: XmlSerializer;
    fn ser_as_element<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        let elem = self.element_start(tag);
        serializer.write_event(Event::Start(elem.clone()))?;
        self.ser_elem_body(serializer)?;
        serializer.write_event(Event::End(elem.to_end()))?;

        Ok(())
    }

    fn element_start<'a>(&self, tag: &'a str) -> BytesStart<'a> {
        let mut elem = BytesStart::new(tag);
        self.ser_elem_attributes(&mut elem);
        elem
    }

    fn attributes(&self) -> &[(&str, &str)] {
        &[]
    }

    fn ser_as_element_empty<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Empty(self.element_start(tag)))
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        todo!("please impl ser_elem_body")
    }

    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        for attr in self.attributes() {
            element.push_attribute(*attr);
        }
    }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        todo!("impl ser_as_text")
    }

    fn as_cow_str<'a>(&'a self) -> Cow<'a, str> {
        todo!("impl as_cow_str")
    }
}

pub trait DeXml: Sized {
    fn deserialize_xml<R: BufRead>(reader: &mut NsReader<R>) -> Result<Self, crate::DeXmlError>;
    fn deserialize_xml_from_text<R: BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, crate::DeXmlError> {
        unimplemented!("impl if applicable")
    }
    fn deserialize_xml_from_attribute(
        start: &BytesStart,
        attr: &str,
    ) -> Result<Self, crate::DeXmlError> {
        unimplemented!("impl if applicable")
    }
    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, crate::DeXmlError> {
        Err(crate::DeXmlError::custom(
            "impl deserialize_xml_from_body if applicable",
        ))
    }
    fn deserialize_xml_from_empty<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, crate::DeXmlError> {
        Self::deserialize_xml_from_body(reader, start)
    }
    fn deserialize_xml_from_body_with_end<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
        expected_end: BytesEnd,
    ) -> Result<Self, crate::DeXmlError> {
        unimplemented!("impl deserialize_xml_from_body_with_end if applicable")
    }
    fn deserialize_xml_from_tag<R: BufRead>(
        reader: &mut NsReader<R>,
        tag: &str,
    ) -> Result<Self, crate::DeXmlError> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let is_empty_elem: bool;
        let self_: Self = match reader.read_event_into(&mut buf)? {
            Event::Empty(evt) if evt.name().as_ref() == tag.as_bytes() => {
                is_empty_elem = true;
                Self::deserialize_xml_from_empty(reader, &evt)?
            }
            Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {
                is_empty_elem = false;
                Self::deserialize_xml_from_body(reader, &evt)?
            }
            evt => {
                return Err(crate::DeXmlError::UnexpectedTag {
                    tag: tag.to_string(),
                    event: format!("{:?}", evt),
                })
            }
        };
        if !is_empty_elem {
            expect_event_end(reader, &mut buf, tag.as_bytes())?;
        }
        Ok(self_)
    }
}

pub fn expect_event_end<R: std::io::BufRead>(
    reader: &mut NsReader<R>,
    buf: &mut Vec<u8>,
    tag: &[u8],
) -> Result<(), crate::DeXmlError> {
    use quick_xml::events::Event;
    match reader.read_event_into(buf)? {
        Event::End(end) if end.name().as_ref() == tag => Ok(()),
        evt => Err(crate::DeXmlError::UnexpectedEvent {
            event: format!(
                "expected End({}), got {:?}",
                String::from_utf8_lossy(tag),
                evt
            ),
        }),
    }
}

pub trait XmlSerializer: Sized {
    type Ok;
    type Error: std::error::Error;
    type SerializeStruct: XmlSerializeStruct<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct, Self::Error>;
    // fn serialize_struct_with_attribute(self, name: &'static str, key: &'static str, value: &'static str) -> Result<Self::SerializeStruct, Self::Error>;
}

pub trait XmlSerializeStruct {
    type Ok;
    type Error: std::error::Error;

    fn serialize_attribute(&mut self, key: &'static str, value: &str) -> Result<(), Self::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: SerXml;

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error>;
}
