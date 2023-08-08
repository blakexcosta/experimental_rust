mod house; // define house module
use crate::house::House; // import house module
mod url; // define misc module
use crate::url::*; // import say_hello

fn main() {
    let house1 = House { doors: 2, windows: 8, address: "21 South Main Street, Chicago, IL 60601".to_string(), color: "Blue".to_string() };
    let mut house2 = House::new(1, 4, "43 West 2nd Steet, Nashville, TN 37203".to_string(), "Red".to_string());
    house2.change_color("Purple".to_string());

    // Convert the Point to a JSON string.
    // Serialization = from a struct -> a JSON string
    let serialized = serde_json::to_string(&house1).unwrap();
    let serialized2 = serde_json::to_string(&house2).unwrap();

    // Prints serialized information
    println!("serialized = {}", serialized);
    println!("serialized2 = {}", serialized2);

    // Convert the JSON string back to a Point.
    // Deserialization = from a JSON string -> a struct
    let deserialized: House = serde_json::from_str(&serialized).unwrap();
    let deserialized2: House = serde_json::from_str(&serialized2).unwrap();

    // Prints deserialized information
    println!("deserialized = {:?}", deserialized);
    println!("deserialized2 = {:?}", deserialized2);


    // Same example as above but just with a url instead
    let mut new_url = Url::new("https://google.com".to_string());
    let serialized_url = serde_json::to_string(&new_url).unwrap(); // serializing a  Url -> string

    println!("serialized_url = {}", serialized_url);

    let deserialized_url: Url = serde_json::from_str(&serialized_url).unwrap(); // deserializing a string -> Url
    println!("deserialized_url = {:?}", deserialized_url);
}
