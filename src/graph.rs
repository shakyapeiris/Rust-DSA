use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct Graph<T> {
    edges: HashMap<T, Vec<T>>
}

impl <T: Eq + Hash + Copy + Debug> Graph<T> {
    pub fn new() -> Graph<T>{
        return Graph { 
            edges: HashMap::new()
        }
    }

    pub fn route(&mut self, from: T, to: T) {
        match self.edges.get_mut(&from) {
            Some(item) => {
                item.push(to)
            },
            None => {
                self.edges.insert(from, vec![to]);
            }
        }
    }

    pub fn get_children(&mut self, key: T) -> Option<&Vec<T>>{
        self.edges.get(&key)
    }

    pub fn get_shortest_path_length(&self, from: &T, to: &T) -> i32 {

        if from == to {
            return 0
        }
        match self.edges.get(&from) {
            Some(arr) => {
                let mut vals = Vec::<i32>::new(); 

                for i in arr {
                    vals.push(self.get_shortest_path_length(i, to));
                }

                let mut m = &vals[0];

                for i in &vals {
                    if i < m {
                        m = i;
                    }
                }

                *m + 1
            },
            None => -1
        }
    }

    pub fn get_shortest_path (&self, from: &T, to: &T) -> Vec<T> {
        if from == to {
            return vec![*to];
        }

        match self.edges.get(&from) {
            Some(arr) => {
                let mut vals = Vec::<Vec<T>>::new(); 

                for i in arr {
                    vals.push(self.get_shortest_path(i, to));
                }

                let mut m = vals[0].len();
                let mut mv = vals[0].clone();

                for i in &vals {
                    let l = i.len();
                    if l < m {
                        m = l;
                        mv = i.clone();
                    }
                }

                mv.insert(0,*from);
                mv

            },
            None => Vec::<T>::new()
        }
    }
}