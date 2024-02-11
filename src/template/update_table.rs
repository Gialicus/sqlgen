use crate::{
    errors::parser_error::ParserError, parser::parser::FieldSchema,
    utils::remove_last_comma::remove_last_comma,
};

pub fn update_template(table_name: &str, fields: &Vec<FieldSchema>) -> Result<String, ParserError> {
    if fields.len() == 0 {
        return Err(ParserError::Schema(format!("fields array cant be empty")));
    }
    let mut base = format!("UPDATE {table_name} SET\n");
    for schema in fields
        .iter()
        .filter(|field| field.key.to_lowercase() != "id")
    {
        let row = format!("  {} = '{}',\n", schema.key, schema.key);
        base += &row;
    }
    remove_last_comma(&mut base);
    base += "WHERE id = 'id';\n";
    Ok(base)
}

#[cfg(test)]
mod update_table_test {
    use super::*;

    #[test]
    fn update_template_successfully() {
        let table_name = "users";
        let fields = vec![
            FieldSchema::new(
                "name".to_string(),
                "text".to_string(),
                vec!["notnull".to_string(), "primarykey".to_string()],
            ),
            FieldSchema::new("age".to_string(), "integer".to_string(), vec![]),
        ];
        let table = update_template(table_name, &fields).unwrap();
        let expected =
            format!("UPDATE users SET\n  name = 'name',\n  age = 'age'\nWHERE id = 'id';\n");
        assert_eq!(table, expected);
    }
}
