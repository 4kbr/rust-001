// API with axum
/*
cargo add tokio --features full
cargo add tower
cargo add serde --features derive
cargo add http
cargo add axum
cargo add axum-extra
cargo add axum-test
cargo add anyhow

*/

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello Akhi" }));
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Running app on server 0.0.0.0:8100");
    axum::serve(listener, app).await.unwrap();
    println!("after unwrap");
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        routing::{get, post},
    };
    use axum_test::TestServer;

    // ## test
    #[tokio::test]
    async fn test_axum() {
        let app = Router::new().route("/", get(|| async { "Hello Akhi" }));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text("Hello Akhi");
    }

    // ## Routing
    #[tokio::test]
    async fn test_method_routing() {
        // function yang dipanggil dihandler
        async fn hello_akhi() -> String {
            String::from("Hello Akhi")
        }

        let app = Router::new()
            // .route("/", get(hello_akhi())) // kalau functionnya gk ada `async` lansung panggil bisa kalau pakai ()
            // .route("/", post(hello_akhi()));
            .route("/", get(hello_akhi))
            .route("/", post(hello_akhi));

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/").await;
        response_get.assert_status_ok();
        response_get.assert_text("Hello Akhi");

        let response_post = server.post("/").await;
        response_post.assert_status_ok();
        response_post.assert_text("Hello Akhi");
    }

    // ## Request ( yang sebenarnya alias dari Request dari package atau crate `http`)
    #[tokio::test]
    async fn test_request() {
        // function yang dipanggil dihandler
        async fn hello_akhi(request: axum::extract::Request) -> String {
            // String::from("Hello Akhi")
            println!("ini isi dari Request {:?}", request);
            // ini isi dari Request Request { method: GET, uri: http://localhost/, version: HTTP/1.1, headers: {}, body: Body(UnsyncBoxBody) }
            format!("Hello Akhi with method {}", request.method())
        }

        let app = Router::new()
            .route("/", get(hello_akhi))
            .route("/", post(hello_akhi));

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/").await;
        response_get.assert_status_ok();
        response_get.assert_text("Hello Akhi with method GET");

        let response_post = server.post("/").await;
        response_post.assert_status_ok();
        response_post.assert_text("Hello Akhi with method POST");
    }

    // ## Http Extractor ( Extractor ini implementasi dari trait FromRequest `http`)
    #[tokio::test]
    async fn test_uri() {
        // function yang dipanggil dihandler
        // async fn route(uri: http::Uri, method: http::Method) -> String {
        // urutan parameter-nya diganti juga bisa
        async fn route(method: http::Method, uri: http::Uri) -> String {
            println!("ini isi dari Uri {:?}", uri); //ini isi dari Uri http://localhost/
            format!("Hello Method: {} Uri: {}", method.as_str(), uri.path())
        }

        let app = Router::new().route("/", get(route)).route("/", post(route));

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/").await;
        response_get.assert_status_ok();
        response_get.assert_text("Hello Method: GET Uri: /");

        let response_post = server.post("/").await;
        response_post.assert_status_ok();
        response_post.assert_text("Hello Method: POST Uri: /");
    }

    // ## something created by others
    // #[tokio::test]
    // async fn test_root_get_in_mod() {
    //     let app = Router::new().route("/", get(|| async { "Hello Akhi" }));
    //     let server = TestServer::new(app).unwrap();
    //     let response = server.get("/").await;
    //     response.assert_status_ok();
    //     response.assert_text("Hello Akhi");
    // }
    // #[tokio::test]
    // async fn test_404_not_found() {
    //     let app = Router::new(); // no routes
    //     let server = TestServer::new(app).unwrap();
    //     let response = server.get("/nope").await;
    //     assert_eq!(response.status(), StatusCode::NOT_FOUND);
    // }
    // #[tokio::test]
    // async fn test_path_param_routing() {
    //     async fn greet(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    //         format!("Hello {}", name)
    //     }
    //     let app = Router::new().route("/hello/:name", get(greet));
    //     let server = TestServer::new(app).unwrap();
    //     let response = server.get("/hello/Akhi").await;
    //     response.assert_status_ok();
    //     response.assert_text("Hello Akhi");
    // }
    // #[derive(Serialize)]
    // struct Msg<'a> {
    //     msg: &'a str,
    // }
    // #[tokio::test]
    // async fn test_json_response_as_text() {
    //     async fn json_ok() -> String {
    //         serde_json::to_string(&Msg { msg: "hello" }).unwrap()
    //     }
    //     let app = Router::new().route("/json", get(json_ok));
    //     let server = TestServer::new(app).unwrap();
    //     let response = server.get("/json").await;
    //     response.assert_status_ok();
    //     response.assert_text(r#"{"msg":"hello"}"#);
    // }
}
