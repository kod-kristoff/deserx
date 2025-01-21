use deserx_quick_xml::from_str;

use crate::manuals::Common;

#[test]
fn deser_common() -> anyhow::Result<()> {
    let expected_str = "<Common><name>child content</name></Common>";
    let expected = Common {
        name: "child content".to_string(),
    };

    let actual: Common = from_str(expected_str)?;
    similar_asserts::assert_eq!(actual, expected);
    // let mut writer = Writer::new(Cursor::new(Vec::new()));
    // // to_writer(&mut buffer, &data).unwrap();
    // data.serialize_xml(&mut writer).unwrap();
    // let buffer = writer.into_inner().into_inner();
    // assert_eq!(
    //     String::from_utf8_lossy(&buffer),
    //     "<Common>\
    //         <name>child content</name>\
    //     </Common>\
    //     "
    // );

    // let mut reader = NsReader::from_reader(Cursor::new(buffer));

    // let data_copy = Common::deserialize_xml(&mut reader).unwrap();

    // assert_eq!(data, data_copy);
    // let mut buf = Vec::new();
    // assert_eq!(
    //     reader.read_event_into(&mut buf).unwrap(),
    //     quick_xml::events::Event::Eof
    // );
    Ok(())
}
