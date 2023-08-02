use std::{cell::RefCell, rc::Rc};

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

pub struct OutputListener<T> {
    listeners: Vec<Rc<OutputTracker<T>>>,
}

impl<T> OutputListener<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        OutputListener { listeners: vec![] }
    }

    pub fn track(&self, data: &T) {
        for listener in self.listeners.iter() {
            listener.add(data.clone());
        }
    }

    pub fn create_tracker(&mut self) -> Rc<OutputTracker<T>> {
        let tracker = Rc::new(OutputTracker::new());
        self.listeners.push(Rc::clone(&tracker));

        tracker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_emitter_emit() {
        let mut output_listener = OutputListener::new();
        let tracker = output_listener.create_tracker();

        output_listener.track(&String::from("test"));

        assert_eq!(tracker.data(), vec![String::from("test")]);
        assert_eq!(tracker.flush(), vec![String::from("test")]);
        assert_eq!(tracker.data(), Vec::<String>::new());
    }
}
