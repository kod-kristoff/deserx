use std::{borrow::Cow, io::BufRead};

mod de_xml_impl;

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    NsReader, Writer,
};

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
        let mut elem = BytesStart::new(tag);
        self.ser_elem_attributes(&mut elem);
        serializer.write_event(Event::Start(elem.clone()))?;
        self.ser_elem_body(serializer)?;
        serializer.write_event(Event::End(elem.to_end()))
    }

    fn ser_as_element_empty<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        let mut elem = BytesStart::new(tag);
        self.ser_elem_attributes(&mut elem);
        serializer.write_event(Event::Empty(elem))
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        todo!("please impl ser_elem_body")
    }

    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart)
    // -> Result<(), quick_xml::Error>
    {
        // todo!("please impl ser_elem_attributes")
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
    fn deserialize_xml<R: BufRead>(reader: &mut NsReader<R>) -> Result<Self, quick_xml::Error>;
    fn deserialize_xml_from_text<R: BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        unimplemented!("impl if applicable")
    }
    fn deserialize_xml_from_attribute(
        start: &BytesStart,
        attr: &str,
    ) -> Result<Self, quick_xml::Error> {
        unimplemented!("impl if applicable")
    }
    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        unimplemented!("impl deserialize_xml_from_body if applicable")
    }
    fn deserialize_xml_from_empty<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_body(reader, start)
    }
    fn deserialize_xml_from_body_with_end<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
        expected_end: BytesEnd,
    ) -> Result<Self, quick_xml::Error> {
        unimplemented!("impl deserialize_xml_from_body_with_end if applicable")
    }
    fn deserialize_xml_from_tag<R: BufRead>(
        reader: &mut NsReader<R>,
        tag: &str,
    ) -> Result<Self, quick_xml::Error> {
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
                return Err(quick_xml::Error::UnexpectedToken(format!(
                    "expected tag='{}', got {:?}",
                    tag, evt
                )))
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
) -> Result<(), quick_xml::Error> {
    use quick_xml::events::Event;
    match reader.read_event_into(buf)? {
        Event::End(end) if end.name().as_ref() == tag => Ok(()),
        evt => Err(quick_xml::Error::UnexpectedToken(format!(
            "expected End({}), got {:?}",
            String::from_utf8_lossy(tag),
            evt
        ))),
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

impl SerXml for String {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        unimplemented!("")
    }

    fn ser_as_element<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        let elem = BytesStart::new(tag);
        serializer.write_event(Event::Start(elem.clone()))?;
        serializer.write_event(Event::Text(BytesText::new(&self)))?;
        serializer.write_event(Event::End(elem.to_end()))
    }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Text(BytesText::new(&self)))
    }

    fn as_cow_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self)
    }
}

impl<T> SerXml for Vec<T>
where
    T: SerXml,
{
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        unimplemented!()
    }

    fn ser_as_element<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        let elem = BytesStart::new(tag);
        if self.is_empty() {
            serializer.write_event(Event::Empty(elem))
        } else {
            serializer.write_event(Event::Start(elem.clone()))?;
            self.ser_elem_body(serializer)?;
            serializer.write_event(Event::End(elem.to_end()))
        }
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        for obj in self {
            obj.serialize_xml(serializer)?;
        }
        Ok(())
    }
}
