#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Clone)]
pub struct SimpleLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            count += 1;
            node = &n.next
        }

        count
    }

    pub fn push(&mut self, element: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(element)));
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node::new(element)));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.as_ref()?.next.is_none() {
            return self.head.take().map(|b| b.data);
        }

        let mut cur = &mut self.head;
        while cur.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.as_mut().unwrap().next.take().map(|b| b.data)
    }

    pub fn peek(&self) -> Option<&T> {
        let mut node = &self.head;
        while let Some(n) = node {
            if n.next.is_none() {
                return Some(&n.data);
            }
            node = &n.next;
        }

        None
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut v: Vec<T> = self.into();
        v.reverse();
        for item in v {
            reversed.push(item);
        }
        reversed
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter.into_iter() {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = vec![];
        while let Some(e) = linked_list.pop() {
            v.push(e);
        }
        v.reverse();
        v
    }
}
