use super::{Token, TokenSerializer};
use crate::SerXml;

pub fn assert_ser_tokens<T: ?Sized>(value: &T, tokens: &[Token])
where
    T: SerXml,
{
    let mut ser = TokenSerializer::new(tokens);
    match value.serialize_xml(&mut ser) {
        Ok(_) => {}
        Err(err) => panic!("value failed to serialize: {}", err),
    }

    if ser.remaining() > 0 {
        panic!("{} remaining tokens", ser.remaining());
    }
}
