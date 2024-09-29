use crate::{XmlSerializeStruct, XmlSerializer};
use quick_xml::writer::{ElementWriter, Writer};
use quick_xml::events::{BytesStart,Event};
pub use std::io::Write;

impl<'s, W: Write> XmlSerializer for &'s mut Writer<W> {
    type Ok = ();
    type Error = quick_xml::Error;
    type SerializeStruct = QuickXmlElementSerializer<'s, W>;

    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(QuickXmlElementSerializer {
            writer: self,
            start_tag: BytesStart::new(name)
        })
    }

    // fn serialize_struct_with_attribute(
    //     self,
    //     name: &'static str,
    //     key: &'static str,
    //     value: &'static str,
    // ) -> Result<Self::SerializeStruct, Self::Error> {
    //     todo!()
    // }
}

pub struct QuickXmlElementSerializer<'a, W> {
    writer: &'a mut Writer<W>,
    start_tag: BytesStart<'a>,
}

impl<'a, W: Write> XmlSerializeStruct for QuickXmlElementSerializer<'a, W> {
    type Ok = ();
    type Error = quick_xml::Error;

    fn serialize_attribute(
        &mut self,
        key: &'static str,
        value: & str,
    ) -> Result<(), Self::Error> {
        self.start_tag.push_attribute((key, value));
        Ok(())
        // Ok(self.with_attribute((key, value)))
    }

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: crate::SerXml,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_event(Event::Empty(self.start_tag))?;
        Ok(())
    }
}
