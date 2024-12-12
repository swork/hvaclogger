use crate::hvac::Observation;
use serde::Serialize;

pub struct ObservationQueue<T: Observation + Serialize> {
    queue: Vec<T>,
    have_clock_init: bool,
}

impl<T: Observation + Serialize> ObservationQueue<T> {
    pub fn new() -> ObservationQueue<T> {
        ObservationQueue {
            queue: Vec::new(),
            have_clock_init: false,
        }
    }

    pub fn submit(&mut self, observation: T) -> bool {
        self.add_newest(observation);
        self.file_observations()
    }

    fn add_newest(&mut self, observation: T) -> usize {
        if !self.have_clock_init {
            self.empty();
        }
        self.queue.push(observation);
        self.queue.len()
    }

    fn empty(&mut self) {
        while self.queue.len() > 0 {
            self.queue.pop();
        }
    }

    fn file_observations(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Thing {
        u: i32,
    }
    impl Observation for Thing {}

    #[test]
    fn queue_len_is_one_without_clock() {
        let mut q: ObservationQueue<Thing> = ObservationQueue::new();
        q.add_newest(Thing { u: 1 });
        q.add_newest(Thing { u: 2 });
        assert_eq!(q.queue.len(), 1);
    }

    #[test]
    fn queue_can_grow_with_clock() {
        let mut q: ObservationQueue<Thing> = ObservationQueue::new();
        q.have_clock_init = true;
        q.add_newest(Thing { u: 1 });
        q.add_newest(Thing { u: 2 });
        assert_eq!(q.queue.len(), 2);
    }
}
