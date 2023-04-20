use crate::clock::Clock;
use std::time;

pub struct TimeTracker<T: Clock> {
    items: Vec<TimeTrackerItem<T>>,
}

pub struct TimeTrackerItem<T: Clock> {
    entries: Vec<TimeTrackerEntry<T>>,
}

pub struct TimeTrackerEntry<T: Clock> {
    start: time::SystemTime,
    end: Option<time::SystemTime>,
    clock: T,
}

impl<T: Clock> TimeTrackerEntry<T> {
    fn new(clock: T) -> Self {
        Self {
            start: clock.current_time(),
            end: None,
            clock,
        }
    }

    fn end_tracking(&mut self) {
        self.end = Some(self.clock.current_time());
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, time::SystemTime, rc::Rc};

    use crate::clock::MockClock;

    use super::TimeTrackerEntry;


    #[test]
    fn entry_starts_with_current_time() {
        let current_time = SystemTime::now();

        let mock_clock = MockClock {
            now: Rc::new(RefCell::new(current_time))
        };

        let entry = TimeTrackerEntry::new(mock_clock);

        assert_eq!(entry.start, current_time);
    }

    #[test]
    fn entry_ends_tracking_with_current_time() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time)
        };

        let mut entry = TimeTrackerEntry::new(mock_clock);
        *current_time.borrow_mut() = SystemTime::now();
        entry.end_tracking();

        assert_eq!(entry.end, Some(*current_time.borrow()));
    }
}
