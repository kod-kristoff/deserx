use std::io::Cursor;

// use deserx::testing::{assert_ser_tokens, Token};
use deserx::{DeXml, DeXmlError, SerXml};
use quick_xml::{events::BytesStart, NsReader, Writer};

#[derive(PartialEq, Debug)]
struct Common {
    name: String,
    friend: Option<String>,
    age: usize,
}

impl DeXml for Common {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "Common")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let name = String::deserialize_xml_from_tag(reader, "name")?;
        let friend = Option::<String>::deserialize_xml_from_tag(reader, "friend")?;
        let age = usize::deserialize_xml_from_tag(reader, "age")?;

        Ok(Common { name, friend, age })
    }
}

impl SerXml for Common {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Common")
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.name.ser_as_element(serializer, "name")?;
        self.friend.ser_as_element(serializer, "friend")?;
        self.age.ser_as_element(serializer, "age")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Root {
    // #[deserx(xml_attribute)]
    attribute: String,
    // #[deserx(xml_attribute)]
    attr_value: usize,
    element: String,
    // #[deserx(xml_text)]
    text: String,
    child: Common,
}
impl DeXml for Root {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "Root")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let attribute = String::deserialize_xml_from_attribute(&start, "attribute")?;
        let attr_value = usize::deserialize_xml_from_attribute(start, "attr_value")?;
        let element = String::deserialize_xml_from_tag(reader, "element")?;
        let text = String::deserialize_xml_from_text(reader)?;
        let child = Common::deserialize_xml_from_tag(reader, "child")?;
        Ok(Self {
            attribute,
            attr_value,
            element,
            text,
            child,
        })
    }
}
impl SerXml for Root {
    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        element.push_attribute(("attribute", self.attribute.as_str()));
        element.push_attribute(("attr_value", self.attr_value.to_string().as_str()));
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.element.ser_as_element(serializer, "element")?;
        self.text.ser_as_text(serializer)?;
        self.child.ser_as_element(serializer, "child")?;
        Ok(())
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Root")
    }
}

#[derive(Debug, PartialEq)]
pub struct Flatten {
    // #[deserx(flatten)]
    common: Common,
    // #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    // #[deserx(xml_text)]
    text: String,
}

impl DeXml for Flatten {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "Flatten")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let common = Common::deserialize_xml_from_body(reader, &start)?;
        let attribute = String::deserialize_xml_from_attribute(&start, "attribute")?;
        let element = String::deserialize_xml_from_tag(reader, "element")?;
        let text = String::deserialize_xml_from_text(reader)?;
        Ok(Flatten {
            common,
            attribute,
            element,
            text,
        })
    }
}
impl SerXml for Flatten {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Flatten")
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.common.ser_elem_body(serializer)?;
        self.element.ser_as_element(serializer, "element")?;
        self.text.ser_as_text(serializer)
    }
    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        element.push_attribute(("attribute", self.attribute.as_str()));
    }
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    resource: String,
}
impl SerXml for Attribute {
    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        element.push_attribute(("resource", self.resource.as_str()));
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element_empty(serializer, "Attribute")
    }
    //         fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //         where
    //             S: XmlSerializer,
    //         {
    //             let mut s = serializer.serialize_struct("Attribute")?;
    //             s.end()
    //         }
}

#[derive(Debug, PartialEq)]
pub struct FlattenAttribute {
    // flatten
    any_name: Attribute,
}
impl DeXml for Attribute {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "Attribute")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        _reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let resource = String::deserialize_xml_from_attribute(start, "resource")?;
        Ok(Self { resource })
    }
}
impl DeXml for FlattenAttribute {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "FlattenAttribute")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let any_name = Attribute::deserialize_xml_from_body(reader, start)?;
        Ok(Self { any_name })
    }
}
impl SerXml for FlattenAttribute {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element_empty(serializer, "FlattenAttribute")
    }
    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        self.any_name.ser_elem_attributes(element)
    }
}

