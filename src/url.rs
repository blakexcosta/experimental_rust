use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Url{
    pub url: String
}

impl Url{
    pub fn new(url: String) -> Self{
        return Url{
            url
        }
    }
}