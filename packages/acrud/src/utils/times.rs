pub fn times<T, F>(nb: u32, f: F) -> Vec<T>
where
    F: FnMut(&std::ops::Range<u32>) -> T,
{
    let v = vec![1..nb];

    v.iter().map(f).collect()
}
