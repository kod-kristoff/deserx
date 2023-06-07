use quick_xml::events::BytesStart;

use crate::DeXml;

impl DeXml for String {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        todo!()
    }

    fn deserialize_xml_from_attribute(
        start: &BytesStart,
        attr: &str,
    ) -> Result<Self, quick_xml::Error> {
        let opt_attr_attribute = start.try_get_attribute(attr)?;
        let attribute = match opt_attr_attribute {
            None => {
                return Err(quick_xml::Error::UnexpectedEof(format!(
                    "missing '{}' in {:?}",
                    attr, start
                )))
            }
            Some(attr_attribute) => {
                String::from_utf8(attr_attribute.value.to_vec()).expect("string")
            }
        };
        Ok(attribute)
    }
    // fn deserialize_xml_from_tag<R: std::io::BufRead>(
    //     reader: &mut quick_xml::NsReader<R>,
    //     tag: &str,
    // ) -> Result<Self, quick_xml::Error> {
    //     use quick_xml::events::Event;
    //     let mut buf = Vec::new();
    //     let mut name: Option<String> = None;
    //     match reader.read_event_into(&mut buf)? {
    //         Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {}
    //         evt => return Err(quick_xml::Error::UnexpectedToken(format!("got {:?}", evt))),
    //     }
    //     let res = match reader.read_event_into(&mut buf)? {
    //         Event::Text(text) => text.unescape().to_owned()?.to_string(),
    //         evt => return Err(quick_xml::Error::UnexpectedToken(format!("got {:?}", evt))),
    //     };
    //     match reader.read_event_into(&mut buf)? {
    //         Event::End(evt) if evt.name().as_ref() == tag.as_bytes() => {}
    //         evt => return Err(quick_xml::Error::UnexpectedToken(format!("got {:?}", evt))),
    //     }
    //     Ok(res)
    // }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_text(reader)
    }
    fn deserialize_xml_from_text<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let res = match reader.read_event_into(&mut buf)? {
            Event::Text(text) => text.unescape().to_owned()?.to_string(),
            evt => return Err(quick_xml::Error::UnexpectedToken(format!("got {:?}", evt))),
        };
        Ok(res)
    }
}

impl<T> DeXml for Vec<T>
where
    T: DeXml,
{
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        unimplemented!("not supported for vec")
    }

    fn deserialize_xml_from_tag<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        tag: &str,
    ) -> Result<Self, quick_xml::Error> {
        use quick_xml::events::Event;
        let mut _vec = Vec::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {
                    println!("start of {:?}", evt.name())
                }
                Event::Start(mut evt) => {
                    let _t = T::deserialize_xml_from_body(reader, &mut evt)?;
                    _vec.push(_t);
                }
                Event::End(evt) if evt.name().as_ref() == tag.as_bytes() => {
                    break;
                }
                Event::End(evt) => {}
                evt => todo!("handle {:?}", evt),
            }
        }
        Ok(_vec)
    }

    // fn deserialize_xml_from_body<R: std::io::BufRead>(
    //     reader: &mut quick_xml::NsReader<R>,
    //     start: &BytesStart,
    // ) -> Result<Self, quick_xml::Error> {
    //     use quick_xml::events::Event;

    //     let mut _vec = Vec::new();
    //     let mut buf = Vec::new();
    //     match reader.read_event_into(&mut buf)? {
    //         Event::Start(mut evt) => {
    //             let val = T::deserialize_xml_from_body(reader, &mut evt)?;
    //             _vec.push(val);
    //         }
    //         _ => todo!(),
    //     }
    //     Ok(_vec)
    // }
}
