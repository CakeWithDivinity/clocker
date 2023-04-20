use crate::clock::Clock;
use std::{time, cell::RefCell};

pub struct TimeTracker<'a, T: Clock> {
    items: Vec<TimeTrackerItem<'a, T>>,
}

pub struct TimeTrackerItem<'a, T: Clock> {
    entries: RefCell<Vec<TimeTrackerEntry<'a, T>>>,
    clock: T,
}

impl<'a, T: Clock> TimeTrackerItem<'a, T> {
    fn new(clock: T) -> Self {
        Self { entries: RefCell::new(vec![]), clock }
    }

    fn track(&'a self) {
       self.entries.borrow_mut().push(TimeTrackerEntry::new(&self.clock));
    }
}

pub struct TimeTrackerEntry<'a, T: Clock> {
    start: time::SystemTime,
    end: Option<time::SystemTime>,
    clock: &'a T,
}

impl<'a, T: Clock> TimeTrackerEntry<'a, T> {
    fn new(clock: &'a T) -> Self {
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
    use std::{cell::RefCell, rc::Rc, time::SystemTime};

    use crate::clock::MockClock;

    use super::{TimeTrackerEntry, TimeTrackerItem};

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
        entry.end_tracking();

        assert_eq!(entry.end, Some(*current_time.borrow()));
    }

    #[test]
    fn item_creates_entry_when_tracked() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };
        
        let item = TimeTrackerItem::new(mock_clock);
        item.track();

        assert_eq!(item.entries.borrow()[0].start, *current_time.borrow());
    }
}
