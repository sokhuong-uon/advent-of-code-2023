pub fn main() -> String {
    let list = vec![1, 3, 5, 7, 9];
    let item = &3;
    let result = binary_search(&list, item);

    format!(
        "binary_search({:?}, {}) = {:?}",
        list,
        item,
        result.unwrap()
    )
}

fn binary_search<T: Ord>(list: &[T], item: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let guess = &list[mid];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    None
}
