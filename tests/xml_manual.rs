use std::io::Cursor;

// use deserx::testing::{assert_ser_tokens, Token};
use deserx::{DeXml, SerXml};
use quick_xml::{events::BytesStart, NsReader, Writer};

#[derive(PartialEq, Debug)]
struct Common {
    name: String,
}

impl DeXml for Common {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Common")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let name = String::deserialize_xml_from_tag(reader, "name")?;

        Ok(Common { name })
    }
}

#[derive(Debug, PartialEq)]
pub struct Root {
    // #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    // #[deserx(xml_text)]
    text: String,
    child: Common,
}
impl DeXml for Root {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Root")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let attribute = String::deserialize_xml_from_attribute(&start, "attribute")?;
        let element = String::deserialize_xml_from_tag(reader, "element")?;
        let text = String::deserialize_xml_from_text(reader)?;
        let child = Common::deserialize_xml_from_tag(reader, "child")?;
        Ok(Self {
            attribute,
            element,
            text,
            child,
        })
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
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Flatten")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
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

pub struct Attribute {
    resource: String,
}
impl SerXml for Attribute {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        use quick_xml::events::Event;
        let mut elem = BytesStart::new("Attribute");
        elem.push_attribute(("resource", self.resource.as_str()));
        serializer.write_event(Event::Empty(elem))
    }
    //         fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //         where
    //             S: XmlSerializer,
    //         {
    //             let mut s = serializer.serialize_struct("Attribute")?;
    //             s.end()
    //         }
}

pub struct FlattenAttribute {
    // flatten
    any_name: Attribute,
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
impl DeXml for Attribute {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Attribute")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        _reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let resource = String::deserialize_xml_from_attribute(start, "resource")?;
        Ok(Self { resource })
    }
}
impl DeXml for FlattenAttribute {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "FlattenAttribute")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let any_name = Attribute::deserialize_xml_from_body(reader, start)?;
        Ok(Self { any_name })
    }
}

#[test]
fn ser_attribute() {
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

    //     assert_ser_tokens(
    //         &val,
    //         &[
    //             Token::Struct {
    //                 name: "Attribute",
    //                 len: 0,
    //             },
    //             Token::StructEnd,
    //         ],
    //     )
}

#[test]
fn de_attribute_empty() {
    let data = r##"<Attribute resource="#A"/>"##;

    let mut reader = NsReader::from_str(data);

    let _contributor = Attribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
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
fn de_flatten_attribute_empty() {
    let data = r##"<FlattenAttribute resource="#A"/>"##;

    let mut reader = NsReader::from_str(data);

    let _contributor = FlattenAttribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
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
fn de_common() {
    let data = Common {
        name: "child content".to_string(),
    };

    //     let mut writer = Writer::new(Cursor::new(Vec::new()));
    //     // to_writer(&mut buffer, &data).unwrap();
    //     data.serialize_xml(&mut writer).unwrap();
    //     let buffer = writer.into_inner().into_inner();
    //     assert_eq!(
    //         String::from_utf8_lossy(&buffer),
    let buffer = "<Common>\
            <name>child content</name>\
        </Common>\
        ";
    //     );

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
fn de_root() {
    let data = Root {
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
        child: Common {
            name: "child content".to_string(),
        },
    };

    //     let mut writer = Writer::new(Cursor::new(Vec::new()));
    //     // to_writer(&mut buffer, &data).unwrap();
    //     data.serialize_xml(&mut writer).unwrap();
    //     let buffer = writer.into_inner().into_inner();
    //     assert_eq!(
    //         String::from_utf8_lossy(&buffer),
    let buffer = "<Root attribute=\"attribute content\">\
            <element>element content</element>\
            text content\
            <child><name>child content</name></child>\
        </Root>\
        ";
    //     );

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
fn de_flatten() {
    let data = Flatten {
        common: Common {
            name: "Name".to_string(),
        },
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
    };

    //     let mut writer = Writer::new(Cursor::new(Vec::new()));
    //     // to_writer(&mut buffer, &data).unwrap();
    //     data.serialize_xml(&mut writer).unwrap();
    //     let buffer = writer.into_inner().into_inner();
    //     assert_eq!(
    //         String::from_utf8_lossy(&buffer),
    let buffer = "<Flatten attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </Flatten>\
        ";
    //     );
    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Flatten::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
    let mut buf = Vec::new();
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        quick_xml::events::Event::Eof
    );
}

//     #[derive(SerXml)]
#[derive(Debug, PartialEq)]
struct FlattenTwice {
    // #[deserx(flatten)]
    field: Flatten,
}
impl DeXml for FlattenTwice {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "FlattenTwice")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let field = Flatten::deserialize_xml_from_body(reader, start)?;
        Ok(Self { field })
    }
}

#[test]
fn de_flatten_twice() {
    let data = FlattenTwice {
        field: Flatten {
            common: Common {
                name: "Name".to_string(),
            },
            attribute: "attribute content".to_string(),
            element: "element content".to_string(),
            text: "text content".to_string(),
        },
    };

    //     let mut writer = Writer::new(Cursor::new(Vec::new()));
    //     // to_writer(&mut buffer, &data).unwrap();
    //     data.serialize_xml(&mut writer).unwrap();
    //     let buffer = writer.into_inner().into_inner();
    //     assert_eq!(
    //         String::from_utf8_lossy(&buffer),
    let buffer = "<FlattenTwice attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </FlattenTwice>\
        ";
    //     );
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
