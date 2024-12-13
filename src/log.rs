use crate::blinkie::{Blinker, BlinkerController, ConcreteBlinker};
use serde::Serialize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ObservationQueueFront<T: Serialize + Send> {
    tx: mpsc::Sender<Arc<Mutex<T>>>,
}

impl<T: Serialize + Send + 'static> ObservationQueueFront<T> {
    pub fn new(blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>) -> ObservationQueueFront<T> {
        let (tx, rx) = mpsc::channel();
        start_backend(rx, blinker);
        ObservationQueueFront { tx }
    }

    pub fn submit(&mut self, observation: T) {
        match self.tx.send(Arc::new(Mutex::new(observation))) {
            Err(mpsc::SendError(_)) => {
                panic!("Back-end (network subsystem) is disconnected, can't continue.");
            }
            _ => {}
        };
    }
}

pub struct ObservationQueueBack<T>
where
    T: Serialize + Send,
{
    queue: Vec<Arc<Mutex<T>>>,
    have_clock_init: bool,
    rx: mpsc::Receiver<Arc<Mutex<T>>>,
    blinker_controller: Box<dyn BlinkerController>,
}

fn start_backend<T: Serialize + Send + 'static>(
    rx: mpsc::Receiver<Arc<Mutex<T>>>,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>,
) -> () {
    thread::spawn(|| run_backend(rx, concrete_blinker));
}

fn run_backend<T: Serialize + Send>(
    rx_item: mpsc::Receiver<Arc<Mutex<T>>>,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>,
) -> () {
    let queue = Vec::<Arc<Mutex<T>>>::new();
    let rx = rx_item;
    let blinker_controller = Blinker::new(concrete_blinker);
    let mut oqb = ObservationQueueBack::<T> {
        rx,
        blinker_controller,
        queue,
        have_clock_init: false,
    };
    loop {
        match oqb.rx.recv_timeout(std::time::Duration::from_millis(
            oqb.blinker_controller.wait_ms(),
        )) {
            Ok(msg) => {
                oqb.add_newest(msg);
                oqb.blinker_controller.start_busy();
                if oqb.file_observations() {
                    oqb.blinker_controller.start_success();
                } else {
                    oqb.blinker_controller.start_trouble();
                };
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                oqb.blinker_controller.next();
            }
            _ => {
                println!("Error in backend rx. Has front-end disconnected?");
                break;
            }
        }
    }
}

impl<T: Serialize + Send> ObservationQueueBack<T> {
    fn add_newest(&mut self, observation: Arc<Mutex<T>>) {
        if !self.have_clock_init {
            self.empty();
        }
        self.queue.push(observation);
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

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Thing {
        u: i32,
    }

    #[test]
    fn queue_len_is_one_without_clock() {
        let mut q: ObservationQueueBack<Thing> = ObservationQueueBack::new();
        q.add_newest(Thing { u: 1 });
        q.add_newest(Thing { u: 2 });
        assert_eq!(q.queue.len(), 1);
    }

    #[test]
    fn queue_can_grow_with_clock() {
        let mut q: ObservationQueueBack<Thing> = ObservationQueueBack::new();
        q.have_clock_init = true;
        q.add_newest(Thing { u: 1 });
        q.add_newest(Thing { u: 2 });
        assert_eq!(q.queue.len(), 2);
    }
}
*/
