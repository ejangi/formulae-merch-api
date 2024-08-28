#[cfg(test)]
mod test {
    use crate::rocket;
    use crate::types::Product;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};
    //use rocket::serde::{Serialize, Deserialize};
    use rocket::serde::json;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).unwrap();
        let res = client.get("/").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        let res_string = res.into_string().unwrap_or("".to_string());
        let res_first2: String = res_string.chars().take(2).collect();
        assert_eq!("[{".to_string(), res_first2);
        let products: Vec<Product> = json::from_str(&res_string).unwrap();
        // println!("Products: {}", products.len());
        assert_ne!(0, products.len());
        // println!("{}", res_string);
    }
}