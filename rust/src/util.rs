pub fn copy<T: Clone>(into: &mut [T], from: &[T]) {
    use std::cmp::min;

    let l = min(into.len(), from.len());
    for i in 0 .. l {
        into[i] = from[i].clone();
    }
}
