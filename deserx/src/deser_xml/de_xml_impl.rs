use quick_xml::events::BytesStart;

use crate::{DeXml, DeXmlError};

impl DeXml for String {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, DeXmlError> {
        todo!()
    }

    fn deserialize_xml_from_attribute(start: &BytesStart, attr: &str) -> Result<Self, DeXmlError> {
        let opt_attr_attribute = start.try_get_attribute(attr)?;
        let attribute = match opt_attr_attribute {
            None => {
                return Err(DeXmlError::MissingAttribute {
                    attr: attr.to_string(),
                    event: format!("{:?}", start),
                })
            }
            Some(attr_attribute) => {
                String::from_utf8(attr_attribute.value.to_vec()).map_err(DeXmlError::custom)?
            }
        };
        Ok(attribute)
    }
    // fn deserialize_xml_from_tag<R: std::io::BufRead>(
    //     reader: &mut quick_xml::NsReader<R>,
    //     tag: &str,
    // ) -> Result<Self, DeXmlError> {
    //     use quick_xml::events::Event;
    //     let mut buf = Vec::new();
    //     let mut name: Option<String> = None;
    //     match reader.read_event_into(&mut buf)? {
    //         Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {}
    //         evt => return Err(DeXmlError::UnexpectedToken(format!("got {:?}", evt))),
    //     }
    //     let res = match reader.read_event_into(&mut buf)? {
    //         Event::Text(text) => text.unescape().to_owned()?.to_string(),
    //         evt => return Err(DeXmlError::UnexpectedToken(format!("got {:?}", evt))),
    //     };
    //     match reader.read_event_into(&mut buf)? {
    //         Event::End(evt) if evt.name().as_ref() == tag.as_bytes() => {}
    //         evt => return Err(DeXmlError::UnexpectedToken(format!("got {:?}", evt))),
    //     }
    //     Ok(res)
    // }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_text(reader)
    }
    fn deserialize_xml_from_empty<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        Ok(String::new())
    }
    fn deserialize_xml_from_text<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, DeXmlError> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let res = match reader.read_event_into(&mut buf)? {
            Event::Text(text) => text.unescape()?.to_string(),
            evt => {
                return Err(DeXmlError::UnexpectedEvent {
                    event: format!("de_xml_impl:69: {:?}", evt),
                })
            }
        };
        Ok(res)
    }
    fn deserialize_xml_from_tag<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        tag: &str,
    ) -> Result<Self, DeXmlError> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let mut res = String::default();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) if e.name().as_ref() == tag.as_bytes() => {
                    res = String::default();
                    break;
                }
                Event::Start(e) if e.name().as_ref() == tag.as_bytes() => {
                    continue;
                }
                Event::End(e) if e.name().as_ref() == tag.as_bytes() => {
                    break;
                }
                Event::Text(text) => res.push_str(text.unescape()?.as_ref()),
                evt => {
                    return Err(DeXmlError::UnexpectedEvent {
                        event: format!("de_xml_impl:97: <String> {:?}", evt),
                    })
                }
            }
        }
        Ok(res)
    }
}

impl<T> DeXml for Option<T>
where
    T: DeXml,
{
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, crate::DeXmlError> {
        let _ = reader;
        unimplemented!("not supported for Option")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, crate::DeXmlError> {
        Ok(Some(T::deserialize_xml_from_body(reader, start)?))
    }

    fn deserialize_xml_from_empty<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, crate::DeXmlError> {
        Ok(None)
    }
}
impl<T> DeXml for Vec<T>
where
    T: DeXml,
{
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, DeXmlError> {
        unimplemented!("not supported for vec")
    }

    fn deserialize_xml_from_tag<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        tag: &str,
    ) -> Result<Self, DeXmlError> {
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

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        use quick_xml::events::Event;

        let mut _vec = Vec::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(mut evt) => {
                    let val = T::deserialize_xml_from_body(reader, &mut evt)?;
                    _vec.push(val);
                }
                evt => todo!("handle {:?}", evt),
            }
        }
        Ok(_vec)
    }
}

impl DeXml for usize {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, crate::DeXmlError> {
        unimplemented!("not supported for usize")
    }

    fn deserialize_xml_from_text<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, DeXmlError> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf)? {
            Event::Text(text) => text
                .unescape()?
                .as_ref()
                .parse::<usize>()
                .map_err(crate::DeXmlError::custom),
            evt => Err(DeXmlError::UnexpectedEvent {
                event: format!("{:?}", evt),
            }),
        }
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, DeXmlError> {
        Self::deserialize_xml_from_text(reader)
    }

    fn deserialize_xml_from_attribute(start: &BytesStart, attr: &str) -> Result<Self, DeXmlError> {
        let opt_attr_attribute = start.try_get_attribute(attr)?;
        let attribute = match opt_attr_attribute {
            None => {
                return Err(DeXmlError::MissingAttribute {
                    attr: attr.to_string(),
                    event: format!("{:?}", start),
                })
            }
            Some(attr_attribute) => String::from_utf8_lossy(attr_attribute.value.as_ref())
                .parse::<usize>()
                .map_err(DeXmlError::custom)?,
        };
        Ok(attribute)
    }
}
