pub fn median(list: &[i64]) -> Option<i64> {
    let mut list = list.to_owned();
    let len = list.len();
    list.sort_unstable();
    if len == 0 {
        return None;
    }
    if len.is_multiple_of(2) {
        let (a, b) = {
            let mid = len / 2;
            (mid, mid - 1)
        };

        let num1 = list.get(a);
        let num2 = list.get(b);

        match (num1, num2) {
            (Some(num1), Some(num2)) => Some((num1 + num2) / 2),
            _ => None,
        }
    } else {
        let num = (len / 2) + 1;
        let mid = list.get(num);

        mid.copied()
    }
}