//     #[derive(SerXml)]
#[derive(Debug, PartialEq)]
struct FlattenTwice {
    // #[deserx(flatten)]
    field: Flatten,
}
impl DeXml for FlattenTwice {
    fn deserialize_xml<R: std::io::BufRead>(reader: &mut NsReader<R>) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "FlattenTwice")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        let field = Flatten::deserialize_xml_from_body(reader, start)?;
        Ok(Self { field })
    }
}
impl SerXml for FlattenTwice {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "FlattenTwice")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.field.ser_elem_body(serializer)
    }
    fn ser_elem_attributes(&self, element: &mut quick_xml::events::BytesStart) {
        self.field.ser_elem_attributes(element)
    }
}

#[test]
fn deser_attribute_empty() {
    let val = Attribute {
        resource: "#A".into(),
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    val.serialize_xml(&mut writer).unwrap();

    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        r##"<Attribute resource="#A"/>"##
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let val_copy = Attribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
    assert_eq!(val, val_copy);
}

#[test]
fn de_attribute_start_end() {
    let data = r##"<Attribute resource="#A"></Attribute>"##;

    let mut reader = NsReader::from_str(data);

    let _contributor = Attribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

#[test]
fn deser_flatten_attribute_empty() {
    let val = FlattenAttribute {
        any_name: Attribute {
            resource: "#A".into(),
        },
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    val.serialize_xml(&mut writer).unwrap();

    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        r##"<FlattenAttribute resource="#A"/>"##
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let val_copy = FlattenAttribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
    assert_eq!(val, val_copy);
}

#[test]
fn de_flatten_attribute_start_end() {
    let data = r##"<FlattenAttribute resource="#A"></FlattenAttribute>"##;

    let mut reader = NsReader::from_str(data);

    let _contributor = FlattenAttribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

#[test]
fn deser_common() {
    let data = Common {
        name: "child content".to_string(),
        friend: None,
        age: 13,
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Common>\
            <name>child content</name>\
            <friend/>\
            <age>13</age>\
        </Common>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Common::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}
#[test]
fn deser_root() {
    let data = Root {
        attribute: "attribute content".to_string(),
        attr_value: 5,
        element: "element content".to_string(),
        text: "text content".to_string(),
        child: Common {
            name: "child content".to_string(),
            friend: Some("sibling".into()),
            age: 6,
        },
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Root attribute=\"attribute content\" attr_value=\"5\">\
            <element>element content</element>\
            text content\
            <child><name>child content</name><friend>sibling</friend><age>6</age></child>\
        </Root>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Root::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

#[test]
fn deser_flatten() {
    let data = Flatten {
        common: Common {
            name: "Name".to_string(),
            friend: None,
            age: 7,
        },
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Flatten attribute=\"attribute content\">\
            <name>Name</name>\
            <friend/>\
            <age>7</age>\
            <element>element content</element>\
            text content\
        </Flatten>\
        "
    );
    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Flatten::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

#[test]
fn deser_flatten_twice() {
    let data = FlattenTwice {
        field: Flatten {
            common: Common {
                name: "Name".to_string(),
                friend: Some("Friend".into()),
                age: 90,
            },
            attribute: "attribute content".to_string(),
            element: "element content".to_string(),
            text: "text content".to_string(),
        },
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<FlattenTwice attribute=\"attribute content\">\
            <name>Name</name>\
            <friend>Friend</friend>\
            <age>90</age>\
            <element>element content</element>\
            text content\
        </FlattenTwice>\
        "
    );
    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = FlattenTwice::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);

    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

// #[test]
// fn ser_derive_vec() {
//     #[derive(SerXml)]
//     struct Base {
//         field: Vec<Common>,
//     }
//     let data = Base {
//         field: vec![
//             Common {
//                 name: "Name 1".to_string(),
//             },
//             Common {
//                 name: "Name 2".to_string(),
//             },
//         ],
//     };

//     let mut writer = Writer::new(Cursor::new(Vec::new()));
//     // to_writer(&mut buffer, &data).unwrap();
//     data.serialize_xml(&mut writer).unwrap();
//     let buffer = writer.into_inner().into_inner();
//     assert_eq!(
//         String::from_utf8_lossy(&buffer),
//         "<Base>\
//             <field>\
//             <Common><name>Name 1</name></Common>\
//             <Common><name>Name 2</name></Common>\
//             </field>\
//         </Base>\
//         "
//     );
// }
