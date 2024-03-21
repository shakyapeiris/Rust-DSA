use rust_dsa::queue::Queue;

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
        }
        Err(e) => panic!("{}", e),
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
        }
        Err(e) => panic!("{}", e),
    }
}
