use std::cmp::Ordering;

pub fn find<A, T>(array: A, key: T) -> Option<usize>
where
    A: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let mut l_idx = 0;
    let mut r_idx = array.len() - 1;
    while l_idx <= r_idx {
        let m_idx: usize = l_idx + (r_idx - l_idx) / 2;
        match array[m_idx].cmp(&key) {
            Ordering::Equal => {
                return Some(m_idx);
            }
            Ordering::Greater => {
                if m_idx == 0 {
                    break;
                }
                r_idx = m_idx - 1;
            }
            Ordering::Less => {
                l_idx = m_idx + 1;
            }
        }
    }
    None
}
