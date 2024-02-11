use crate::errors::parser_error::ParserError;

use super::db_type_parser::parse_db_type;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldSchema {
    pub key: String,
    pub db_type: String,
    pub constraint: Vec<String>,
}

impl FieldSchema {
    pub fn new(key: String, db_type: String, constraint: Vec<String>) -> Self {
        Self {
            key,
            db_type,
            constraint,
        }
    }
}

pub fn parse_field(value: &str) -> Result<FieldSchema, ParserError> {
    if value.is_empty() {
        return Err(ParserError::Column("field can' t be empty".to_string()));
    }
    let mut splitted = value.split(":");
    let key = splitted.next().ok_or_else(|| {
        ParserError::Schema(format!(
            "Not found key in {}. use synthax 'key:db_type'",
            value
        ))
    })?;
    if key.is_empty() {
        return Err(ParserError::Schema(format!(
            "Not found key in {}. use synthax 'key:db_type'",
            key
        )));
    }
    let db_type = splitted.next().ok_or_else(|| {
        ParserError::Schema(format!(
            "Not found db_type in {}. use synthax 'key:db_type'",
            value
        ))
    })?;
    if db_type.is_empty() {
        return Err(ParserError::Schema(format!(
            "Not found db_type in {}. use synthax 'key:db_type'",
            value
        )));
    }
    let db_type = parse_db_type(db_type)?;
    let mut constraint = vec![];
    while let Some(c) = splitted.next() {
        constraint.push(c.to_string());
    }
    Ok(FieldSchema::new(key.to_string(), db_type, constraint))
}

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_field_successfully() {
        let result = parse_field("name:text").unwrap();
        assert_eq!(
            result,
            FieldSchema {
                key: "name".to_string(),
                db_type: "TEXT".to_string(),
                constraint: vec![]
            }
        );
    }

    #[test]
    fn parse_field_fail_for_missing_key() {
        let err = parse_field(":text");
        assert_eq!(err.is_err(), true)
    }

    #[test]
    fn parse_field_fail_for_missing_db_type() {
        let err: Result<FieldSchema, ParserError> = parse_field("key:");
        assert_eq!(err.is_err(), true)
    }

    #[test]
    fn parse_field_fail_for_only_column() {
        let err: Result<FieldSchema, ParserError> = parse_field(":");
        assert_eq!(err.is_err(), true)
    }

    #[test]
    fn parse_field_fail_for_empty_string() {
        let err: Result<FieldSchema, ParserError> = parse_field("");
        assert_eq!(err.is_err(), true)
    }
}
