#[cfg(test)]
mod tests {
    use crate::value;

    #[test]
    fn test() {
        let v = value!("abc");
        assert_eq!(v.as_str().unwrap(), "abc");
    }
}
