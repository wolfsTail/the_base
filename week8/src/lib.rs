use std::fmt;
use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

pub const CAPACITY: usize = 32;

#[derive(Clone)]
pub struct MyVec<T> {
    data: [Option<T>; CAPACITY],
    len: usize,
}

impl<T> MyVec<T> {
    pub const CAPACITY: usize = CAPACITY;

    pub fn new() -> Self {
        Self {
            data: [const { None }; CAPACITY],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.data[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        let last = self.len.checked_sub(1)?;
        self.len = last;
        self.data[last].take()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data[..self.len].get(idx).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.data[..self.len].get_mut(idx).and_then(Option::as_mut)
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn last(&self) -> Option<&T> {
        self.get(self.len.checked_sub(1)?)
    }

    pub fn iter(&self) -> MyVecIter<'_, T> {
        MyVecIter { vec: self, pos: 0 }
    }

    pub fn clear(&mut self) {
        for slot in &mut self.data[..self.len] {
            *slot = None;
        }
        self.len = 0;
    }
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for MyVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for MyVec<T> {}

impl<T> Index<usize> for MyVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        match self.get(idx) {
            Some(item) => item,
            None => index_out_of_bounds(self.len, idx),
        }
    }
}

impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        let len = self.len;
        match self.get_mut(idx) {
            Some(item) => item,
            None => index_out_of_bounds(len, idx),
        }
    }
}

#[cold]
#[track_caller]
fn index_out_of_bounds(len: usize, idx: usize) -> ! {
    panic!("index out of bounds: the len is {len} but the index is {idx}")
}

impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = MyVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,
    pos: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.pos)?;
        self.pos += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len().saturating_sub(self.pos);
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for MyVecIter<'_, T> {}

