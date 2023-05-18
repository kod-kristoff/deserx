use std::{borrow::Cow, io::BufRead};

use quick_xml::{
    events::{BytesStart, BytesText, Event},
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
        todo!("please impl ser_as_element")
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
        todo!("please impl ser_elem_attributes")
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
