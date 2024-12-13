/*
Make a notifier to indicate the state of network activities - at least an LED that is off when all is well and quiet (blinking for 100ms every say 10s so we know it's still breathing), blinks lazily before the first successful network activity (to help distinguish config errors from operation errors), and blinks excitedly when a network transmission fails after at least one has succeeded.

A compleat implementation would be more general than this.

The network activity happens on a separate thread (rather than implement non-blocking semantics everywhere) so this stuff needs to run on the back-end thread. Seems that precludes using a trait object to hold the LED-twiddler implementation and passing it to the spawned back-end, since trait objects can't be Sync (rustc --explain E0321). That's why the blinker implementation is generic in the concrete implementation choice.

The weird design was inspired by RustBook section "Implementing an Object-Oriented Design Pattern", which turns out to be not particularly resilient. Particularly, it depends on "let" of a new variable every state change, can't store the state trait object in a structure directly because size isn't known; can't do compile-time checks promised in that doc section because trait object must have all slots implemented (for dynamic dispatch). So we end up with all the awkwardness and still have runtime dispatch of trait methods, so runtime failures possible in the calling sequence.
 */

// BlinkerController goes in a Box because needs DerefMut and used only in its thread.
// ConcreteBlinker goes in Arc<Mutex<>> because needs DerefMut and is moved across threads.

use std::sync::{Arc, Mutex};

const NOTYET_OFF: u64 = 1000;
const NOTYET_ON: u64 = 1000;
const SUCCESS_ON: u64 = 100;
const SUCCESS_OFF: u64 = 10_000;
const ERROR_ON: u64 = 500;
const ERROR_OFF: u64 = 500;

pub struct Blinker {}
impl Blinker {
    pub fn new(concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>) -> Box<dyn BlinkerController> {
        BlinkieTrouble::new(concrete_blinker, false)
    }
}

pub trait ConcreteBlinker {
    fn init(&mut self) -> ();
    fn toggle(&mut self, turn_on: Option<bool>) -> bool;
}

pub trait BlinkerController {
    fn start_busy(&self) -> Box<dyn BlinkerController> {
        panic!("Can't call BlinkerController::start_busy here");
    }

    fn start_trouble(&self) -> Box<dyn BlinkerController> {
        panic!("Can't call BlinkerController::start_trouble here");
    }

    fn start_success(&self) -> Box<dyn BlinkerController> {
        panic!("Can't call BlinkerController::start_success here");
    }

    fn next(&mut self) -> () {
        panic!("Can't call BlinkerController::next here");
    }
    fn wait_ms(&self) -> u64 {
        panic!("Can't call BlinkerController::wait_ms here");
    }
}

struct BlinkieTrouble {
    ever_succeeded: bool,
    wait_ms: u64,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>,
}

impl BlinkieTrouble {
    fn new(
        concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>,
        ever_succeeded: bool,
    ) -> Box<dyn BlinkerController> {
        concrete_blinker.lock().unwrap().toggle(Some(false));
        let mut wait_ms = NOTYET_OFF;
        if ever_succeeded {
            wait_ms = ERROR_OFF;
        }
        Box::new(BlinkieTrouble {
            ever_succeeded,
            wait_ms,
            concrete_blinker,
        })
    }
}

impl BlinkerController for BlinkieTrouble {
    fn start_busy(&self) -> Box<dyn BlinkerController> {
        BlinkieBusy::new(self.concrete_blinker.clone(), self.ever_succeeded)
    }

    fn next(&mut self) -> () {
        let was = self.concrete_blinker.lock().unwrap().toggle(None);
        let mut off_ms = NOTYET_OFF;
        let mut on_ms = NOTYET_ON;
        if self.ever_succeeded {
            off_ms = ERROR_OFF;
            on_ms = ERROR_ON;
        }
        match was {
            true => self.wait_ms = off_ms,
            _ => self.wait_ms = on_ms,
        };
    }

    fn wait_ms(&self) -> u64 {
        self.wait_ms
    }
}

struct BlinkieBusy {
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>,
    ever_succeeded: bool,
}

impl BlinkieBusy {
    fn new(
        concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>,
        ever_succeeded: bool,
    ) -> Box<dyn BlinkerController> {
        concrete_blinker.lock().unwrap().toggle(Some(true));
        Box::new(BlinkieBusy {
            concrete_blinker,
            ever_succeeded,
        })
    }
}

impl BlinkerController for BlinkieBusy {
    fn start_success(&self) -> Box<dyn BlinkerController> {
        BlinkieSuccess::new(self.concrete_blinker.clone())
    }

