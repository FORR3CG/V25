use std::fmt::Display;

use crate::bill::Bill;

pub struct Bilasala {
    bilar: Vec<Bill>,
    id: u32,
}

impl Bilasala {
    pub fn new(upphafs_id: u32) -> Self {
        Self {
            bilar: Vec::new(),
            id: upphafs_id,
        }
    }

    fn next_id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    pub fn skra(&mut self, framleidandi: &str, gerd: &str, verd: u32) {
        let nytt_id = self.next_id();
        self.bilar.push(Bill::new(nytt_id, framleidandi, gerd, verd))
    }

    pub fn verdmaeti(&self) -> u64 {
        let mut heildar_verd = 0_u64;
        for bill in &self.bilar {
           heildar_verd += bill.verd() as u64; 
        }
        heildar_verd
    }
}

impl Display for Bilasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut texti = String::new();
        for bill in &self.bilar {
            texti.push_str(format!("{}\n", bill).as_str());
        }
        let medalverd = self.verdmaeti() as f32 / self.bilar.len() as f32;
        texti.push_str(format!("Heildar verðmæti: {}, meðalverð: {:.2}", 
                                       self.verdmaeti(), medalverd).as_str());  
        writeln!(f, "{}", texti)    
    }
}