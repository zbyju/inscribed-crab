pub fn words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn chars(content: &str) -> usize {
    content.len()
}
