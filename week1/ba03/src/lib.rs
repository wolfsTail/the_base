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

pub fn merge_sort_buf<T>(v: &mut [T])
where
    T: Ord + Clone,
{
    if v.len() <= 1 {
        return;
    }
    let mut buf = v.to_vec();
    split_and_merge(v, &mut buf);
}

fn split_and_merge<T>(v: &mut [T], buf: &mut [T])
where
    T: Ord + Clone,
{
    if v.len() <= 1 {
        return;
    }
    let mid = v.len() / 2;
    {
        let (v_l, v_r) = v.split_at_mut(mid);
        let (b_l, b_r) = buf.split_at_mut(mid);
        split_and_merge(v_l, b_l);
        split_and_merge(v_r, b_r);
    }
    {
        let (l, r) = v.split_at(mid);
        let (mut i, mut j, mut k) = (0_usize, 0_usize, 0_usize);
        while i < l.len() && j < r.len() {
            if l[i] <= r[j] {
                buf[k].clone_from(&l[i]);
                i += 1;
            } else {
                buf[k].clone_from(&r[j]);
                j += 1;
            }
            k += 1;
        }
        while i < l.len() {
            buf[k].clone_from(&l[i]);
            i += 1;
            k += 1;
        }
        while j < r.len() {
            buf[k].clone_from(&r[j]);
            j += 1;
            k += 1;
        }
    }
    for t in 0..v.len() {
        let (vv, bb) = (&mut v[t], &buf[t]);
        vv.clone_from(bb);
    }
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

    #[test]
    fn test_buf_mergesort() {
        let cases: [&[i8]; 4] = [&[], &[1], &[5, 4, 3, 2], &[1, 2, 10, 8, 6, 0]];
        for c in cases {
            let mut v: Vec<i8> = c.iter().map(|x| *x).collect();
            let mut expected = v.clone();
            expected.sort();
            merge_sort_buf(&mut v);
            assert_eq!(v, expected);
        }
    }
}
