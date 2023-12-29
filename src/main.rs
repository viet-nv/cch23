use axum::{
    extract::Json,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn xor_pow_3(Path(param): Path<String>) -> String {
    let num = param
        .split('/')
        .map(|val| val.parse().unwrap())
        .reduce(|a, b| a ^ b);
    num.unwrap_or(1_i64).pow(3).to_string()
}

#[derive(Deserialize)]
struct User1 {
    strength: i64,
}

async fn strength(Json(payload): Json<Vec<User1>>) -> String {
    let mut res = 0;
    for user in payload {
        res += user.strength;
    }
    res.to_string()
}

#[derive(Deserialize, Clone)]
struct User {
    name: String,
    strength: i64,
    speed: f64,
    height: i64,
    snow_magic_power: i64,
    favorite_food: String,
    antler_width: i64,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u64,
}

#[derive(Serialize)]
struct Response {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(Json(users): Json<Vec<User>>) -> Json<Response> {
    let fastest = users
        .iter()
        .max_by(|x, y| x.speed.partial_cmp(&y.speed).unwrap())
        .unwrap();
    let tallest = users
        .iter()
        .max_by(|x, y| x.height.partial_cmp(&y.height).unwrap())
        .unwrap();
    let magician = users
        .iter()
        .max_by(|x, y| x.snow_magic_power.partial_cmp(&y.snow_magic_power).unwrap())
        .unwrap();
    let consumer = users
        .iter()
        .max_by(|x, y| {
            x.candies_eaten_yesterday
                .partial_cmp(&y.candies_eaten_yesterday)
                .unwrap()
        })
        .unwrap();

    Json(Response {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route(
            "/-1/error",
            get(|| async { StatusCode::INTERNAL_SERVER_ERROR }),
        )
        .route("/1/*param", get(xor_pow_3))
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest));

    Ok(router.into())
}
