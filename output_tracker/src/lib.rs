use std::cell::RefCell;

pub struct OutputTracker<T> {
    output: RefCell<Vec<T>>,
}

impl<T> OutputTracker<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        OutputTracker {
            output: RefCell::new(vec![]),
        }
    }

    pub fn add(&self, output: T) {
        self.output.borrow_mut().push(output);
    }

    pub fn data(&self) -> Vec<T> {
        self.output.borrow().clone()
    }

    pub fn flush(&self) -> Vec<T> {
        let output = self.output.borrow().clone();

        self.output.borrow_mut().clear();

        output
    }
}

pub struct OutputListener<'a, T> {
    listeners: Vec<&'a OutputTracker<T>>,
}

impl<'a, T> OutputListener<'a, T>
where
    T: Clone,
{
    pub fn new() -> Self {
        OutputListener { listeners: vec![] }
    }

    pub fn track(&mut self, data: T) {
        for listener in self.listeners.iter() {
            listener.add(data.clone());
        }
    }

    pub fn add_tracker(&mut self, tracker: &'a OutputTracker<T>) {
        self.listeners.push(tracker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_emitter_emit() {
        let tracker = OutputTracker::new();
        let mut output_listener = OutputListener::new();
        output_listener.add_tracker(&tracker);

        output_listener.track(String::from("test"));

        assert_eq!(tracker.data(), vec![String::from("test")]);
        assert_eq!(tracker.flush(), vec![String::from("test")]);
        assert_eq!(tracker.data(), Vec::<String>::new());
    }
}
