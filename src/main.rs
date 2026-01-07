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
    use std::collections::HashMap;

    use axum::{
        Json, Router,
        body::Body,
        extract::{Query, rejection::JsonRejection},
        response::Response,
        routing::{get, post},
    };
    use axum_test::TestServer;
    use http::{HeaderMap, HeaderValue, StatusCode};
    use serde::{Deserialize, Serialize};

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

    // ## Extractor ( Extractor ini implementasi dari trait FromRequest `http`)
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

    // ## Common Extractor

    // extract query
    #[tokio::test]
    async fn test_query() {
        // function yang dipanggil dihandler
        // async fn route(uri: http::Uri, method: http::Method) -> String {
        // urutan parameter-nya diganti juga bisa
        async fn route(Query(params): Query<HashMap<String, String>>) -> String {
            println!("ini isi dari Params {:?}", params); // ini isi dari Params {"name": "Akhi"}
            format!("Hello {}", params.get("name").unwrap())
        }

        let app = Router::new().route("/", get(route)).route("/", post(route));

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/").add_query_param("name", "Akhi").await;
        response_get.assert_status_ok();
        response_get.assert_text("Hello Akhi");
    }
    // extract header
    #[tokio::test]
    async fn test_header() {
        // function yang dipanggil dihandler
        // async fn route(uri: http::Uri, method: http::Method) -> String {
        // urutan parameter-nya diganti juga bisa
        async fn route(headers: HeaderMap) -> String {
            println!("ini isi dari headers {:?}", headers);
            // ini isi dari headers {"token": "Akhi"}
            format!("Hello {}", headers["token"].to_str().unwrap())
        }

        let app = Router::new().route("/", get(route)).route("/", post(route));

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/").add_header("token", "Akhi").await;
        response_get.assert_status_ok();
        response_get.assert_text("Hello Akhi");
    }

    // ## Path Parameter Extractor
    #[tokio::test]
    async fn test_path_parameter() {
        // function yang dipanggil dihandler
        async fn route(
            axum::extract::Path((product_id, category_id)): axum::extract::Path<(String, String)>,
        ) -> String {
            format!("Product: {}, Category: {}", product_id, category_id)
        }

        let app = Router::new().route(
            "/products/{product_id}/categories/{category_id}",
            get(route),
        );

        let server = TestServer::new(app).unwrap();
        let response_get = server.get("/products/123/categories/456").await;

        response_get.assert_status_ok();
        response_get.assert_text("Product: 123, Category: 456");
    }

    // ## Body Extractor

    // body string
    #[tokio::test]
    async fn test_body_extractor_string() {
        // function yang dipanggil dihandler
        async fn route(
            body: String, // rust akan otomatis mem-format request bodynya menjadi sesuai tipe di parameter function
        ) -> String {
            format!("Body: {}", body)
        }

        let app = Router::new().route("/product", post(route));

        let server = TestServer::new(app).unwrap();
        let response = server.post("/product").text("payload dari request").await;

        response.assert_status_ok();
        response.assert_text("Body: payload dari request");
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }
    #[tokio::test]
    async fn test_body_extractor_json() {
        // function yang dipanggil dihandler
        async fn route(axum::Json(payload): axum::Json<LoginRequest>) -> String {
            println!("ini isi payload: {:?}", payload);
            // ini isi payload: LoginRequest { username: "Riley", password: "LKHKWnRDeG" }
            format!("Username: {}", payload.username)
        }

        let app = Router::new().route("/auth", post(route));

        let payload: LoginRequest = LoginRequest {
            username: "Riley".to_string(),
            password: String::from("LKHKWnRDeG"),
        };

        let server = TestServer::new(app).unwrap();
        let response = server.post("/auth").json(&payload).await;

        response.assert_status_ok();
        response.assert_text("Username: Riley");
    }
    // json error
    #[tokio::test]
    async fn test_body_extractor_json_not_valid() {
        // function yang dipanggil dihandler
        async fn route(payload: Result<axum::Json<LoginRequest>, JsonRejection>) -> String {
            println!("ini isi payload: {:?}", payload);
            // ini isi payload: LoginRequest { username: "Akhi", password: "LKHKWnRDeG" }
            match payload {
                Ok(request) => {
                    format!("Username: {}", request.username)
                }
                Err(error) => {
                    format!("Error: {:?}", error)
                }
            }
        }

        let app = Router::new().route("/auth", post(route));

        let payload: LoginRequest = LoginRequest {
            username: "Akhi".to_string(),
            password: String::from("LKHKWnRDeG"),
        };

        let server = TestServer::new(app).unwrap();
        let response = server.post("/auth").json(&payload).await;

        response.assert_status_ok();
        response.assert_text("Username: Akhi");

        let response = server.post("/auth").text("&payload").await;
        response.assert_status_ok();
        response.assert_text("Error: MissingJsonContentType(MissingJsonContentType)");
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct NamePayload {
        name: String,
    }

    #[tokio::test]
    async fn test_body_json_2() {
        async fn route(axum::Json(payload): axum::Json<NamePayload>) -> String {
            format!("Hello {}", payload.name)
        }

        let app = Router::new().route("/json", post(route));

        let payload: NamePayload = NamePayload {
            name: "Akhi".to_string(),
        };

        let server = TestServer::new(app).unwrap();
        let response = server
            .post("/json")
            .add_header("content-type", "application/json")
            .json(&payload)
            .await;

        response.assert_status_ok();
        response.assert_text("Hello Akhi");
    }

    // ## Response (type data apapun dari `IntoResponse` bisa dijadikan sebagai response)

    // response buat manual
    #[tokio::test]
    async fn test_response() {
        async fn route(request: axum::extract::Request) -> axum::response::Response {
            axum::response::Response::builder()
                .status(http::StatusCode::OK)
                .header("X-Owner", "Akhi")
                .body(Body::from(format!("Hello {}", request.method())))
                .unwrap()
        }

        let app = Router::new().route("/response", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/response").await;

        response.assert_status_ok();
        response.assert_text("Hello GET");
        response.assert_header("X-Owner", "Akhi");
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct LoginResponse {
        token: String,
        username: String,
    }

    // response Json
    #[tokio::test]
    async fn test_response_json() {
        async fn route(payload: axum::Json<LoginRequest>) -> axum::Json<LoginResponse> {
            axum::Json(LoginResponse {
                token: "TOKEN".to_string(),
                username: payload.username.clone(),
            })
        }

        let app = Router::new().route("/response", post(route));

        let server = TestServer::new(app).unwrap();

        let payload: LoginRequest = LoginRequest {
            username: "Akhi".to_string(),
            password: "token".to_string(),
        };
        let response = server.post("/response").json(&payload).await;

        response.assert_status_ok();
        response.assert_text_contains("TOKEN");
        response.assert_text_contains("Akhi");
    }

    // response tuple
    #[tokio::test]
    async fn test_response_tuple() {
        async fn route() -> (Response<()>, Json<LoginResponse>) {
            (
                Response::builder()
                    .status(StatusCode::OK)
                    .header("X-Owner", "Akhi")
                    .body(())
                    .unwrap(),
                Json(LoginResponse {
                    token: "TOKEN".to_string(),
                    username: "Akhi".to_string(),
                }),
            )
        }
        let app = Router::new().route("/get", get(route));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text_contains("TOKEN");
        response.assert_header("X-Owner", "Akhi");
    }
    // test dengan tuple berbeda
    #[tokio::test]
    async fn test_response_tuple3() {
        // urutan tuplenya bisa bebas
        async fn route() -> (StatusCode, HeaderMap, Json<LoginResponse>) {
            let mut header = HeaderMap::new();
            header.insert("X-Owner", HeaderValue::from_str("Akhi").unwrap());
            (
                StatusCode::OK,
                header,
                Json(LoginResponse {
                    token: "TOKEN".to_string(),
                    username: "Akhi".to_string(),
                }),
            )
        }
        let app = Router::new().route("/get", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;

        response.assert_status_ok();
        response.assert_text_contains("TOKEN");
        response.assert_header("X-Owner", "Akhi");
    }

    // ## Form request

    #[tokio::test]
    async fn test_form_request() {
        async fn route(axum::Form(form): axum::Form<LoginRequest>) -> String {
            println!("Ini isi form: {:?}", form);
            format!("Hello {}", form.username)
        }
        let app = Router::new().route("/", post(route));

        let server = TestServer::new(app).unwrap();
        let response = server
            .post("/")
            .form(&LoginRequest {
                username: "Akhi".to_string(),
                password: "123".to_string(),
            })
            .await;

        response.assert_status_ok();
        response.assert_text_contains("Hello Akhi");
    }

    //##
    //##
    //##
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
