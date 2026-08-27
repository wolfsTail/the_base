pub fn merge_sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let mid = v.len() / 2;
    let right = v.split_off(mid);
    merge(merge_sort(v), merge_sort(right))
}

fn merge<T: Ord>(mut l: Vec<T>, mut r: Vec<T>) -> Vec<T> {
    let mut out = Vec::with_capacity(l.len() + r.len());

    while !l.is_empty() && !r.is_empty() {
        if l[l.len() - 1] > r[r.len() - 1] {
            out.push(l.pop().unwrap());
        } else {
            out.push(r.pop().unwrap());
        }
    }

    while !l.is_empty() {
        out.push(l.pop().unwrap());
    }
    while !r.is_empty() {
        out.push(r.pop().unwrap());
    }

    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_sort() {
        let cases: [&[&str]; 4] = [
            &[],
            &["a"],
            &["c", "b", "d", "a"],
            &["A", "a", "A", "a", "hello,", "world,"],
        ];
        for c in cases {
            let v: Vec<String> = c.iter().map(|x| x.to_string()).collect();
            let mut expected = v.clone();
            expected.sort();
            assert_eq!(merge_sort(v), expected);
        }
    }

    #[test]
    fn test_int_sort() {
        let cases: [&[i8]; 4] = [&[], &[1], &[5, 4, 3, 2], &[1, 2, 10, 8, 6, 0]];
        for c in cases {
            let v: Vec<i8> = c.iter().map(|x| *x).collect();
            let mut expected = v.clone();
            expected.sort();
            assert_eq!(merge_sort(v), expected);
        }
    }
}
