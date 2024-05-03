use actix_web::{
    web::{self, block},
    App, HttpRequest, HttpServer,
};
use tracing_actix_web::TracingLogger;

// #[instrument]
async fn index(req: HttpRequest) -> &'static str {
    println!("index");
    "Hello world!"
}

async fn dopanic() -> &'static str {
    println!("dopanic");
    panic!("dopanic");
    "dopanic"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 2つの非同期を実行し，joinでまつ
    let (a, b) = tokio::join!(actix_body1(), actix_body2());

    a?;
    b?;
    Ok(())
}

async fn actix_body1() -> std::io::Result<()> {
    println!("actix_body1");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(web::resource("/dopanic").to(dopanic))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn actix_body2() -> std::io::Result<()> {
    println!("actix_body2");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(web::resource("/dopanic").to(dopanic))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use actix_web::{body::to_bytes, dev::Service, http, test, Error};

//     use super::*;

//     #[actix_web::test]
//     async fn test_index() -> Result<(), Error> {
//         let app = App::new().route("/", web::get().to(index));
//         let app = test::init_service(app).await;

//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = app.call(req).await?;

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = resp.into_body();
//         assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

//         Ok(())
//     }
// }
