pub mod linked_list;

#[cfg(test)]
mod tests {
    use self::linked_list::Node;

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn get_new_value<T>(head: &Node<T>) -> &Option<T>{
        match &head.next {
            Some(node) => get_new_value(&node),
            None => &head.value
        }
    }

    #[test]
    fn create_a_new_node() {
        let new_node = linked_list::Node::new(Some(10));

        assert_eq!(new_node.next, None);
        assert_eq!(new_node.value, Some(10));
    }

    #[test] 
    fn inserts_a_new_node(){
        let mut new_node = linked_list::Node::new(Some(10));
        new_node.insert(Some(100));

        let newly_inserted_value = get_new_value(&new_node);
        assert_eq!(*newly_inserted_value, Some(100))
    }

    #[test]
    fn reverse_a_list() {
        let mut new_node = linked_list::Node::new(Some(10));
        new_node.insert(Some(100));
        new_node.insert(Some(200));

        let new_head = new_node.reverse(None);
        assert_eq!(new_head.value, Some(200))
    }
    
}
