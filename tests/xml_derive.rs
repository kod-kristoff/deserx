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
