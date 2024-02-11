use crate::{
    errors::parser_error::ParserError, parser::parser::FieldSchema,
    utils::remove_last_comma::remove_last_comma,
};

fn format_insert_into_row(
    table_name: &str,
    fields: &Vec<FieldSchema>,
) -> Result<String, ParserError> {
    if fields.len() == 0 {
        return Err(ParserError::Schema(format!("fields array cant be empty")));
    }
    let mut base = format!("INSERT INTO {table_name} (\n");
    for schema in fields.iter() {
        let row = format!("  {}", schema.key);
        base += &row;
        base += ",\n";
    }
    remove_last_comma(&mut base);
    base += ")\n";
    Ok(base)
}
fn format_values(fields: &Vec<FieldSchema>) -> Result<String, ParserError> {
    if fields.len() == 0 {
        return Err(ParserError::Schema(format!("fields array cant be empty")));
    }
    let mut base = "VALUES (\n".to_string();
    for schema in fields.iter() {
        let row = format!("  '{}'", schema.key);
        base += &row;
        base += ",\n";
    }
    remove_last_comma(&mut base);
    base += ");\n";
    Ok(base)
}

pub fn insert_into_template(
    table_name: &str,
    fields: &Vec<FieldSchema>,
) -> Result<String, ParserError> {
    let mut base = format_insert_into_row(table_name, &fields)?;
    base += format_values(&fields)?.as_str();
    Ok(base)
}

#[cfg(test)]
mod insert_into_test {
    use super::*;

    #[test]
    fn insert_into_template_successfully() {
        let table_name = "users";
        let fields = vec![
            FieldSchema::new(
                "name".to_string(),
                "text".to_string(),
                vec!["notnull".to_string(), "primarykey".to_string()],
            ),
            FieldSchema::new("age".to_string(), "integer".to_string(), vec![]),
        ];
        let table = insert_into_template(table_name, &fields).unwrap();
        let expected =
            format!("INSERT INTO users (\n  name,\n  age\n)\nVALUES (\n  'name',\n  'age'\n);\n");
        assert_eq!(table, expected);
    }
}
