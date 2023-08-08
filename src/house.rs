use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct House {
    pub doors: i32,
    pub windows: i32,
    pub address: String,
    pub color: String,
}

impl House {
    pub fn new(doors: i32, windows: i32, address: String, color: String) -> Self {
        return House {
            doors,
            windows,
            address,
            color,
        }
    }

    pub fn change_color(&mut self, new_color: String) {
        self.color = new_color;
    }

}