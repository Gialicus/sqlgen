use crate::{
    parser::{db_constraint::parse_constraint, db_type_parser::parse_db_type, parser::FieldSchema},
    utils::remove_last_comma::remove_last_comma,
};

pub fn create_table_template(table_name: &str, fields: &Vec<FieldSchema>) -> String {
    let mut base = format!("CREATE TABLE {table_name} (\n");
    for schema in fields.iter() {
        let mut row = format!("  {} {}", schema.key, parse_db_type(&schema.db_type));
        for constraint in schema.constraint.iter() {
            let constraint = parse_constraint(&constraint);
            row += (" ".to_owned() + &constraint).as_str();
        }
        base += &row;
        base += ",\n";
    }
    remove_last_comma(&mut base);
    base += ");\n";
    base
}

#[cfg(test)]
mod create_table_test {
    use super::*;

    #[test]
    fn create_table_template_successfully() {
        let table_name = "users";
        let fields = vec![
            FieldSchema::new(
                "name".to_string(),
                "text".to_string(),
                vec!["notnull".to_string(), "primarykey".to_string()],
            ),
            FieldSchema::new("age".to_string(), "integer".to_string(), vec![]),
        ];
        let table = create_table_template(table_name, &fields);
        let expected =
            format!("CREATE TABLE users (\n  name TEXT NOT NULL PRIMARY KEY,\n  age INTEGER\n);\n");
        assert_eq!(table, expected);
    }
}
