fn print_me() -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        assert_eq!(super::print_me(), String::from("Hello"));
    }

    #[test]
    fn another_test() {
        assert_eq!(true, true);
    }
}
