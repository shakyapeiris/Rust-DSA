#[derive(PartialEq, Debug, Clone)]
pub struct Node<T> {
    pub next: Option<Box<Node<T>>>,
    pub value: Option<T>
}

impl<T> Node<T> {
    pub fn new(value: Option<T>) -> Node<T> {
        Node {
            next: None,
            value
        }
    }

    pub fn insert(&mut self, value: Option<T>){
        match &mut self.next {
            Some(node) => {
                node.insert(value)
            },
            None => {
                let new_node = Node {
                    next: None,
                    value
                };
                self.next = Some(Box::new(new_node))
            }
        }
    }

    pub fn reverse(self, prev: Option<Box<Node<T>>>) -> Node<T>{
        let next = self.next;
        // if next none, then it's the tail so return the new head
        // else previous will be the current one, current will be the new one
        match next {
            Some(node) => {
                let new_node = Node {
                    next: prev,
                    value: self.value
                };
                node.reverse(Some(Box::new(new_node)))
            },
            None => {
                Node {
                    next: prev,
                    value: self.value
                }
            }
        }

    }

    pub fn len(&self) -> i32{
        match &self.next {
            Some(node) => node.len() + 1,
            None => 1
        }
    }
}