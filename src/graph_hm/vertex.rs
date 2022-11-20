use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Drop;
use std::rc::Rc;

#[derive(Debug)]
pub struct Vertex<T> {
    pub value: Option<T>,
    pub edge: RefCell<Vec<*mut Vertex<T>>>,
}

impl<T> Drop for Vertex<T> {
    fn drop(&mut self) {}
}

impl<T: PartialEq + Clone + std::fmt::Debug> Vertex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            edge: RefCell::new(vec![]),
        }
    }
    pub fn new_default() -> Self {
        Self {
            value: None,
            edge: RefCell::new(vec![]),
        }
    }
    pub fn push_vertex(&mut self, ptr: *mut Vertex<T>) {
        self.edge.borrow_mut().push(ptr);
    }
    pub fn insert_value(&mut self, value: T) -> &mut Vertex<T> {
        self.value = Some(value);
        self
    }
    pub fn pop(&self, ptr: *mut Vertex<T>) {
        let rc = Rc::new(&self);
        let rc_find_id = Rc::clone(&rc);
        let rc_pop_vertex = Rc::clone(&rc);

        let id = {
            rc_find_id
                .edge
                .borrow()
                .iter()
                .position(|some_ptr| *some_ptr == ptr)
        };
        match id {
            Some(id) => {
                let result = rc_pop_vertex.edge.borrow_mut().remove(id);
                drop(result);
            }
            None => {}
        }
    }
    pub fn bfs(&self, value: T, vec_deque: &mut VecDeque<&Vertex<T>>) {
        println!("{:?}", self.value);
        if *self.value.as_ref().unwrap() == value {
            println!("Find");
            while let Some(vertex) = vec_deque.pop_front() {
                drop(vertex);
            }
        } 
        else {
            self.edge.borrow().iter().for_each(|ptr| {
                let vertex = unsafe { &**ptr };
                vec_deque.push_back(vertex);
            });

            while let Some(vertex) = vec_deque.pop_front() {
                vertex.bfs(value.clone(), vec_deque);
            }
        }
    }
}
