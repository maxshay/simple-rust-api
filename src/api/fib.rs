use actix_web::{error, get, web::Json, web::Path, Responder, Result};
use derive_more::{Display, Error};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct FibRes {
    r#type: String,
    result: usize,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "type: {}, message: {}", r#type, message)]
pub struct MyError {
    r#type: String,
    message: String,
}

impl error::ResponseError for MyError {}

fn fibonacci_approx(n: usize) -> usize {
    let result = (1.0 + 5.0_f32.sqrt() / 2.0).powf(n as f32) / 5.0_f32.sqrt();
    result.floor() as usize
}

fn fibonacci(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let v = match n {
        0 | 1 => 1,
        _ => fibonacci(n - 2, memo) + fibonacci(n - 1, memo),
    };

    memo.insert(n, v);
    v
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/api/fib/{n}")]
pub async fn compute_fib(n: Path<usize>) -> Result<impl Responder, MyError> {
    let result: usize;
    let obj: FibRes;
    if *n > 90 {
        result = fibonacci_approx(*n);
        obj = FibRes {
            r#type: "approx".to_string(),
            result: result,
        };
    } else {
        let mut memo = HashMap::new();
        result = fibonacci(*n, &mut memo);
        obj = FibRes {
            r#type: "exact".to_string(),
            result: result,
        };
    }

    Ok(Json(obj))
}
