use crate::parser::parser::FieldSchema;

pub fn create_table_template(table_name: &str, fields: Vec<FieldSchema>) -> String {
    let mut base = format!("CREATE TABLE {table_name} (\n");
    for schema in fields.iter() {
        let row = format!("  {} {},\n", schema.key, schema.db_type);
        base += &row;
    }
    if let Some(last_comma_position) = base.rfind(',') {
        base.remove(last_comma_position);
    }
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
            FieldSchema::new("name".to_string(), "text".to_string(), vec![]),
            FieldSchema::new("age".to_string(), "integer".to_string(), vec![]),
        ];
        let table = create_table_template(table_name, fields);
        let expected = format!("CREATE TABLE users (\n  name text,\n  age integer\n);\n");
        assert_eq!(table, expected);
    }
}
