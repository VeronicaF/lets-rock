use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prior: Link<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        // Oh look it's drop again
        while let Some(_) = self.pop_front() {}
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_front = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                next: None,
                prior: None,
            })));

            if let Some(old_front) = self.front {
                (*old_front.as_ptr()).prior = Some(new_front);
                (*new_front.as_ptr()).next = Some(old_front)
            } else {
                self.back = Some(new_front);
            }
            self.front = Some(new_front);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|n| {
                let mut boxed_node = Box::from_raw(n.as_ptr());

                self.front = boxed_node.next;
                boxed_node.next = None;
                if let Some(new_front) = self.front {
                    (*new_front.as_ptr()).prior = None
                } else {
                    self.back = None
                }
                self.len -= 1;
                boxed_node.elem
            })
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new_back = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                next: None,
                prior: None,
            })));
            if self.len == 0 {
                self.front = Some(new_back);
            } else {
                self.back.map(|old_back| {
                    (*old_back.as_ptr()).next = Some(new_back);
                    (*new_back.as_ptr()).prior = Some(old_back);
                });
            }
            self.back = Some(new_back);
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.map(|n| unsafe {
            let boxed_node = Box::from_raw(n.as_ptr());
            self.back = boxed_node.prior;
            if let Some(new_back) = self.back {
                (*new_back.as_ptr()).next = None
            } else {
                self.front = None
            }
            self.len -= 1;
            boxed_node.elem
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        self.front.map(|n| unsafe { &(*n.as_ptr()).elem })
    }

    pub fn front_mut(&self) -> Option<&mut T> {
        self.front.map(|n| unsafe { &mut (*n.as_ptr()).elem })
    }

    pub fn back(&self) -> Option<&T> {
        self.back.map(|n| unsafe { &(*n.as_ptr()).elem })
    }

    pub fn back_mut(&self) -> Option<&mut T> {
        self.back.map(|n| unsafe { &mut (*n.as_ptr()).elem })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    /// ```
    /// use too_many_lists::fifth::IterMut;
    ///
    /// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>) -> IterMut<'i, &'a T> { x }
    /// ```
    fn iter_mut_invariant() {}

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.front.map(|n| unsafe {
                self.front = (*n.as_ptr()).next;
                self.len -= 1;
                &(*n.as_ptr()).elem
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.back.map(|n| unsafe {
                self.back = (*n.as_ptr()).prior;
                self.len -= 1;
                &(*n.as_ptr()).elem
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.front.map(|n| unsafe {
                self.front = (*n.as_ptr()).next;
                self.len -= 1;
                &mut (*n.as_ptr()).elem
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.back.map(|n| unsafe {
                self.back = (*n.as_ptr()).prior;
                self.len -= 1;
                &mut (*n.as_ptr()).elem
            })
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        self.iter().for_each(|n| new_list.push_back(n.clone()));
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item)
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();
        for item in iter {
            new_list.push_back(item);
        }
        new_list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

impl<T> LinkedList<T> {
    fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            cur: None,
            list: self,
            index: None,
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    fn index(&self) -> Option<usize> {
        self.index
    }

    fn move_next(&mut self) {
        unsafe {
            if let Some(old_cur) = self.cur {
                // We're on a real element, go to its next (back)
                self.cur = (*old_cur.as_ptr()).next;
                if self.cur.is_none() {
                    // walked to the ghost, no more index
                    self.index = None
                } else {
                    *self.index.as_mut().unwrap() += 1;
                }
            } else {
                if !self.list.is_empty() {
                    self.cur = self.list.front;
                    self.index = Some(0)
                }
            }
        }
    }

    fn move_prior(&mut self) {
        unsafe {
            if let Some(old_cur) = self.cur {
                // We're on a real element, go to its prior (front)
                self.cur = (*old_cur.as_ptr()).prior;
                // update index
                if self.cur.is_none() {
                    // walked to the ghost, no more index
                    self.index = None
                } else {
                    *self.index.as_mut().unwrap() -= 1;
                }
            } else {
                if !self.list.is_empty() {
                    self.cur = self.list.back;
                    self.index = Some(self.list.len - 1)
                }
            }
        }
    }

    fn current(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).elem) }
    }

    fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur
                .and_then(|n| (*n.as_ptr()).next)
                .map(|n| &mut (*n.as_ptr()).elem)
        }
    }

    fn peek_prior(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur
                .and_then(|n| (*n.as_ptr()).prior)
                .map(|n| &mut (*n.as_ptr()).elem)
        }
    }

    fn split_before(&mut self) -> LinkedList<T> {
        /*!
           1. The normal case
           2. The normal case, but prev is the ghost
           3. The ghost case, where we return the whole list and become empty
           4. The ghost case, but the list is empty, so do nothing and return the empty list
        */
        unsafe {
            if let Some(cur) = self.cur {
                /*
                    list.front -> A <-> B <-> C <-> D <- list.back
                                              ^
                                             cur

                    list.front -> C <-> D <- list.back
                                  ^
                                 cur

                    return.front -> A <-> B <- return.back
                */
                // old state
                let old_idx = self.index.unwrap();
                let old_len = self.list.len;
                let prior = (*cur.as_ptr()).prior;

                // new state
                let new_len = old_len - old_idx;
                let new_front = cur;
                let new_back = self.list.back;
                let new_idx = Some(0);

                // output list state
                let output_len = old_idx;
                let output_front = self.list.front;
                let output_back = prior;

                if let Some(prior) = prior {
                    (*cur.as_ptr()).prior = None;
                    (*prior.as_ptr()).next = None;
                }

                self.list.front = Some(new_front);
                self.list.len = new_len;
                self.list.back = new_back;
                self.index = new_idx;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            } else {
                // case 3 and case 4
                mem::replace(self.list, LinkedList::new())
            }
        }
    }

    fn split_after(&mut self) -> LinkedList<T> {
        /*!
           1. The normal case
           2. The normal case, but next is the ghost
           3. The ghost case, where we return the whole list and become empty
           4. The ghost case, but the list is empty, so do nothing and return the empty list
        */
        unsafe {
            if let Some(cur) = self.cur {
                /*
                    we have this:
                        list.front -> A <-> B <-> C <-> D <- list.back
                                            ^
                                           cur
                    we get this:
                        list.front -> A <-> B <- list.back
                                            ^
                                           cur

                        return.front -> C <-> D <- return.back
                */
                // old state
                let old_idx = self.index.unwrap();
                let old_next = (*cur.as_ptr()).next;
                let old_len = self.list.len;

                // new state
                let new_idx = Some(old_idx);
                let new_front = self.list.front;
                let new_back = Some(cur);
                let new_len = old_idx + 1;

                // output list state
                let output_len = old_len - new_len;
                let output_front = old_next;
                let output_back = self.list.back;

                if let Some(next) = old_next {
                    (*cur.as_ptr()).next = None;
                    (*next.as_ptr()).prior = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_idx;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            } else {
                mem::replace(self.list, LinkedList::new())
            }
        }
    }

    fn splice_before(&mut self, mut input: LinkedList<T>) {
        // we have:
        //      input.front -> 1 <-> 2 <- input.back
        //
        //      list.front -> A <-> B <-> C <- list.back
        //                          ^
        //                         cur
        // we get:
        //      list.front -> A <-> 1 <-> 2 <-> B <-> C <- list.back
        if input.is_empty() {
            // input is empty, do nothing
            return;
        }

        unsafe {
            if let Some(cur) = self.cur {
                // both list are not empty
                let in_front = input.front.take().unwrap();
                let in_back = input.front.take().unwrap();
                if let Some(prior) = (*cur.as_ptr()).prior {
                    // general case
                    (*prior.as_ptr()).next = Some(in_front);
                    (*in_back.as_ptr()).next = Some(cur);
                    (*cur.as_ptr()).prior = Some(in_back);
                    (*in_front.as_ptr()).prior = Some(prior);
                } else {
                    // no prior, append to front
                    (*cur.as_ptr()).prior = Some(in_back);
                    (*in_back.as_ptr()).next = Some(cur);
                    self.list.front = Some(in_front);
                }
                // change index
                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*back.as_ptr()).next = Some(in_front);
                (*in_front.as_ptr()).prior = Some(back);
                self.list.back = Some(in_back);
            } else {
                mem::swap(self.list, &mut input);
            }
            self.list.len += input.len;
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
            .iter()
            .copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }
}
