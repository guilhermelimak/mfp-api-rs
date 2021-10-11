use select::document::Document;
use select::predicate::Class;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;
mod helpers;
use helpers::selector_helpers;

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse<'a> {
    calories: &'a str,
    protein: &'a str,
    carbs: &'a str,
    fat: &'a str,
}

#[tokio::main]
async fn main() {
    async fn handle(name: String) -> Result<impl warp::Reply, Infallible> {
        let html_text = reqwest::get(format!("https://www.myfitnesspal.com/food/diary/{}", name))
            .await
            .expect("Error while getting request")
            .text()
            .await
            .expect("Error while getting request text");

        let doc = Document::from(html_text.as_str());

        let mut remaining = doc
            .find(Class("remaining"))
            .next()
            .expect("Couldn't find .remaining on page")
            .children()
            .filter(selector_helpers::is_not_text);

        remaining.next();

        let calories = selector_helpers::get_calories(remaining.next().unwrap());
        let carbs = selector_helpers::get_macro_value(remaining.next().unwrap());
        let fat = selector_helpers::get_macro_value(remaining.next().unwrap());
        let protein = selector_helpers::get_macro_value(remaining.next().unwrap());

        let res = ServerResponse {
            calories,
            carbs,
            protein,
            fat,
        };
        Ok(warp::reply::json(&res))
    }

    let hello = warp::path("mfp").and(warp::path::param()).and_then(handle);

    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
