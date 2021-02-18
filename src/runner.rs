pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Runner {}
    }

    pub fn run(&self) {
        println!("[runner]: run")
    }

    pub fn complete_task(&self) {
        println!("[runner]: complete task")
    }
}
