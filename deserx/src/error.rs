use std::fmt;

#[derive(Debug)]
pub enum DeXmlError {
    MissingAttribute { attr: String, event: String },
    XmlError(quick_xml::Error),
    UnexpectedEvent { event: String },
    UnexpectedTag { tag: String, event: String },
    Custom(String),
}

impl DeXmlError {
    pub fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
impl From<quick_xml::Error> for DeXmlError {
    fn from(value: quick_xml::Error) -> Self {
        Self::XmlError(value)
    }
}

impl fmt::Display for DeXmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "Custom error: {}", s),
            Self::MissingAttribute { attr, event } => {
                f.write_fmt(format_args!("Attribute '{attr}' is missing in {event}"))
            }
            Self::XmlError(_err) => f.write_fmt(format_args!("XML error")),
            Self::UnexpectedEvent { event } => {
                f.write_fmt(format_args!("Unexpected event: {event}"))
            }
            Self::UnexpectedTag { tag, event } => {
                f.write_fmt(format_args!("Expected tag '{tag}', got: {event}"))
            }
        }
    }
}

impl std::error::Error for DeXmlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::XmlError(error) => Some(error),
            _ => None,
        }
    }
}
