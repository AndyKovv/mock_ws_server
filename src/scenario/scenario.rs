extern crate ws;

use std::sync::{Mutex, Arc};
use self::ws::{Sender, Message};


/// Scenario struct use for run secenaroion in ws code
pub struct Scenario {
    run_func: Arc<Mutex<Box<Fn(&Sender, &Message) + Send + 'static>>>
}

pub trait TestScenario {
    fn run_scenario(&self, output: &Sender, message: &Message);
}

/// Run scenario with run func scenario
impl TestScenario for Scenario {
    fn run_scenario(&self, output: &Sender,  message: &Message) {
        let f = self.run_func.lock().unwrap();
        (*f)(&output, &message);
    }
}
