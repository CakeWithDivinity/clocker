use crate::clock::Clock;
use std::time;

pub struct TimeTracker<T: Clock> {
    items: Vec<TimeTrackerItem>,
    clock: T,
}

impl<T: Clock> TimeTracker<T> {
    fn new(clock: T) -> Self {
        Self {
            items: vec![],
            clock,
        }
    }

    fn add_item(&mut self, label: String) {
        self.items.push(TimeTrackerItem::new(label));
    }
}

pub struct TimeTrackerItem {
    label: String,
    entries: Vec<TimeTrackerEntry>,
}

impl TimeTrackerItem {
    fn new(label: String) -> Self {
        Self {
            entries: vec![],
            label,
        }
    }

    fn track(&mut self, clock: &dyn Clock) {
        self.entries.push(TimeTrackerEntry::new(clock));
    }
}

pub struct TimeTrackerEntry {
    start: time::SystemTime,
    end: Option<time::SystemTime>,
}

impl TimeTrackerEntry {
    fn new(clock: &dyn Clock) -> Self {
        Self {
            start: clock.current_time(),
            end: None,
        }
    }

    fn end_tracking(&mut self, clock: &dyn Clock) {
        self.end = Some(clock.current_time());
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, time::SystemTime};

    use crate::clock::MockClock;

    use super::{TimeTracker, TimeTrackerEntry, TimeTrackerItem};

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

    #[test]
    fn item_creates_entry_when_tracked() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut item = TimeTrackerItem::new("TestItem".to_string());
        item.track(&mock_clock);

        assert_eq!(item.entries[0].start, *current_time.borrow());
    }

    #[test]
    fn tracker_can_add_items() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        tracker.add_item("TestItem".to_string());

        let item = &tracker.items[0];
        assert_eq!("TestItem", item.label);
    }
}
