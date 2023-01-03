use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prior: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prior: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Node::new(elem);

        match self.head.take() {
            // empty list
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node)
            }
            Some(old_head) => {
                node.borrow_mut().next = Some(old_head.clone());
                old_head.borrow_mut().prior = Some(node.clone());
                self.head = Some(node);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let node = Node::new(elem);

        match self.tail.take() {
            // empty list
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node)
            }
            Some(old_tail) => {
                node.borrow_mut().prior = Some(old_tail.clone());
                old_tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                // empty after pop
                None => {
                    self.tail = None;
                }
                Some(new_head) => {
                    new_head.borrow_mut().prior = None;
                    self.head = Some(new_head);
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prior.take() {
                // empty after pop
                None => {
                    self.head = None;
                }
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |n| &n.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |n| &n.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|head| RefMut::map(head.borrow_mut(), |n| &mut n.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |n| &mut n.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    #[allow(unused)]
    pub fn iter(&self) -> Iter<T> {
        let a = self.head.as_ref().map(|head| head.borrow());
        Iter(a)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

struct IntoIter<T>(List<T>);

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

struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
        // self.0.take().map(|n_ref| {
        //     self.0 = n_ref.next.as_ref().map(|n| n.borrow());
        //     Ref::map(n_ref, |n| &n.elem)
        // })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
