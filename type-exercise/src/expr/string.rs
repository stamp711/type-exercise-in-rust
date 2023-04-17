pub fn str_contains(i1: &str, i2: &str) -> bool {
    i1.contains(i2)
}

pub fn str_concat(i1: &str, i2: &str) -> String {
    i1.to_owned() + i2
}
