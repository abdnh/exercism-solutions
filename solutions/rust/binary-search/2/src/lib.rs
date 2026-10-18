pub fn find<T: Ord, I: AsRef<[T]>>(array: I, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut start = 0;
    let mut end = array.len();
    while end - start != 0 {
        let mid = start + (end - start) / 2;
        if array[mid] == key {
            return Some(mid);
        }
        if array[mid] > key {
            end = mid;
        } else if array[mid] < key {
            start = mid + 1;
        }
    }

    None
}
