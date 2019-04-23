use harsh::{Harsh, HarshBuilder};

pub struct Hasher {
    generator: Harsh
}

impl Hasher {
    pub fn new() -> Hasher {
        let generator = HarshBuilder::new().init().unwrap();
        Hasher { generator }
    }

    pub fn hash(&self, val: String) -> String {
        let val_int: u64 = val.parse().unwrap();
        let hashed = self.generator.encode(&[val_int]).unwrap();
        hashed
    }
}