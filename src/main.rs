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
        extract::{Query, Request, rejection::JsonRejection},
        response::Response,
        routing::{get, post},
    };
    use axum_extra::extract::{CookieJar, cookie::Cookie};
    use axum_test::{
        TestServer,
        multipart::{MultipartForm, Part},
    };
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

    // ## Multipart Request
    /*
    # Multipart Request
    - Selain Form, Axum juga menyediakan Extractor untuk menangani Multipart Request menggunakan struct Multipart
    - https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
    - Karena Multipart Request memerlukan membaca seluruh data Request Body, maka untuk Multipart Request, kita perlu tambahkan sebagai parameter di bagian
    - paling akhir jika menggunakan lebih dari satu Extractor
    - Multipart di Axum memerlukan features multipart, kita bisa tambahkan --features multipart, ketika menambah library axum

    */
    #[tokio::test]
    async fn test_multipart_request() {
        // Multipart memerlukan features multipart dari package/crate axum
        async fn route(mut payload: axum::extract::Multipart) -> String {
            println!("Ini isi payload: {:?}", payload);
            /*
            Ini isi payload: Multipart { inner: Multipart { state: Mutex { data: MultipartState { buffer: StreamBuffer,
            boundary: "f2b2be0ded00768f-2575e2d4e1089dd2-d3c6356b75f77a02-c9a24e17bee0a3dc-446e84f0d9c5a49a-3131f1e452e0fa82-70a79cc7d0bb1d3f-747656fc9118a904",
            stage: FindingFirstBoundary, next_field_idx: 0, curr_field_name: None, curr_field_size_limit: 18446744073709551615,
            curr_field_size_counter: 0, constraints: Constraints { size_limit: SizeLimit { whole_stream: 18446744073709551615,
            per_field: 18446744073709551615, field_map: {} }, allowed_fields: None } }} } }
            */

            let mut profile: axum::body::Bytes = axum::body::Bytes::new();
            let mut username: String = "".to_string();

            while let Some(field) = payload.next_field().await.unwrap() {
                if field.name().unwrap_or("") == "profile" {
                    profile = field.bytes().await.unwrap();
                } else if field.name().unwrap_or("") == "username" {
                    username = field.text().await.unwrap();
                }
            }

            assert!(profile.len() > 0); // make sure profile is not empty
            format!("Hello {}", username)
        }
        let app = Router::new().route("/", post(route));

        let request = MultipartForm::new()
            .add_text("username", "Akhi")
            .add_text("password", "U0QJ9tt8cDwb")
            // add part diakhir
            .add_part("profile", Part::bytes(axum::body::Bytes::from("profile")));

        let server = TestServer::new(app).unwrap();
        let response = server.post("/").multipart(request).await;

        response.assert_status_ok();
        response.assert_text_contains("Hello Akhi");
    }

    // ## Cookie
    /*
    # Cookie
    - Saat kita membuat Web, kadang kita butuh membuat Cookie
    - Sebenarnya cara membuat Cookie sendiri kita bisa lakukan secara manual menggunakan Response Header Set-Cookie
    - Namun, kita bisa gunakan library axum-extra untuk membantu melakukan manajemen Cookie secara mudah
    - https://crates.io/crates/axum-extra
    `cargo add axum-extra --features cookie`
    */
    #[tokio::test]
    async fn test_cookie_response() {
        async fn route(query: Query<HashMap<String, String>>) -> (CookieJar, String) {
            println!("Ini isi query: {:?}", query);
            /*
            Ini isi query: Query({"name": "Akhi"})
            */
            let name: &String = query.get("name").unwrap();
            (
                CookieJar::new().add(axum_extra::extract::cookie::Cookie::new(
                    "name",
                    name.clone(),
                )),
                format!("Hello {}", name),
            )
        }

        let app = Router::new().route("/", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").add_query_param("name", "Akhi").await;

        response.assert_status_ok();
        response.assert_text_contains("Hello Akhi");
        response.assert_contains_header("Set-Cookie");
        response.assert_header("Set-Cookie", "name=Akhi");
    }
    #[tokio::test]
    async fn test_cookie_request() {
        async fn route(cookie: CookieJar) -> String {
            println!("Ini isi cookie: {:?}", cookie);
            /*
            Ini isi cookie: CookieJar { jar: CookieJar { original_cookies: {DeltaCookie { cookie: Cookie { cookie_string: Some("name=Akhi"),
            name: Indexed(0, 4), value: Indexed(5, 9), expires: None, max_age: None, domain: None, path: None, secure: None,
            http_only: None, same_site: None, partitioned: None }, removed: false }}, delta_cookies: {} } }
            */
            let name = cookie.get("name").unwrap().value();
            format!("Hello {}", name)
        }

        let app = Router::new().route("/", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server
            .get("/")
            // .add_header("Cookie", "name=Akhi") //ini bisa
            .add_cookie(Cookie::new("name", "Akhi")) // ini juga bisa
            .await;

        response.assert_status_ok();
        response.assert_text_contains("Hello Akhi");
    }

    // ## Middleware
    /*
    # Axum Middleware
    - Untuk menambahkan Middleware di Axum nya, kita bisa gunakan di level Router menggunakan Router::layer / Router::route_layer
    - Jika kita tambahkan di Router, artinya semua routing akan menggunakan Middleware tersebut
    - Atau kita bisa tambahkan di MethodRouter::layer / MethodRouter::route_layer
    - Jika kita tambahkan di MethodRouter, artinya hanya routing tersebut yang akan menggunakan Middleware tersebut
    */
    // function untuk dipakai dimiddleware
    async fn log_middleware(
        request: axum::extract::Request,
        next: axum::middleware::Next,
    ) -> axum::response::Response {
        println!("Receive request {} {}", request.method(), request.uri());
        let response = next.run(request).await;
        println!("Send response {}", response.status());
        response
    }
    async fn request_id_middleware<T>(mut request: Request<T>) -> Request<T> {
        let request_id = "random-id"; // create random id, example from uuid
        request
            .headers_mut()
            .insert("X-Request-Id", request_id.parse().unwrap());
        request
    }

    // test pakai middleware
    #[tokio::test]
    async fn test_middleware() {
        async fn route(method: http::Method, headers: HeaderMap) -> String {
            println!("Ini isi headers: {:?}", headers);
            /*
            Ini isi headers: {"x-request-id": "random-id"}
            */
            let request_id = headers.get("X-Request-Id").unwrap();
            format!(
                "Hello from method: {}, with request id: {}",
                method,
                request_id.to_str().unwrap()
            )
        }

        let app = Router::new()
            .route(
                "/",
                get(route)
                    // middleware di method get
                    .layer(axum::middleware::from_fn(log_middleware)),
            )
            // middleware di route "/"
            .layer(axum::middleware::map_request(request_id_middleware));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text_contains("Hello from method: GET, with request id: random-id");
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
