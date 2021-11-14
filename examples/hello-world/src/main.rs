//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    async_trait,
    body::{Bytes, Full},
    extract::{FromRequest, RequestParts},
    http::Response,
    response::IntoResponse,
};
use axum_debug::debug_handler;
use std::convert::Infallible;

struct A;

impl A {
    #[debug_handler]
    async fn handler(self) -> Self {
        A
    }
}

impl IntoResponse for A {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        todo!()
    }
}

#[async_trait]
impl FromRequest for A {
    type Rejection = ();

    async fn from_request(_req: &mut RequestParts) -> Result<Self, Self::Rejection> {
        unimplemented!()
    }
}

fn main() {}
