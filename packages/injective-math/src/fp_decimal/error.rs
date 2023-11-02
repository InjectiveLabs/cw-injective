#[derive(Debug, PartialEq, Eq)]
pub enum FPDecimalError {
    Undefined(String),
    NotSupported(String),
}
