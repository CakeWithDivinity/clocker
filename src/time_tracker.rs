use crate::{clock::Clock, time_tracker_item::TimeTrackerItem};

#[derive(Debug)]
pub enum TimeTrackerError {
    ItemNotFound,
}

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

    fn track_item(&mut self, item_label: String) -> Result<(), TimeTrackerError> {
        let item = match self.items.iter_mut().find(|item| item.label == item_label) {
            Some(item) => item,
            None => return Err(TimeTrackerError::ItemNotFound),
        };

        item.track(&self.clock);

        Ok(())
    }

    fn get_currently_tracked_item(&mut self) -> Option<&mut TimeTrackerItem> {
        self.items.iter_mut().find(|item| item.is_tracked())
    }

    fn stop_tracking_current_item(&mut self) {
        if let Some(item) = self.items.iter_mut().find(|item| item.is_tracked()) {
            item.end_tracking(&self.clock)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, time::SystemTime};

    use crate::clock::MockClock;

    use super::*;

    #[test]
    fn can_add_items() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        tracker.add_item("TestItem".to_string());

        let item = &tracker.items[0];
        assert_eq!("TestItem", item.label);
    }

    #[test]
    fn can_track_item() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        tracker.add_item("TestItem".to_string());

        assert!(tracker.track_item("TestItem".to_string()).is_ok());
        assert_eq!(tracker.items[0].entries[0].start, *current_time.borrow());
    }

    #[test]
    fn returns_err_when_tracking_non_existent_item() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        assert!(tracker.track_item("NonExistent".to_string()).is_err());
    }

    #[test]
    fn returns_currently_tracked_item() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        tracker.add_item("TestItem".to_string());
        tracker.add_item("TrackedTestItem".to_string());
        tracker.track_item("TrackedTestItem".to_string()).unwrap();

        assert_eq!(
            tracker.get_currently_tracked_item().unwrap().label,
            "TrackedTestItem"
        );
    }

    #[test]
    fn can_stop_tracking_current_item() {
        let current_time = Rc::new(RefCell::new(SystemTime::now()));
        let mock_clock = MockClock {
            now: Rc::clone(&current_time),
        };

        let mut tracker = TimeTracker::new(mock_clock);
        tracker.add_item("TestItem".to_string());
        tracker.track_item("TestItem".to_string()).unwrap();
        tracker.stop_tracking_current_item();

        assert_eq!(
            tracker.items[0].entries[0].end,
            Some(*current_time.borrow())
        );
        assert!(tracker.get_currently_tracked_item().is_none());
    }
}
