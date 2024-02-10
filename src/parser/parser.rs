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

pub fn parse_field(value: &str) -> FieldSchema {
    if value.is_empty() {
        panic!("value cant be empty")
    }
    let mut splitted = value.split(":");
    let key = splitted
        .next()
        .expect(format!("Not found key in {}. use synthax 'key:db_type'", value).as_str());
    let db_type = splitted
        .next()
        .expect(format!("Not found db_type in {}. use synthax 'key:db_type'", value).as_str());
    if key.is_empty() {
        panic!("key can't be empty")
    }
    if db_type.is_empty() {
        panic!("db_type can't be empty")
    }
    let db_type = parse_db_type(db_type);
    let mut constraint = vec![];
    while let Some(c) = splitted.next() {
        constraint.push(c.to_string());
    }
    FieldSchema::new(key.to_string(), db_type, constraint)
}

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_field_successfully() {
        let result = parse_field("name:text");
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
    #[should_panic]
    fn parse_field_fail_for_missing_key() {
        parse_field(":text");
    }

    #[test]
    #[should_panic]
    fn parse_field_fail_for_missing_db_type() {
        parse_field("key:");
    }

    #[test]
    #[should_panic]
    fn parse_field_fail_for_only_column() {
        parse_field(":");
    }

    #[test]
    #[should_panic]
    fn parse_field_fail_for_empty_string() {
        parse_field("");
    }
}
