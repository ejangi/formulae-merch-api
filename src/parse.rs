pub mod parse {
    use scraper::{Html, Selector};
    use crate::types::Product;
    use regex::Regex;

    const STORE_BASENAME: &str = "https://store.fiaformulae.com";

    pub fn get_products_from_html(body: String) -> Vec<Product> {
        let mut products:Vec<Product> = Vec::new();
        let product_selector = Selector::parse(".product-grid > .search-results-tile > .product > .product-tile").unwrap();
        let document = Html::parse_document(&body);
        let product_elements = document.select(&product_selector);
        let sku_regex = Regex::new(r#"\"sku\":\"([0-9]+)\""#).unwrap();
        let title_regex = Regex::new(r#"\"productName\":\"([^\"]+)\""#).unwrap();
        let url_selector = Selector::parse("a[href].product-tile-image").unwrap();
        let image_regex = Regex::new(r#"\"absUrl\":\"([^\"]+)\""#).unwrap();
        let brand_regex = Regex::new(r#"\"brand\":\"([^\"]+)\""#).unwrap();
        let price_regex = Regex::new(r#"\"value\":([0-9\.]+)"#).unwrap();

        for product_element in product_elements {
            let mut sku = String::from("");
            let mut title = String::from("");
            let mut url = String::from("");
            let mut image_url = String::from("");
            let mut brand = String::from("");
            let mut price = f32::from(0.0);

            let mut product_json = String::from("");

            
            if let Some(data) = product_element.value().attr("data-product") {
                product_json = data.to_string();
            }
            
            // println!("JSON: {:?}", product_json);
            
            if let Some(sku_matches) = sku_regex.captures(&product_json) {
                if let Some(sku_match) = sku_matches.get(1) {
                    sku = sku_match.as_str().to_string();
                }
            }

            if let Some(title_matches) = title_regex.captures(&product_json) {
                if let Some(title_match) = title_matches.get(1) {
                    title = title_match.as_str().to_string();
                }
            }

            if let Some(href) = product_element.select(&url_selector).next().unwrap().value().attr("href") {
                url = STORE_BASENAME.to_string() + &href.to_string();
            }

            if let Some(image_matches) = image_regex.captures(&product_json) {
                if let Some(image_match) = image_matches.get(1) {
                    image_url = image_match.as_str().to_string();
                }
            }

            if let Some(brand_matches) = brand_regex.captures(&product_json) {
                if let Some(brand_match) = brand_matches.get(1) {
                    brand = brand_match.as_str().to_string();
                }
            }

            if let Some(price_matches) = price_regex.captures(&product_json) {
                if let Some(price_match) = price_matches.get(1) {
                    match price_match.as_str().parse::<f32>() {
                        Ok(price_parsed) => {
                            price = price_parsed;
                        },
                        Err(_) => {}
                    }
                }
            }

            let product = Product::new(sku, title, url, image_url, brand, price);
            products.push(product);
        }

        return products;
    }

    #[tokio::test]
    pub async fn can_get_products() {
        use crate::wget::wget::fetch_url;

        let body = fetch_url("https://storage.googleapis.com/ejangi-public-files/formulae-merch-api-example.html".to_string()).await;
        let products = self::get_products_from_html(body.unwrap());

        assert!(products.len() > 0);

        for product in products {
            assert_ne!(0, product.sku.len());
            assert_ne!(0, product.title.len());
            assert_ne!(0, product.url.len());
            assert_ne!(0, product.image_url.len());
            assert_ne!(0, product.brand.len());
            assert_ne!(0.0, product.price);
            // println!("Price: {:?}", product.price);
        }
    }
}