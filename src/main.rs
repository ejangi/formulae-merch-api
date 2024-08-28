#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;
use parse::parse::get_products_from_html;
use rocket::serde::json::{Json, Value, json};
pub mod types;
pub mod wget;
pub mod parse;
use crate::types::Product;
use crate::wget::wget::fetch_url;

#[get("/")]
async fn index() -> Json<Vec<Product>> {
    let mut products:Vec<Product> = Vec::new();
    let store_url: String = "https://store.fiaformulae.com/au/en/search/?pmin=0%2c00&srule=new-arrivals&start=0&sz=36".to_string();
    match fetch_url(store_url.clone()).await {
        Ok(response) => {
            let parsed_products = get_products_from_html(response.to_string());
            products.extend(parsed_products);
        },
        Err(_) => {
            eprintln!("Unable to fetch store URL: {:?}", store_url.clone());
        }
    }
    
    Json(products)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}