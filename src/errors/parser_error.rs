#[derive(Debug, PartialEq)]
pub enum ParserError {
    Constraint(String),
    Column(String),
    Schema(String),
}
