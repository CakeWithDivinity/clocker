use std::{time::SystemTime, cell::RefCell, rc::Rc};

pub trait Clock {
    fn current_time(&self) -> SystemTime;
}

struct SystemClock; 

impl Clock for SystemClock {
    fn current_time(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct MockClock {
    pub now: Rc<RefCell<SystemTime>>,
}

impl Clock for MockClock {
    fn current_time(&self) -> SystemTime {
        *self.now.borrow() 
    }
}
