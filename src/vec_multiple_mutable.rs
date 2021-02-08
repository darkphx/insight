// https://stackoverflow.com/a/30075629
pub enum Pair<T> {
    Both(T, T),
    One(T),
    None,
}

pub fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Pair<&mut T> {
    if a == b {
        slc.get_mut(a).map_or(Pair::None, Pair::One)
    } else {
        if a >= slc.len() || b >= slc.len() {
            Pair::None
        } else {
            // safe because a, b are in bounds and distinct
            unsafe {
                let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
                let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
                Pair::Both(ar, br)
            }
        }
    }
}