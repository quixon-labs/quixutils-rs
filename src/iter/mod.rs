mod internal;

use self::internal::*;

/// Like split, but includes the delimiter
pub fn split_with_delimiter(s: &str, c: char) -> impl Iterator<Item = &str> {
    let n = c.len_utf8();
    let mut end = 0;
    s.match_indices(c).map(move |(i, _)| {
        let start = end;
        end = i + n;
        &s[start..end]
    })
}

/// Provides slices of all heads. Similar to Haskell `inits`.
pub fn heads<T>(s: &[T]) -> impl Iterator<Item = &[T]> {
    let inner_iter = SliceIterIndexed::new(s);
    HeadsIter(inner_iter)
}

/// Provides slices of all tails. Similar to Haskell `tails`.
pub fn tails<T>(s: &[T]) -> impl Iterator<Item = &[T]> {
    let inner_iter = SliceIterIndexed::new(s);
    TailsIter(inner_iter)
}
