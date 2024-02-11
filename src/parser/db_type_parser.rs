use crate::errors::parser_error::ParserError;

#[derive(Debug, PartialEq)]
enum DatabaseColumnType {
    Integer,
    Text,
    Varchar,
    Char,
    Float,
    Double,
    Boolean,
    Date,
    Time,
    Timestamp,
    Blob,
    Json,
}

impl DatabaseColumnType {
    pub fn from_str(input: &str) -> Result<DatabaseColumnType, ParserError> {
        match input.to_lowercase().as_str() {
            "integer" => Ok(DatabaseColumnType::Integer),
            "text" => Ok(DatabaseColumnType::Text),
            "varchar" => Ok(DatabaseColumnType::Varchar),
            "char" => Ok(DatabaseColumnType::Char),
            "float" => Ok(DatabaseColumnType::Float),
            "double" => Ok(DatabaseColumnType::Double),
            "boolean" => Ok(DatabaseColumnType::Boolean),
            "date" => Ok(DatabaseColumnType::Date),
            "time" => Ok(DatabaseColumnType::Time),
            "timestamp" => Ok(DatabaseColumnType::Timestamp),
            "blob" => Ok(DatabaseColumnType::Blob),
            "json" => Ok(DatabaseColumnType::Json),
            _ => Err(ParserError::Column(format!(
                "{} is not valid database column type",
                input
            ))),
        }
    }
}

pub fn parse_db_type(value: &str) -> Result<String, ParserError> {
    if value.is_empty() {
        return Err(ParserError::Column("db_type can' t be empty".to_string()));
    }
    let column_type = DatabaseColumnType::from_str(value)?;
    match column_type {
        DatabaseColumnType::Integer => Ok(value.to_uppercase()),
        DatabaseColumnType::Text => Ok(value.to_uppercase()),
        DatabaseColumnType::Varchar => Ok(value.to_uppercase()),
        DatabaseColumnType::Char => Ok(value.to_uppercase()),
        DatabaseColumnType::Float => Ok(value.to_uppercase()),
        DatabaseColumnType::Double => Ok(value.to_uppercase()),
        DatabaseColumnType::Boolean => Ok(value.to_uppercase()),
        DatabaseColumnType::Date => Ok(value.to_uppercase()),
        DatabaseColumnType::Time => Ok(value.to_uppercase()),
        DatabaseColumnType::Timestamp => Ok(value.to_uppercase()),
        DatabaseColumnType::Blob => Ok(value.to_uppercase()),
        DatabaseColumnType::Json => Ok(value.to_uppercase()),
    }
}

#[cfg(test)]
mod db_type_parser_test {
    use super::*;

    #[test]
    fn test_from_str_known_type() {
        assert_eq!(
            DatabaseColumnType::from_str("Integer").unwrap(),
            DatabaseColumnType::Integer
        );
        assert_eq!(
            DatabaseColumnType::from_str("TEXT").unwrap(),
            DatabaseColumnType::Text
        );
        assert_eq!(
            DatabaseColumnType::from_str("VarChar").unwrap(),
            DatabaseColumnType::Varchar
        );
    }

    #[test]
    fn test_from_str_unknown_type() {
        let err = DatabaseColumnType::from_str("foobar");
        assert_eq!(err.is_err(), true)
    }

    #[test]
    fn test_parse_db_type_known_types() {
        assert_eq!(parse_db_type("Integer").unwrap(), "INTEGER");
        assert_eq!(parse_db_type("Text").unwrap(), "TEXT");
        assert_eq!(parse_db_type("VarChar").unwrap(), "VARCHAR");
    }

    #[test]
    fn test_parse_db_type_empty_value() {
        let err = DatabaseColumnType::from_str("");
        assert_eq!(err.is_err(), true)
    }
}
