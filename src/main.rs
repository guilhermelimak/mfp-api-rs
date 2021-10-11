use select::document::Document;
use select::node::{Children, Node};
use select::predicate::{Child, Class, Element, Name, Text};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

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

        let is_not_text = |node: &Node| node.name().is_some();

        let mut remaining = doc
            .find(Class("remaining"))
            .next()
            .expect("Couldn't find .remaining on page")
            .children()
            .filter(is_not_text);

        remaining.next();

        let calories = remaining
            .next()
            .unwrap()
            .children()
            .next()
            .unwrap()
            .as_text()
            .unwrap();
        let carbs = remaining
            .next()
            .unwrap()
            .children()
            .filter(is_not_text)
            .next()
            .unwrap()
            .children()
            .next()
            .unwrap()
            .as_text()
            .unwrap();
        let fat = remaining
            .next()
            .unwrap()
            .children()
            .filter(is_not_text)
            .next()
            .unwrap()
            .children()
            .next()
            .unwrap()
            .as_text()
            .unwrap();
        let protein = remaining
            .next()
            .unwrap()
            .children()
            .filter(is_not_text)
            .next()
            .unwrap()
            .children()
            .next()
            .unwrap()
            .as_text()
            .unwrap();

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
