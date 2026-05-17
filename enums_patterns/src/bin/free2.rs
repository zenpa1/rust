enum SystemState {
    Online,
    Offline,
    Error(String),
    Diagnostic(u32),
}

struct ShipComponent {
    name: String,
    state: SystemState
}

impl ShipComponent {
    fn new(name: String, state: SystemState) -> Self {
        Self {
            name,
            state,
        }
    }

    fn status_report(&self) {
        match &self.state {
            // match based on states
            SystemState::Online => println!("The system is now online."),
            SystemState::Offline => println!("The system is now offline."),
            SystemState::Error(err_str) => println!("The system encountered an error: {}", err_str),
            SystemState::Diagnostic(err_code) => println!("The system's error code is: {}", err_code),
        }
    }

    fn emergency_reboot(&self) {
        // if state is Error
        if let SystemState::Error(err_str) = &self.state { // why does this one need &
            println!("CRITICAL: Rebooting {} to resolve error: {err_str}", self.name);
        }
    }

    fn run_diagnostics(&self) {
        // the state MUST be diagnostic

        // if not diagnostic
        let SystemState::Diagnostic(err_code) = &self.state else { // but this one does not?? dark magic apparently
            // idiomatic practice is to use & when borrowing data using &self
            println!("Component is not in diagnostic mode.");
            return; // kick out
        };

        // if it is diagnostic
        println!("Running deep scan on {} using diagnostic code: {err_code}", self.name);
    }
}

fn main() {
    let s1: ShipComponent = ShipComponent::new(String::from("Ship 1"), SystemState::Online);
    let s2: ShipComponent = ShipComponent::new(String::from("Ship 2"), SystemState::Offline);
    let s3: ShipComponent = ShipComponent::new(String::from("Ship 3"), SystemState::Error("Unauthorized Access".to_string()));
    let s4: ShipComponent = ShipComponent::new(String::from("Ship 4"), SystemState::Diagnostic(21));

    s1.status_report();
    s1.emergency_reboot();
    s1.run_diagnostics();

    s2.status_report();
    s2.emergency_reboot();
    s2.run_diagnostics();

    s3.status_report();
    s3.emergency_reboot();
    s3.run_diagnostics();

    s4.status_report();
    s4.emergency_reboot();
    s4.run_diagnostics();
}