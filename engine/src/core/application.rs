use crate::core::logger::{
    initalize_logging, log_debug, log_error, log_fatal, log_info, log_trace, log_warning,
};

pub struct Application {
    is_running: bool,
    is_suspended: bool,
    last_time: f64,
}

impl Application {
    pub fn create() -> Application {
        // Start logging
        initalize_logging();

        // TODO: remove
        log_fatal("Fatal stuff".to_string());
        log_error("Error stuff".to_string());
        log_warning("Warning stuff".to_string());
        log_info("Info stuff".to_string());
        log_debug("Debug stuff".to_string());
        log_trace("Trace stuff".to_string());

        // Return app
        Application {
            is_running: false,
            is_suspended: false,
            last_time: 0.0,
        }
    }
    pub fn run(&mut self) {
        // Some game loop logic
        self.is_running = true;

        while self.is_running {
            self.last_time += 1.0;
            println!("{}", self.last_time);
            if self.last_time >= 20.0 {
                self.is_running = false;
            }
        }
    }
}
