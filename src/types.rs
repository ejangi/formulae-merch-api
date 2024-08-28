use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub sku: String,
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub brand: String,
    pub price: f32
}

impl Product {
    pub fn new(sku: String, title: String, url: String, image_url: String, brand: String, price: f32) -> Self {
        Product {
            sku,
            title,
            url,
            image_url,
            brand,
            price
        }
    }

    pub fn fields_list() -> Vec<String> {
        let fields = std::any::type_name::<Product>().to_string();
        return fields.split(">::").map(|f| f.to_string()).collect();
    }
}