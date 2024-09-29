#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    Struct { name: &'static str, len: usize },
    StructEnd,
}
