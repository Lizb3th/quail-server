

enum Level {
    error,
    warn,
    info,
}

enum LogValue {
    message(&str),
}

struct LogEntry {
    time: std::time::SystemTime,
    value: LogValue,
}

impl LogFactory {

}

struct Logger {
    buffer: [LogEntry],
}

impl logger {
    fn new() {

    }

    fn log(message: &Str) {
        bu
    }

    fn flush() {
        
    }
}