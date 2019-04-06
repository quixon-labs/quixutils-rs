pub(crate) struct SliceIterIndexed<'a, T> {
    elem: &'a [T],
    curr_index: usize,
}

impl<'a, T> SliceIterIndexed<'a, T> {
    pub(crate) fn new(elem: &'a [T]) -> SliceIterIndexed<'a, T> {
        SliceIterIndexed {
            elem,
            curr_index: 0,
        }
    }
}

pub(crate) struct HeadsIter<'a, T>(pub(crate) SliceIterIndexed<'a, T>);
pub(crate) struct TailsIter<'a, T>(pub(crate) SliceIterIndexed<'a, T>);

impl<'a, T> Iterator for TailsIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let e: &[T] = self.0.elem;
        let i = self.0.curr_index;
        if i < e.len() {
            self.0.curr_index += 1;
            return Some(e.split_at(i).1);
        }
        None
    }
}

impl<'a, T> Iterator for HeadsIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let e: &[T] = self.0.elem;
        let i = self.0.curr_index + 1;
        if i <= e.len() {
            self.0.curr_index += 1;
            return Some(e.split_at(i).0);
        }
        None
    }
}
