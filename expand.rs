#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use actix_web::{web, App, HttpRequest, HttpServer};
use tracing_actix_web::TracingLogger;
async fn index(req: HttpRequest) -> &'static str {
    "Hello world!"
}
fn main() -> std::io::Result<()> {
    let body = async {
        HttpServer::new(|| {
                App::new()
                    .service(
                        web::resource("/index.html").to(|| async { "Hello world!" }),
                    )
                    .service(web::resource("/").to(index))
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
