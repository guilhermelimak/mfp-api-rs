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

#[derive(Serialize, Deserialize, Debug)]
struct ServerError<'a> {
    error: &'a str,
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

        let mut remaining = match doc.find(Class("remaining")).next() {
            Some(r) => r.children().filter(selector_helpers::is_not_text),
            None => {
                return Ok(warp::reply::json(&ServerError {
                    error: "Error while getting .remaining element. Maybe the username is wrong?",
                }));
            }
        };

        remaining.next();

        let calories = match selector_helpers::get_calories(remaining.next().unwrap()) {
            Some(c) => c,
            None => {
                return Ok(warp::reply::json(&ServerError {
                    error: "Error while getting calories",
                }));
            }
        };

        let carbs = match selector_helpers::get_macro_value(remaining.next().unwrap()) {
            Some(c) => c,
            None => {
                return Ok(warp::reply::json(&ServerError {
                    error: "Error while getting carbs",
                }));
            }
        };

        let fat = match selector_helpers::get_macro_value(remaining.next().unwrap()) {
            Some(c) => c,
            None => {
                return Ok(warp::reply::json(&ServerError {
                    error: "Error while getting fat",
                }));
            }
        };

        let protein = match selector_helpers::get_macro_value(remaining.next().unwrap()) {
            Some(c) => c,
            None => {
                return Ok(warp::reply::json(&ServerError {
                    error: "Error while getting protein",
                }));
            }
        };

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
