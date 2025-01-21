use std::fmt;

#[derive(Clone, Debug)]
pub enum DeError {
    /// Deserx custom error
    Custom(String),
    /// XML parsing error
    InvalidXml(quick_xml::Error),
}

impl fmt::Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeError::Custom(s) => write!(f, "{}", s),
            DeError::InvalidXml(e) => write!(f, "{}", e),
        }
    }
}

impl ::std::error::Error for DeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DeError::InvalidXml(e) => Some(e),
            _ => None,
        }
    }
}

impl deserx::de_xml::DeXmlError for DeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        DeError::Custom(msg.to_string())
    }
}

impl From<quick_xml::Error> for DeError {
    fn from(e: quick_xml::Error) -> Self {
        DeError::InvalidXml(e)
    }
}
