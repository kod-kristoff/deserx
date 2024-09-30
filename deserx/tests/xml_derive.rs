use std::io::Cursor;

// use deserx::testing::{assert_ser_tokens, Token};
use deserx::{DeXml, SerXml};
use quick_xml::{
    events::{BytesStart, Event},
    NsReader, Writer,
};

#[derive(Debug, PartialEq, SerXml, DeXml)]
pub struct Attribute {
    #[deserx(xml_attribute)]
    resource: String,
}
#[derive(SerXml, PartialEq, Debug, DeXml)]
struct Common {
    name: String,
    friend: Option<String>,
}

#[derive(SerXml, Debug, PartialEq, DeXml)]
pub struct Flatten {
    #[deserx(flatten)]
    common: Common,
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
}

#[derive(Debug, PartialEq, SerXml, DeXml)]
pub struct FlattenAttribute {
    #[deserx(flatten)]
    any_name: Attribute,
}

#[derive(SerXml, Debug, PartialEq, DeXml)]
struct FlattenTwice {
    #[deserx(flatten)]
    field: Flatten,
}

#[derive(SerXml, Debug, PartialEq, DeXml)]
pub struct Root {
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
    child: Common,
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
        // TODO: should serialize to this: r##"<FlattenAttribute resource="#A"/>"##
        r##"<FlattenAttribute resource="#A"></FlattenAttribute>"##
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
    // TODO should use this
    // let data = r##"<FlattenAttribute resource="#A"></FlattenAttribute>"##;
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
fn deser_derive_common() {
    let data = Common {
        name: "child content".to_string(),
        friend: None,
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
        </Common>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Common::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
}
#[test]
fn deser_root() {
    let data = Root {
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
        child: Common {
            name: "child content".to_string(),
            friend: Some("sibling".into()),
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
            <child><name>child content</name><friend>sibling</friend></child>\
        </Root>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Root::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
}

#[test]
fn deser_derive_flatten() {
    let data = Flatten {
        common: Common {
            name: "Name".to_string(),
            friend: None,
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
            <element>element content</element>\
            text content\
        </Flatten>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Flatten::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
}

#[test]
fn deser_derive_flatten_twice() {
    let data = FlattenTwice {
        field: Flatten {
            common: Common {
                name: "Name".to_string(),
                friend: Some("Friend".into()),
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
            <element>element content</element>\
            text content\
        </FlattenTwice>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = FlattenTwice::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
}

#[test]
fn deser_derive_vec() {
    #[derive(SerXml, DeXml, Debug, PartialEq)]
    struct Base {
        field: Vec<Common>,
    }
    let data = Base {
        field: vec![
            Common {
                name: "Name 1".to_string(),
                friend: None,
            },
            Common {
                name: "Name 2".to_string(),
                friend: Some("Name 1".into()),
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
            <Common><name>Name 1</name><friend/></Common>\
            <Common><name>Name 2</name><friend>Name 1</friend></Common>\
            </field>\
        </Base>\
        "
    );

    let mut reader = NsReader::from_reader(Cursor::new(buffer));

    let data_copy = Base::deserialize_xml(&mut reader).unwrap();

    assert_eq!(data, data_copy);
}
