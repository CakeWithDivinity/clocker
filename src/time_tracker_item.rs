use crate::{clock::Clock, time_tracker_entry::TimeTrackerEntry};

pub struct TimeTrackerItem {
    pub label: String,
    pub(crate) entries: Vec<TimeTrackerEntry>,
}

impl TimeTrackerItem {
    pub fn new(label: String) -> Self {
        Self {
            entries: vec![],
            label,
        }
    }

    pub fn track(&mut self, clock: &dyn Clock) {
        self.entries.push(TimeTrackerEntry::new(clock));
    }

    pub fn is_tracked(&self) -> bool {
        match self.entries.last() {
            None => false,
            Some(last_entry) => !last_entry.has_ended(),
        }
    }

    pub fn end_tracking(&mut self, clock: &dyn Clock) {
        if let Some(entry) = self.entries.last_mut() {
            entry.end_tracking(clock);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, time::SystemTime};

    use super::TimeTrackerItem;
    use crate::clock::MockClock;

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
}
