use lambda_http::{http::Method, handler, lambda_runtime, Body, IntoResponse, Request, Context, RequestExt, Response};
use serde::{Deserialize};
use base64::{encode_config, URL_SAFE_NO_PAD};
use tiny_keccak::{Hasher, KangarooTwelve};

const BUFFER_LEN: usize = 12;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Debug,Deserialize,Default)]
struct Args {
  #[serde(default)]
  data: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(world)).await?;
    Ok(())
}

async fn world(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // let data = match req.method() {
    //     &Method::GET =>
    //         Ok(req.query_string_parameters().get("data")),
    //     &Method::POST =>
    //         match req.payload::<Args>() {
    //             Ok(args) => args.map(|args| args.data)
    //             Err(error) => build_error(400, error.to_string().into()),
    //         },
    //     _ => Err(build_error(501, "".into())),
    // }
    Ok(match req.method() {
        &Method::GET =>
            match req.query_string_parameters().get("data") {
                Some(data) => build_response(data.as_bytes()),
                _ => build_error(422, "The `data` url field is empty.".into()),
            },
        &Method::POST =>
            match req.payload::<Args>() {
                Ok(args) =>
                    match args {
                        Some(args) => build_response(args.data.as_bytes()),
                        _ => build_error(422, "The `data` body field is empty.".into()),
                    },
                Err(error) => build_error(400, error.to_string().into()),
            },
        _ => build_error(501, "".into()),
    })
}

fn build_response(data: &[u8]) -> Response<Body> {
    Response::builder()
        .status(303)
        .header("Location", hash(data))
        .body("".into())
        .expect("failed to render response")
}

fn build_error(status: u16, body: Body) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(body)
        .expect("failed to render response")
}

fn hash(input: &[u8]) -> String {
    let mut hasher = KangarooTwelve::new(b"identicon-service");
    hasher.update(input);

    let mut output = [0u8; BUFFER_LEN];
    hasher.finalize(&mut output);

    return encode_config(output, URL_SAFE_NO_PAD);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn world_handles() {
//         let request = Request::default();
//         let expected = json!({
//         "message": "Go Serverless v1.0! Your function executed successfully!"
//         })
//         .into_response();
//         let response = world(request, Context::default())
//             .await
//             .expect("expected Ok(_) value")
//             .into_response();
//         assert_eq!(response.body(), expected.body())
//     }
// }
