mod internal;

use internal::*;

pub fn split_including_terminator(s: &str, c: char) -> impl Iterator<Item = &str> {
    let n = c.len_utf8();
    let mut end = 0;
    s.match_indices(c).map(move |(i, _)| {
        let start = end;
        end = i + n;
        &s[start..end]
    })
}

pub fn heads<T>(s: &[T]) -> impl Iterator<Item = &[T]> {
    let inner_iter = SliceIterIndexed::new(s);
    HeadsIter(inner_iter)
}

pub fn tails<T>(s: &[T]) -> impl Iterator<Item = &[T]> {
    let inner_iter = SliceIterIndexed::new(s);
    TailsIter(inner_iter)
}