    fn start_trouble(&self) -> Box<dyn BlinkerController> {
        BlinkieTrouble::new(self.concrete_blinker.clone(), self.ever_succeeded)
    }

    // no wait_ms() or next() - just leave the light on while busy.
}

struct BlinkieSuccess {
    wait_ms: u64,
    concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>,
}

impl BlinkieSuccess {
    fn new(concrete_blinker: Arc<Mutex<dyn ConcreteBlinker>>) -> Box<dyn BlinkerController> {
        concrete_blinker.lock().unwrap().toggle(Some(false));
        let wait_ms = SUCCESS_OFF;
        Box::new(BlinkieSuccess {
            wait_ms,
            concrete_blinker,
        })
    }
}

impl BlinkerController for BlinkieSuccess {
    fn start_busy(&self) -> Box<dyn BlinkerController> {
        BlinkieBusy::new(self.concrete_blinker.clone(), true)
    }

    fn wait_ms(&self) -> u64 {
        self.wait_ms
    }

    fn next(&mut self) -> () {
        match self.concrete_blinker.lock().unwrap().toggle(None) {
            true => self.wait_ms = SUCCESS_OFF,
            _ => self.wait_ms = SUCCESS_ON,
        };
    }
}

pub struct ExampleConcreteBlinker {
    is_on: bool,
}

impl ExampleConcreteBlinker {
    pub fn new() -> ExampleConcreteBlinker {
        let mut cb = ExampleConcreteBlinker { is_on: false };
        cb.init();
        cb
    }
}

impl ConcreteBlinker for ExampleConcreteBlinker {
    fn init(&mut self) {
        self.toggle(Some(false));
        self.is_on = false;
    }

    fn toggle(&mut self, turn_on: Option<bool>) -> bool {
        let was = self.is_on;
        match turn_on {
            Some(turn_on) => {
                if turn_on && !self.is_on {
                    println!("(ON)");
                    self.is_on = true;
                } else if !turn_on && self.is_on {
                    println!("(off)");
                    self.is_on = false;
                }
            }
            _ => {
                if !self.is_on {
                    println!("(ON)");
                    self.is_on = true;
                } else {
                    println!("(off)");
                    self.is_on = false;
                }
            }
        };
        was
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        let concrete_blinker = Arc::new(Mutex::new(ExampleConcreteBlinker::new()));
        let mut b = Blinker::new(concrete_blinker);
        assert_eq!(b.wait_ms(), NOTYET_OFF);
        b.next();
        assert_eq!(b.wait_ms(), NOTYET_ON);
        b.next();
        assert_eq!(b.wait_ms(), NOTYET_OFF);

        let b = b.start_busy();

        let mut b = b.start_trouble();
        assert_eq!(b.wait_ms(), NOTYET_OFF);
        b.next();
        assert_eq!(b.wait_ms(), NOTYET_ON);
        b.next();
        assert_eq!(b.wait_ms(), NOTYET_OFF);
        b.next();
        assert_eq!(b.wait_ms(), NOTYET_ON);

        let b = b.start_busy();

        let mut b = b.start_success();
        assert_eq!(b.wait_ms(), SUCCESS_OFF);
        b.next();
        assert_eq!(b.wait_ms(), SUCCESS_ON);
        b.next();
        assert_eq!(b.wait_ms(), SUCCESS_OFF);
        b.next();
        assert_eq!(b.wait_ms(), SUCCESS_ON);

        let b = b.start_busy();

        let mut b = b.start_trouble();
        assert_eq!(b.wait_ms(), ERROR_OFF);
        b.next();
        assert_eq!(b.wait_ms(), ERROR_ON);
        b.next();
        assert_eq!(b.wait_ms(), ERROR_OFF);
    }

    /*
     * busy_no_next correctly won't compile for missing .next().
     *
    #[test]
    #[should_panic]
    fn busy_no_next() {
        let cb = Arc::<_>::new(ExampleConcreteBlinker::new());
        let b = Blinkie::<ExampleConcreteBlinker>::new(cb);
        let b = b.start_busy();
        b.next();
    }
     */

    /*
     * busy_no_wait correctly won't compile for missing .wait_ms().
     *
    #[test]
    #[should_panic]
    fn busy_no_wait() {
        let cb = ExampleConcreteBlinker::new();
        let b = Blinkie::new(cb);
        let b = b.start_busy();
        b.wait_ms();
    }
     */
}
