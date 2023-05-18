use std::io::Cursor;

// use deserx::testing::{assert_ser_tokens, Token};
use deserx::SerXml;
use quick_xml::{
    events::{BytesStart, Event},
    Writer,
};

#[derive(SerXml)]
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

#[derive(SerXml)]
pub struct Root {
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
    child: Common,
}

#[test]
fn ser_attribute() {
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
fn ser_derive() {
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
