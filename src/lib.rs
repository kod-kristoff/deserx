mod deser_xml;
pub use crate::deser_xml::{DeXml, SerXml, XmlSerializeStruct, XmlSerializer};

pub use deserx_derive::*;

pub mod quick_xml_impl;
// pub mod testing;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
