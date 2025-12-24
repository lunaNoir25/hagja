mod data;

pub struct Hagja {
    id: &'static str,
}

impl Hagja {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id: id,
        }
    }

    pub fn info(&self, msg: &str) {
        println!("[{}] [INFO] [{}]: {}", data::get_time(), self.id, msg);
    }
}