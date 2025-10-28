pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            count += 1;
            current_node = &node.next;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new_head_node = Node {
            data: element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_head_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut new_sll = Self::new();
        while let Some(data) = self.pop() {
            new_sll.push(data);
        }
        new_sll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = Self::new();
        for item in iter {
            res.push(item);
        }
        res
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        while let Some(data) = linked_list.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}
