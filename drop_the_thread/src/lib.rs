use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }

    pub fn new_thread(&self, cmd: String) -> (usize, Thread) {
        let mut states = self.states.borrow_mut();
        let pid = states.len();
        states.push(false); // not dropped

        let thread = Thread {
            pid,
            cmd,
            parent: self,
        };

        (pid, thread)
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap_or(&true)
    }

    pub fn drop_thread(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(state) = states.get_mut(id) {
            if *state {
                panic!("{} is already dropped", id);
            } else {
                *state = true;
                self.drops.set(self.drops.get() + 1);
            }
        } else {
            panic!("Invalid thread id: {}", id);
        }
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(pid: usize, cmd: String, t: &'a ThreadPool) -> Self {
        Thread {
            pid,
            cmd,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}



