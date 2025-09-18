#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker, 
        }
    }
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    // Add worker at the start of the list
    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    // Remove the first worker (last added)
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut head) = self.grade.take() {
            self.grade = head.next.take(); // move head to next
            Some(head.name)
        } else {
            None
        }
    }

    // Return the last added worker (head of the list)
    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|w| (w.name.clone(), w.role.clone()))
    }
}
