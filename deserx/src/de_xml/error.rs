use std::{error::Error as StdError, fmt::Display};

pub trait DeXmlError: StdError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}
