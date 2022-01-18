use lambda_http::{handler, lambda_runtime, IntoResponse, Request, Context, RequestExt, Response};
use base64::{decode_config, URL_SAFE_NO_PAD};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(world)).await?;
    Ok(())
}

async fn world(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(match event.path_parameters().get("hash") {
        Some(hash) => visualize(hash).into_response(),
        _ => Response::builder()
            .status(400)
            .body("Empty hash".into())
            .expect("failed to render response"),
    })
}

fn visualize(hash: &str) -> String {
    let buffer = decode_config(hash, URL_SAFE_NO_PAD).unwrap();
    let vec: Vec<u8> = (0..buffer.len() * 8).step_by(6).map(|pos| get_bits(&buffer, pos, 6)).collect();
    return format!("{:?}", vec);
}

fn get_bits(buffer: &Vec<u8>, position: usize, count: usize) -> u8 {
    let index = position / 8;
    let byte_index = position % 8;

    let bits = buffer[index] << byte_index >> 8 - count;

    if byte_index + count > 8 {
        let hang = get_bits(buffer, position + 8 - byte_index, byte_index + count - 8);
        return bits | hang;
    }

    return bits;
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
