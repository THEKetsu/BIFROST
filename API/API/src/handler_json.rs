/* fichier qui permet de gerer le json  */

use serde_json::{Number, Value};
use std::include_str;

// Marche pas bien mais en gros extraire un fichier json et faire des modifs dedans
pub async fn collect(){
    let mut companies = {
        let text = std::fs::read_to_string(&"id.json").unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let N = companies["companies"].as_array().unwrap().len();
    for index in 0..N{
        if let Value::Number(n) = &companies["companies"][index]["number"] {
            // Double the quantity for each element in 'missy_food_schedule'
            companies["companies"][index]["number"] =
                Value::Number(Number::from_f64(n.as_f64().unwrap() * 2.).unwrap());
        }
    }
      std::fs::write("id2.json",serde_json::to_string_pretty(&companies).unwrap()).unwrap();
}


pub async fn get_demo_json() -> axum::response::Json<&'static str>{
    include_str!("id.json").into()
}
/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(axum::extract::Json(data): axum::extract::Json<serde_json::Value>) -> String{
    format!("Put demo JSON data: {:?}", data)
}