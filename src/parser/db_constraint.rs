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
    pub fn from_str(input: &str) -> DbConstraintType {
        match input.to_lowercase().as_str() {
            "primarykey" => DbConstraintType::PrimaryKey,
            "unique" => DbConstraintType::Unique,
            "foreingkey" => DbConstraintType::ForeignKey,
            "check" => DbConstraintType::Check,
            "default" => DbConstraintType::Default,
            "notnull" => DbConstraintType::NotNull,
            _ => panic!("{} is not valid db constraint", input),
        }
    }
}

pub fn parse_constraint(value: &str) -> String {
    if value.is_empty() {
        panic!("value can't be empty")
    }
    let column_type = DbConstraintType::from_str(value);
    match column_type {
        DbConstraintType::PrimaryKey => "PRIMARY KEY".to_string(),
        DbConstraintType::Unique => value.to_uppercase(),
        DbConstraintType::ForeignKey => "FOREIGN KEY".to_string(),
        DbConstraintType::Check => value.to_uppercase(),
        DbConstraintType::Default => value.to_uppercase(),
        DbConstraintType::NotNull => "NOT NULL".to_string(),
    }
}

#[cfg(test)]
mod db_constraint_test {
    use super::*;

    #[test]
    #[should_panic(expected = "foobar is not valid db constraint")]
    fn test_from_str_unknown_type() {
        DbConstraintType::from_str("foobar");
    }

    #[test]
    fn test_parse_constraint_valid() {
        let result = parse_constraint("notnull");
        assert_eq!("NOT NULL", result)
    }
}
