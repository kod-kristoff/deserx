use crate::{deser_xml::XmlSerializeStruct, XmlSerializer};

use super::Token;

#[derive(Debug)]
pub struct TokenSerializer<'a> {
    tokens: &'a [Token],
}

impl<'a> TokenSerializer<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some((&first, rest)) = self.tokens.split_first() {
            self.tokens = rest;
            Some(first)
        } else {
            None
        }
    }
    pub fn remaining(&self) -> usize {
        self.tokens.len()
    }
}

impl<'s, 'a> XmlSerializer for &'s mut TokenSerializer<'a> {
    type Ok = ();
    type Error = super::Error;
    type SerializeStruct = Self;

    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct, Self::Error> {
        match self.next_token() {
            Some(Token::Struct { name: n, len }) if n == name => {}
            Some(token) => {
                panic!("Expected Token::Struct, got {:?}", token);
            }
            None => {
                panic!(
                    "Expected end of tokens, got Token::Struct {{ name: {} }}",
                    name
                );
            }
        }
        Ok(self)
    }
}

impl<'s, 'a> XmlSerializeStruct for &'s mut TokenSerializer<'a> {
    type Ok = ();
    type Error = super::Error;

    fn serialize_attribute(
        &mut self,
        key: &'static str,
        value: &'static str,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: crate::SerXml,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.next_token() {
            Some(Token::StructEnd) => {}
            Some(token) => {
                panic!("Expected Token::StructEnd, got {:?}", token);
            }
            None => panic!("Expected end of tokens, but serialized as Token::StructEnd"),
        };
        Ok(())
    }
}