impl<T> FusedIterator for MyVecIter<'_, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct DropCounter(Rc<Cell<i32>>);

    impl DropCounter {
        fn new(counter: &Rc<Cell<i32>>) -> Self {
            Self(Rc::clone(counter))
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn push_pop_basic() {
        let mut v: MyVec<i32> = MyVec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn iter_borrows() {
        let mut v: MyVec<String> = MyVec::new();
        v.push("a".into()).unwrap();
        v.push("b".into()).unwrap();
        let collected: Vec<&String> = v.iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected, [&"a".to_string(), &"b".to_string()]);
    }

    #[test]
    fn capacity_overflow() {
        let mut v: MyVec<i32> = MyVec::new();
        for i in 0..32 {
            v.push(i).unwrap();
        }
        assert!(v.is_full());
        assert!(v.push(99).is_err());
        assert_eq!(v.push(99), Err(99));
        assert_eq!(v.len(), 32);
        assert_eq!(v.last(), Some(&31));
    }

    #[test]
    fn drop_called() {
        let drops = Rc::new(Cell::new(0));

        let mut v = MyVec::new();
        for _ in 0..5 {
            v.push(DropCounter::new(&drops)).unwrap();
        }
        assert_eq!(
            drops.get(),
            0,
            "пока элементы в векторе, ничего не дропнуто"
        );

        v.clear();
        assert_eq!(drops.get(), 5, "clear() должен дропнуть каждый элемент");
        assert!(v.is_empty());

        v.clear();
        assert_eq!(drops.get(), 5);
    }

    #[test]
    fn new_is_empty() {
        let mut v: MyVec<String> = MyVec::new();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
        assert_eq!(v.pop(), None);
        assert_eq!(v.first(), None);
        assert_eq!(v.iter().next(), None);
    }

    #[test]
    fn get_respects_len_not_capacity() {
        let mut v = MyVec::new();
        v.push(10).unwrap();
        v.push(20).unwrap();

        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(1), Some(&20));
        assert_eq!(v.get(2), None, "индекс равен len");
        assert_eq!(v.get(31), None, "слот есть, но он свободен");
        assert_eq!(v.get(1000), None);
    }

    #[test]
    fn get_mut_mutates_in_place() {
        let mut v = MyVec::new();
        v.push(String::from("hi")).unwrap();

        v.get_mut(0).unwrap().push_str(", there");
        assert_eq!(v.get(0).map(String::as_str), Some("hi, there"));
        assert!(v.get_mut(1).is_none());
    }

    #[test]
    fn index_operators() {
        let mut v = MyVec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();

        v[1] *= 10;
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 20);
    }

    #[test]
    #[should_panic(expected = "the len is 1 but the index is 1")]
    fn index_out_of_bounds_panics() {
        let mut v = MyVec::new();
        v.push('x').unwrap();
        let _ = v[1];
    }

    #[test]
    fn pop_moves_out_without_dropping() {
        let drops = Rc::new(Cell::new(0));
        let mut v = MyVec::new();
        v.push(DropCounter::new(&drops)).unwrap();

        let popped = v.pop().expect("элемент был");
        assert_eq!(drops.get(), 0, "pop передаёт владение, а не дропает");
        assert!(v.is_empty());

        drop(popped);
        assert_eq!(drops.get(), 1);
    }

    #[test]
    fn dropping_vec_drops_elements() {
        let drops = Rc::new(Cell::new(0));
        {
            let mut v = MyVec::new();
            v.push(DropCounter::new(&drops)).unwrap();
            v.push(DropCounter::new(&drops)).unwrap();
            assert_eq!(Rc::strong_count(&drops), 3);
        }
        assert_eq!(drops.get(), 2);
        assert_eq!(Rc::strong_count(&drops), 1);
    }

    #[test]
    fn overwrite_via_get_mut_drops_old_value() {
        let drops = Rc::new(Cell::new(0));
        let mut v = MyVec::new();
        v.push(DropCounter::new(&drops)).unwrap();

        *v.get_mut(0).unwrap() = DropCounter::new(&drops);
        assert_eq!(drops.get(), 1, "старое значение дропнуто при замене");
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn slots_are_reusable_after_pop_and_clear() {
        let mut v = MyVec::new();
        for i in 0..32 {
            v.push(i).unwrap();
        }
        assert!(v.push(-1).is_err());

        v.pop();
        v.push(-1).unwrap();
        assert_eq!(v[31], -1);

        v.clear();
        for i in 0..32 {
            v.push(i * 2).unwrap();
        }
        assert_eq!(v.len(), 32);
        assert_eq!(v[31], 62);
    }

    #[test]
    fn iter_yields_in_order_and_is_exact_size() {
        let mut v = MyVec::new();
        for i in 1..=4 {
            v.push(i).unwrap();
        }

        let mut it = v.iter();
        assert_eq!(it.len(), 4);
        assert_eq!(it.size_hint(), (4, Some(4)));

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.len(), 3);

        assert_eq!(it.by_ref().collect::<Vec<_>>(), [&2, &3, &4]);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None, "итератор fused: после None снова None");
    }

    #[test]
    fn references_outlive_the_iterator() {
        let mut v = MyVec::new();
        v.push(String::from("keep me")).unwrap();

        let first: &String = {
            let mut it = v.iter();
            it.next().unwrap()
        };
        assert_eq!(first, "keep me");
    }

    #[test]
    fn for_loop_over_reference() {
        let mut v = MyVec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();

        let mut sum = 0;
        for x in &v {
            sum += x;
        }
        assert_eq!(sum, 6);
        assert_eq!(v.len(), 3, "`&v` не забирает вектор");
    }

    #[test]
    fn debug_eq_clone_default() {
        let mut a: MyVec<i32> = MyVec::default();
        a.push(1).unwrap();
        a.push(2).unwrap();
        assert_eq!(format!("{a:?}"), "[1, 2]");

        let b = a.clone();
        assert_eq!(a, b);

        a.pop();
        assert_ne!(a, b);
    }
}
