pub fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
//input data must live long enough than the returned reference to it does.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}