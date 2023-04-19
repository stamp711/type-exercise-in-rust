pub fn str_contains(i1: &str, i2: &str) -> bool {
    i1.contains(i2)
}

pub fn str_concat(i1: &str, i2: &str) -> String {
    i1.to_owned() + i2
}

#[cfg(test)]
mod test {
    use crate::array::*;
    use crate::expr::*;
    use crate::test_util::*;

    #[test]
    fn test_str_contains() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains);
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
                &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
            )
            .unwrap();
        check_array_eq::<BoolArray>(
            (&result).try_into().unwrap(),
            &[Some(true), Some(false), None],
        );
    }

    #[test]
    fn test_concat_string() {
        let expr = BinaryExpression::<StringArray, StringArray, StringArray, _>::new(str_concat);
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("aa"), Some("bb"), None]).into(),
                &StringArray::from_slice(&[Some("aa"), None, Some("cc")]).into(),
            )
            .unwrap();
        check_array_eq::<StringArray>((&result).try_into().unwrap(), &[Some("aaaa"), None, None]);
    }
}
