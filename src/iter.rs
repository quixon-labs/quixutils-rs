#[allow(dead_code)]
pub fn split_including_terminator(s: &str, c: char) -> impl Iterator<Item=&str> {
    let n = c.len_utf8();
    let mut end = 0;
    s.match_indices(c).map(move |(i, _)| {
        let start = end;
        end = i + n;
        &s[start..end]
    })
}