#[derive(Debug, Clone)]
pub struct Queue<T> {
    values: Vec<T>,
    max_length: u32
}


impl<T: Clone> Queue<T> {
    pub fn new(max_length: u32) -> Queue<T> {
        return Queue {
            values: vec![],
            max_length
        }
    }
    pub fn enqueue(&mut self, item: T) -> Result<(), String>{
        if self.values.len() == self.max_length as usize{
            return Err(String::from("Maximum size has been reached already!"));
        }
        self.values.push(item);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<(), String>{
        if self.values.len() == 0 {
            return Err(String::from("Queue is empty!"));
        }
        self.values = self.values[0..self.values.len() - 1].to_vec();
        Ok(())
    }

    pub fn peek(&self) -> Result<&T, String> {
        if self.values.len() == 0 {
            return Err(String::from("Queue is empty!"));
        }

        Ok(&self.values[self.values.len() - 1])
    }

    pub fn is_null(&self) -> bool {
        self.values.len() == 0
    }

    pub fn is_full(&self) -> bool{
        self.max_length == self.values.len() as u32
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}