extern crate ws;

use std::thread;
use std::sync::{Mutex, Arc};
use scenario::scenario::{Scenario, TestScenario};
use self::ws::{Sender, Message, WebSocket};
use self::ws::deflate::DeflateHandler;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct ServerState {
    started: bool,
    stoped: bool,
    errors: Vec<String>,
    scenario_run: i32,
}

trait ArcServerState {
    fn new() -> Arc<Mutex<Self>>;
    fn set_start(&mut self);
    fn set_stop(self) -> Self;
    fn add_error(self, error: String);
    fn incr_scenario_run(&mut self);
    fn mock_server<S>(scenario: S) -> Arc<Mutex<Self>> where S: TestScenario + Send + 'static;
}

impl ArcServerState for ServerState {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self{started: false, stoped: false, errors: vec![], scenario_run: 0}))
    }

    /// Method should set start to bool value
    fn set_start(&mut self){
        self.started = true;
    }

    /// Method should stop attrebute
    fn set_stop(mut self) -> Self {
        self.stoped = true;
        self
    }

    /// Method should add error to error handler
    fn add_error(mut self, error: String) {
        self.errors.push(error);
    }

    /// Method should increment scenario run counter
    /// Scenario run counter is counter of running scenarios
    /// Each run scenario is increment to one counter
    fn incr_scenario_run(&mut self) {
        self.scenario_run += 1;
    }

    /// Runs mocked ws server with income scenario.
    fn mock_server<S>(scenario: S) -> Arc<Mutex<Self>> where S: TestScenario + Send + 'static {
    let state = ServerState::new();
    let cloned = Arc::clone(&state);
    let _res = thread::spawn( move || {

        let clone_two = Arc::clone(&state);
        {
            let mut res = clone_two.lock().unwrap();
            res.set_start();
        }

        let scenario_inst = Rc::new(RefCell::new(scenario));
        let  ws = WebSocket::new(|output: Sender| {
        let scenario_inst = scenario_inst.clone();

        let clone_three = Arc::clone(&state);
        let handler = move |msg: Message| {
                let scenario_inst = scenario_inst.clone();
                // Try implement check all rules are run
                scenario_inst.borrow().run_scenario(&output, &msg);
                let mut res = clone_three.lock().unwrap();
                res.incr_scenario_run();

                println!("Finish");
                Ok(())
            };
        DeflateHandler::new(handler)
        }).unwrap();
        ws.listen("127.0.0.1:3012").unwrap();
    });

    cloned
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_should_mock_response_from_default_scenario_from_server() {

    }
}
