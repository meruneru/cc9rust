#[macro_use]
extern crate log;
/// strtol
/// str to long
///
/// ```
/// let (v, s) = cc9rust::strtol("123+20");
/// assert_eq!(v.unwrap(), 123);
/// assert_eq!(s, "+20");
/// ```
pub fn strtol(s: &str) -> (Option<i64>, &str) {
    if s.is_empty() || !char::is_numeric(s.chars().nth(0).unwrap()) {
        return (None, s);
    }
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    let (v, remain) = s.split_at(first_non_num);
    info!("v={:?}, remain: {:?}", v, remain);
    match v.parse::<i64>() {
        Ok(t) => (Some(t), remain),
        Err(_e) => (None, remain),
    }
}
