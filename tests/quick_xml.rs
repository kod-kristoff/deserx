use std::error::Error;
use std::io::Cursor;

use deserx::{SerXml, XmlSerializeStruct, XmlSerializer};
use quick_xml::writer::Writer;

pub struct Contributor {
    resource: String,
}

impl SerXml for Contributor {
    fn serialize_xml<'s, S>(&'s self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: XmlSerializer,
    {
        let mut s = serializer.serialize_struct("Contributor")?;
        s.serialize_attribute("resource", &self.resource)?;
        s.end()
    }
}

#[test]
fn ser_attribute() -> Result<(), Box<dyn Error>> {
    let val = Contributor {
        resource: "#A".into(),
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    val.serialize_xml(&mut writer)?;
    // writer
    //     .create_element("Contributor")
    //     .with_attribute(("resource", "#A"))
    //     .write_empty()?;

    let result = writer.into_inner().into_inner();
    let expected = r##"<Contributor resource="#A"/>"##;
    assert_eq!(result, expected.as_bytes());
    Ok(())
}
