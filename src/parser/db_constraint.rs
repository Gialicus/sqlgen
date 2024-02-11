use crate::errors::parser_error::ParserError;

#[derive(Debug, PartialEq)]
enum DbConstraintType {
    PrimaryKey,
    Unique,
    ForeignKey,
    Check,
    Default,
    NotNull,
}
impl DbConstraintType {
    pub fn from_str(input: &str) -> Result<DbConstraintType, ParserError> {
        match input.to_lowercase().as_str() {
            "primarykey" => Ok(DbConstraintType::PrimaryKey),
            "unique" => Ok(DbConstraintType::Unique),
            "foreingkey" => Ok(DbConstraintType::ForeignKey),
            "check" => Ok(DbConstraintType::Check),
            "default" => Ok(DbConstraintType::Default),
            "notnull" => Ok(DbConstraintType::NotNull),
            _ => Err(ParserError::Constraint(format!(
                "{} is not valid db constraint",
                input
            ))),
        }
    }
}

pub fn parse_constraint(value: &str) -> Result<String, ParserError> {
    if value.is_empty() {
        panic!("value can't be empty")
    }
    let column_type = DbConstraintType::from_str(value)?;
    match column_type {
        DbConstraintType::PrimaryKey => Ok("PRIMARY KEY".to_string()),
        DbConstraintType::Unique => Ok(value.to_uppercase()),
        DbConstraintType::ForeignKey => Ok("FOREIGN KEY".to_string()),
        DbConstraintType::Check => Ok(value.to_uppercase()),
        DbConstraintType::Default => Ok(value.to_uppercase()),
        DbConstraintType::NotNull => Ok("NOT NULL".to_string()),
    }
}

#[cfg(test)]
mod db_constraint_test {
    use super::*;

    #[test]
    fn test_from_str_unknown_type() {
        let result = DbConstraintType::from_str("foobar");
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_parse_constraint_valid() {
        if let Ok(result) = parse_constraint("notnull") {
            assert_eq!("NOT NULL", result)
        };
    }
}
