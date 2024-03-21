pub mod linked_list;
pub mod binary_search_tree;
pub mod std_tree;
pub mod queue;
pub mod stack;

#[cfg(test)]
mod tests {
    use self::linked_list::Node;
    use queue::Queue;
    use super::*;

    fn get_new_value<T>(head: &Node<T>) -> &Option<T>{
        match &head.next {
            Some(node) => get_new_value(&node),
            None => &head.value
        }
    }

    #[test]
    fn test_create_a_new_node() {
        let new_node = linked_list::Node::new(Some(10));

        assert_eq!(new_node.next, None);
        assert_eq!(new_node.value, Some(10));
    }

    #[test] 
    fn test_inserts_a_new_node(){
        let mut new_node = linked_list::Node::new(Some(10));
        new_node.insert(Some(100));

        let newly_inserted_value = get_new_value(&new_node);
        assert_eq!(*newly_inserted_value, Some(100))
    }

    #[test]
    fn test_reverse_a_list() {
        let mut new_node = linked_list::Node::new(Some(10));
        new_node.insert(Some(100));
        new_node.insert(Some(200));

        let new_head = new_node.reverse(None);
        assert_eq!(new_head.value, Some(200));
        match new_head.next {
            Some(node) => assert_eq!(node.value, Some(100)),
            None => panic!("Head of the reverse list should point to the node with value 200")
        }
    }

    #[test]
    fn test_get_length(){
        let mut new_node = linked_list::Node::new(Some(10));
        new_node.insert(Some(100));
        new_node.insert(Some(200));

        assert_eq!(new_node.len(), 3)
    }

    #[test]
    fn test_new_queue() {
        let new_queue: Queue<i32> = Queue::new(10);
        assert!(new_queue.is_null());
        assert_eq!(new_queue.len(), 0);
    }

    #[test]
    fn test_queue_enqueue() -> Result<(), String> {
        let mut new_queue: Queue<i32> = Queue::new(10);
        new_queue.enqueue(1)?;
        new_queue.enqueue(2)?;
        new_queue.enqueue(3)?;

        assert_eq!(new_queue.len(), 3);
        match new_queue.peek() {
            Ok(num) => {
                assert_eq!(*num, 3);
                Ok(())
            },
            Err(e) => panic!("{}", e)
        }
    }
    
    #[test]
    fn test_queue_dequeue() -> Result<(), String> {
        let mut new_queue: Queue<i32> = Queue::new(10);
        new_queue.enqueue(1)?;
        new_queue.enqueue(2)?;
        new_queue.enqueue(3)?;
        new_queue.dequeue()?;

        assert_eq!(new_queue.len(), 2);
        match new_queue.peek() {
            Ok(num) => {
                assert_eq!(*num, 2);
                Ok(())
            },
            Err(e) => panic!("{}", e)
        }
    }
}
