use chrono::Local;

pub fn get_time() -> String {
    let now = Local::now();
    let time = now.format("%H:%M:%S");

    time.to_string()
}