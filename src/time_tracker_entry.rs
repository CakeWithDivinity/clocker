use std::time;

use crate::clock::Clock;

pub struct TimeTrackerEntry {
    pub(crate) start: time::SystemTime,
    pub(crate) end: Option<time::SystemTime>,
}

impl TimeTrackerEntry {
    pub fn new(clock: &dyn Clock) -> Self {
        Self {
            start: clock.current_time(),
            end: None,
        }
    }

    pub fn end_tracking(&mut self, clock: &dyn Clock) {
        self.end = Some(clock.current_time());
    }

    pub fn has_ended(&self) -> bool {
        self.end.is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::{time::SystemTime, cell::RefCell, rc::Rc};

    use crate::{clock::MockClock, time_tracker_entry::TimeTrackerEntry};

    #[test]
    fn entry_starts_with_current_time() {
        let current_time = SystemTime::now();

        let mock_clock = MockClock {
            now: Rc::new(RefCell::new(current_time)),
        };

        let entry = TimeTrackerEntry::new(&mock_clock);

        assert_eq!(entry.start, current_time);
    }

    #[test]
    fn entry_ends_tracking_with_current_time() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut entry = TimeTrackerEntry::new(&mock_clock);
        *current_time.borrow_mut() = SystemTime::now();
        entry.end_tracking(&mock_clock);

        assert_eq!(entry.end, Some(*current_time.borrow()));
    }
}
