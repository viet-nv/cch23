use axum::{extract::Path, routing::get, Router};

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*param", get(xor_pow_3));

    Ok(router.into())
}
