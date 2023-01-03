struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, elem: T) {
        let n = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(n);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            ptr: self.head.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            ptr: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_val) = cur_link {
            cur_link = boxed_val.next.take()
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|n| {
            self.0.head = n.next;
            n.elem
        })
    }
}

struct Iter<'a, T> {
    ptr: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.map(|n| {
            self.ptr = n.next.as_deref();
            &n.elem
        })
    }
}

struct IterMut<'a, T> {
    ptr: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|n| {
            self.ptr = n.next.as_deref_mut();
            &mut n.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
