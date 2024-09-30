use std::borrow::Cow;

use quick_xml::{
    events::{BytesText, Event},
    Writer,
};

use super::SerXml;

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
        serializer
            .create_element(tag)
            .write_text_content(BytesText::new(&self))?;
        Ok(())
    }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Text(BytesText::new(&self)))
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_text(serializer)
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
        let elem_writer = serializer.create_element(tag);
        if self.is_empty() {
            elem_writer.write_empty()?;
        } else {
            elem_writer.write_inner_content(|serializer| self.ser_elem_body(serializer))?;
            // self.ser_elem_body(serializer)?;
        }
        Ok(())
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

impl<T> SerXml for Option<T>
where
    T: SerXml,
{
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        unimplemented!("not supported for Option")
    }

    fn ser_as_element<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
        tag: &str,
    ) -> Result<(), quick_xml::Error> {
        let elem = serializer.create_element(tag);
        match self {
            None => elem.write_empty(),
            Some(body) => elem.write_inner_content(|serializer| body.ser_elem_body(serializer)),
        }?;
        Ok(())
    }

    // fn ser_as_text<W: std::io::Write>(
    //     &self,
    //     serializer: &mut Writer<W>,
    // ) -> Result<(), quick_xml::Error> {
    //     serializer.write_event(Event::Text(BytesText::new(&self)))
    // }
}

impl SerXml for usize {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        unimplemented!("not supprorted for usize")
    }

    // fn ser_as_element<W: std::io::Write>(
    //     &self,
    //     serializer: &mut Writer<W>,
    //     tag: &str,
    // ) -> Result<(), quick_xml::Error> {
    //     serializer
    //         .create_element(tag)
    //         .write_text_content(BytesText::new(&format!("{}", self)))?;
    //     Ok(())
    // }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Text(BytesText::new(&format!("{}", self))))
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_text(serializer)
    }
}
