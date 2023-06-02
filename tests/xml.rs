use std::io::Cursor;

// use deserx::testing::{assert_ser_tokens, Token};
use deserx::{DeXml, SerXml};
use quick_xml::{
    events::{BytesStart, Event},
    NsReader, Writer,
};

#[derive(SerXml, PartialEq, Debug)]
struct Common {
    name: String,
}

#[derive(SerXml)]
pub struct RootWithFlatten {
    #[deserx(flatten)]
    common: Common,
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
}

#[derive(SerXml, Debug, PartialEq)]
pub struct Root {
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
    child: Common,
}

mod contributor {

    use super::*;
    use quick_xml::NsReader;

    pub struct Contributor {
        resource: String,
    }
    impl SerXml for Contributor {
        fn serialize_xml<W: std::io::Write>(
            &self,
            serializer: &mut Writer<W>,
        ) -> Result<(), quick_xml::Error> {
            let mut elem = BytesStart::new("Contributor");
            elem.push_attribute(("resource", self.resource.as_str()));
            serializer.write_event(Event::Empty(elem))
        }
        //         fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        //         where
        //             S: XmlSerializer,
        //         {
        //             let mut s = serializer.serialize_struct("Contributor")?;
        //             s.end()
        //         }
    }

    impl DeXml for Contributor {
        fn deserialize_xml<R: std::io::BufRead>(
            reader: &mut NsReader<R>,
        ) -> Result<Self, quick_xml::Error> {
            use quick_xml::events::Event;
            let mut buf = Vec::new();
            let mut resource: Option<String> = None;
            match reader.read_event_into(&mut buf)? {
                Event::Empty(evt) if evt.name().as_ref() == b"Contributor" => {
                    let opt_attr_resource = evt.try_get_attribute("resource")?;
                    match opt_attr_resource {
                        None => {
                            return Err(quick_xml::Error::UnexpectedEof(format!(
                                "missing 'resource' in {:?}",
                                evt
                            )))
                        }
                        Some(attr_resource) => {
                            resource = Some(
                                String::from_utf8(attr_resource.value.to_vec()).expect("value"),
                            )
                        }
                    }
                }
                Event::Start(evt) if evt.name().as_ref() == b"Contributor" => {
                    let opt_attr_resource = evt.try_get_attribute("resource")?;
                    match opt_attr_resource {
                        None => {
                            return Err(quick_xml::Error::UnexpectedEof(format!(
                                "missing 'resource' in {:?}",
                                evt
                            )))
                        }
                        Some(attr_resource) => {
                            resource = Some(
                                String::from_utf8(attr_resource.value.to_vec()).expect("value"),
                            )
                        }
                    }
                }
                evt => return Err(quick_xml::Error::UnexpectedToken(format!("got {:?}", evt))),
            }
            Ok(Self {
                resource: resource.take().unwrap(),
            })
        }
    }

    #[test]
    fn ser_attribute() {
        let val = Contributor {
            resource: "#A".into(),
        };

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        val.serialize_xml(&mut writer).unwrap();

        let buffer = writer.into_inner().into_inner();
        assert_eq!(
            String::from_utf8_lossy(&buffer),
            r##"<Contributor resource="#A"/>"##
        );

        //     assert_ser_tokens(
        //         &val,
        //         &[
        //             Token::Struct {
        //                 name: "Contributor",
        //                 len: 0,
        //             },
        //             Token::StructEnd,
        //         ],
        //     )
    }

    #[test]
    fn de_attribute_empty() {
        let data = r##"<Contributor resource="#A"/>"##;

        let mut reader = NsReader::from_str(data);

        let contributor = Contributor::deserialize_xml(&mut reader).unwrap();
    }

    #[test]
    fn de_attribute_start_end() {
        let data = r##"<Contributor resource="#A"></Contributor>"##;

        let mut reader = NsReader::from_str(data);

        let contributor = Contributor::deserialize_xml(&mut reader).unwrap();
    }
}
#[test]
fn deser_derive_common() {
    let data = Common {
        name: "child content".to_string(),
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Common>\
            <name>child content</name>\
        </Common>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    // let data_copy = Common::deserialize_xml(&mut reader).unwrap();

    // assert_eq!(data, data_copy);
}
#[test]
fn deser_derive() {
    let data = Root {
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
        child: Common {
            name: "child content".to_string(),
        },
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Root attribute=\"attribute content\">\
            <element>element content</element>\
            text content\
            <child><name>child content</name></child>\
        </Root>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    // let data_copy = Root::deserialize_xml(&mut reader).unwrap();

    // assert_eq!(data, data_copy);
}

#[test]
fn ser_derive_flatten() {
    let data = RootWithFlatten {
        common: Common {
            name: "Name".to_string(),
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
        "<RootWithFlatten attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </RootWithFlatten>\
        "
    );
}

#[test]
fn ser_derive_flatten_complex() {
    #[derive(SerXml)]
    struct Base {
        #[deserx(flatten)]
        field: RootWithFlatten,
    }
    let data = Base {
        field: RootWithFlatten {
            common: Common {
                name: "Name".to_string(),
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
        "<Base attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </Base>\
        "
    );
}

#[test]
fn ser_derive_vec() {
    #[derive(SerXml)]
    struct Base {
        field: Vec<Common>,
    }
    let data = Base {
        field: vec![
            Common {
                name: "Name 1".to_string(),
            },
            Common {
                name: "Name 2".to_string(),
            },
        ],
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // to_writer(&mut buffer, &data).unwrap();
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    assert_eq!(
        String::from_utf8_lossy(&buffer),
        "<Base>\
            <field>\
            <Common><name>Name 1</name></Common>\
            <Common><name>Name 2</name></Common>\
            </field>\
        </Base>\
        "
    );
}
