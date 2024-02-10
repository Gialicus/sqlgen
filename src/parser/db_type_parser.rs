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
    NotNull,
    Unknown,
}

impl DatabaseColumnType {
    pub fn from_str(input: &str) -> DatabaseColumnType {
        match input.to_lowercase().as_str() {
            "integer" => DatabaseColumnType::Integer,
            "text" => DatabaseColumnType::Text,
            "varchar" => DatabaseColumnType::Varchar,
            "char" => DatabaseColumnType::Char,
            "float" => DatabaseColumnType::Float,
            "double" => DatabaseColumnType::Double,
            "boolean" => DatabaseColumnType::Boolean,
            "date" => DatabaseColumnType::Date,
            "time" => DatabaseColumnType::Time,
            "timestamp" => DatabaseColumnType::Timestamp,
            "blob" => DatabaseColumnType::Blob,
            "json" => DatabaseColumnType::Json,
            "notnull" => DatabaseColumnType::NotNull,
            _ => DatabaseColumnType::Unknown,
        }
    }
}

pub fn parse_db_type(value: &str) -> String {
    if value.is_empty() {
        panic!("value can't be empty")
    }
    let column_type = DatabaseColumnType::from_str(value);
    match column_type {
        DatabaseColumnType::Integer => value.to_uppercase(),
        DatabaseColumnType::Text => value.to_uppercase(),
        DatabaseColumnType::Varchar => value.to_uppercase(),
        DatabaseColumnType::Char => value.to_uppercase(),
        DatabaseColumnType::Float => value.to_uppercase(),
        DatabaseColumnType::Double => value.to_uppercase(),
        DatabaseColumnType::Boolean => value.to_uppercase(),
        DatabaseColumnType::Date => value.to_uppercase(),
        DatabaseColumnType::Time => value.to_uppercase(),
        DatabaseColumnType::Timestamp => value.to_uppercase(),
        DatabaseColumnType::Blob => value.to_uppercase(),
        DatabaseColumnType::Json => value.to_uppercase(),
        DatabaseColumnType::NotNull => String::from("NOT NULL"),
        DatabaseColumnType::Unknown => panic!("Unknown type is not valid"),
    }
}

#[cfg(test)]
mod db_type_parser_test {
    use super::*;

    #[test]
    fn test_from_str_known_type() {
        assert_eq!(
            DatabaseColumnType::from_str("Integer"),
            DatabaseColumnType::Integer
        );
        assert_eq!(
            DatabaseColumnType::from_str("TEXT"),
            DatabaseColumnType::Text
        );
        assert_eq!(
            DatabaseColumnType::from_str("VarChar"),
            DatabaseColumnType::Varchar
        );
    }

    #[test]
    fn test_from_str_unknown_type() {
        assert_eq!(
            DatabaseColumnType::from_str("UnknownType"),
            DatabaseColumnType::Unknown
        );
        assert_eq!(
            DatabaseColumnType::from_str("invalid"),
            DatabaseColumnType::Unknown
        );
    }

    #[test]
    fn test_parse_db_type_known_types() {
        assert_eq!(parse_db_type("Integer"), "INTEGER");
        assert_eq!(parse_db_type("Text"), "TEXT");
        assert_eq!(parse_db_type("VarChar"), "VARCHAR");
    }

    #[test]
    #[should_panic(expected = "Unknown type is not valid")]
    fn test_parse_db_type_unknown_type() {
        assert_eq!(parse_db_type("UnknownType"), "Unknown type is not valid");
        assert_eq!(parse_db_type("invalid"), "Unknown type is not valid");
    }

    #[test]
    #[should_panic(expected = "value can't be empty")]
    fn test_parse_db_type_empty_value() {
        parse_db_type("");
    }
}