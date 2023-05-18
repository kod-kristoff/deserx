// use std::error::Error;
// use std::io::Cursor;

// use deserx::{SerXml, XmlSerializeStruct, XmlSerializer};
// use quick_xml::writer::Writer;

// // type Result<T> = std::result::Result<T, Box<dyn Error>>;

// pub struct Contributor {
//     resource: String,
// }

// impl SerXml for Contributor {
//     fn serialize_xml<'s, S>(&'s self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: XmlSerializer,
//     {
//         let mut s = serializer.serialize_struct("Contributor")?;
//         s.serialize_attribute("resource", &self.resource)?;
//         s.end()
//     }
// }

// pub struct Date {
//     original: String,
// }

// impl SerXml for Date {
//     fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: XmlSerializer,
//     {
//         let mut s = serializer.serialize_struct("Date")?;
//         // s.serialize_field("original",&self.original )?;
//         s.end()
//     }
// }

// #[test]
// fn ser_attribute() -> Result<(), Box<dyn Error>> {
//     let val = Contributor {
//         resource: "#A".into(),
//     };
//     let mut writer = Writer::new(Cursor::new(Vec::new()));
//     val.serialize_xml(&mut writer)?;
//     // writer
//     //     .create_element("Contributor")
//     //     .with_attribute(("resource", "#A"))
//     //     .write_empty()?;

//     let result = writer.into_inner().into_inner();
//     let expected = r##"<Contributor resource="#A"/>"##;
//     assert_eq!(result, expected.as_bytes());
//     Ok(())
// }

// #[test]
// fn ser_field() -> Result<(), Box<dyn Error>> {
//     let val = Date {
//         original: "17 Feb 1960".into(),
//     };
//     let mut writer = Writer::new(Cursor::new(Vec::new()));

//     val.serialize_xml(&mut writer)?;

//     let result = writer.into_inner().into_inner();
//     let expected = r##"<Date><original>17 Feb 1960</original></Date>"##;
//     assert_eq!(result, expected.as_bytes());
//     Ok(())
// }
