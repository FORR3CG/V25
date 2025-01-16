#[derive(Debug)]
pub struct Bill {
    id: u32,
    tegund: String,
}

impl Bill {
    // associated function
    pub fn new(id: u32, tegund: &str) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
        }
    }
}

impl Bill {
    // method
    pub fn prenta(&self) {
        println!("{:?}", self)
    }

    // getter
    pub fn id(&self) -> u32 {
        self.id
    }

    // setter
    pub fn set_id(&mut self, nytt_id: u32) {
        self.id = nytt_id;
    }

    pub fn tegund(&self) -> &String {
        &self.tegund
    }
}
