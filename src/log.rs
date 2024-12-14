use crate::blinkie::{Blinker, BlinkerController, ConcreteBlinker};
use crate::poster::Poster;
use serde::Serialize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ObservationQueueFront<T: Serialize + Send> {
    tx: mpsc::Sender<Arc<Mutex<T>>>,
    join_handle: thread::JoinHandle<i32>,
}

impl<T: Serialize + Send + 'static> ObservationQueueFront<T> {
    pub fn new(
        concrete_poster: Arc<Mutex<dyn Poster<T> + Send + Sync>>,
        concrete_blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>,
    ) -> ObservationQueueFront<T> {
        let (tx, rx) = mpsc::channel();
        let join_handle = start_backend(rx, concrete_poster, concrete_blinker);
        ObservationQueueFront { tx, join_handle }
    }

    pub fn submit(&mut self, observation: T) {
        match self.tx.send(Arc::new(Mutex::new(observation))) {
            Err(mpsc::SendError(_)) => {
                panic!("Back-end (network subsystem) is disconnected, can't continue.");
            }
            _ => {}
        };
    }

    pub fn end_when_idle(self) {
        let tx = self.tx; // move out from reference
        drop(tx);
        let j = self.join_handle;
        match j.join() {
            Ok(v) => {
                println!("Ending program, backend join() value: {v}")
            }
            Err(e) => {
                println!("Ending program, join() error: {e:#?}")
            }
        }
    }
}

pub struct ObservationQueueBack<T>
where
    T: Serialize + Send,
{
    queue: Vec<Arc<Mutex<T>>>,
    have_clock_init: bool,
    rx: mpsc::Receiver<Arc<Mutex<T>>>,
    concrete_poster: Arc<Mutex<dyn Poster<T>>>,
    blinker_controller: Box<dyn BlinkerController>,
}

fn start_backend<T: Serialize + Send + 'static>(
    rx: mpsc::Receiver<Arc<Mutex<T>>>,
    concrete_poster: Arc<Mutex<dyn Poster<T> + Send + Sync>>,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>,
) -> thread::JoinHandle<i32> {
    thread::spawn(|| run_backend(rx, concrete_poster, concrete_blinker))
}

fn run_backend<T: Serialize + Send>(
    rx_item: mpsc::Receiver<Arc<Mutex<T>>>,
    concrete_poster: Arc<Mutex<dyn Poster<T> + Send + Sync>>,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker + Send + Sync>>,
) -> i32 {
    let queue = Vec::<Arc<Mutex<T>>>::new();
    let rx = rx_item;
    let blinker_controller = Blinker::new(concrete_blinker);
    let mut oqb = ObservationQueueBack::<T> {
        rx,
        concrete_poster,
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
                oqb.blinker_controller = oqb.blinker_controller.start_busy();
                if oqb.file_observations() {
                    oqb.blinker_controller = oqb.blinker_controller.start_success();
                } else {
                    oqb.blinker_controller = oqb.blinker_controller.start_trouble();
                };
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                oqb.blinker_controller.next();
            }
            _ => {
                println!("Has front-end disconnected?");
                break;
            }
        }
    }
    0
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

    fn file_observations(&mut self) -> bool {
        let success = self
            .concrete_poster
            .lock()
            .unwrap()
            .post(&self.queue[0].lock().unwrap()); // too dumb, just sends one! TODO
        if success {
            self.queue.clear()
        }
        success
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
