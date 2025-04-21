trait Logger {
    fn log(&self, message: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[Console]: {}", message);
    }
}

pub struct FileLogger {
    file_path: String,
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        // Here we would write to a file, but for simplicity, we'll just print to console
        println!("[File]: {} - {}", self.file_path, message);
    }
}

// Static dispatch - generic function
fn log_status<T: Logger>(logger: &T, status: &str) {
    logger.log(format!("Status: {}", status).as_str());
}

pub fn demonstrate_static_dispatch() {
    let console_logger = ConsoleLogger;
    let file_logger = FileLogger {
        file_path: String::from("log.txt"),
    };

    log_status(&console_logger, "All systems operational");
    log_status(&file_logger, "All systems operational");
}

// Dynamic dispatch - trait object
fn log_status_dynamic(logger: &dyn Logger, status: &str) {
    logger.log(format!("Status: {}", status).as_str());
}

pub fn demonstrate_dynamic_dispatch() {
    let console_logger = ConsoleLogger;
    let file_logger = FileLogger {
        file_path: String::from("log.txt"),
    };

    log_status_dynamic(&console_logger, "All systems operational");
    log_status_dynamic(&file_logger, "All systems operational");
}
