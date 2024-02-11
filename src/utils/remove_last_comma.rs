pub fn remove_last_comma(base: &mut String) {
    if let Some(last_comma_position) = base.rfind(',') {
        base.remove(last_comma_position);
    }
}

#[cfg(test)]
mod create_table_test {
    use super::*;

    #[test]
    fn remove_last_comma_successfully() {
        let mut t = "Hello, World!".to_string();
        remove_last_comma(&mut t);
        assert_eq!(t, "Hello World!")
    }
    #[test]
    fn remove_last_comma_with_not_comma() {
        let mut t = "Hello World!".to_string();
        remove_last_comma(&mut t);
        assert_eq!(t, "Hello World!")
    }
}
