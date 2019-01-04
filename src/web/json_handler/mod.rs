use crate::infrastructure::service;
use crate::infrastructure::service::ContentError;
use crate::infrastructure::SaveInfo;
use crate::infrastructure::service::FileError;
use crate::infrastructure::service::UriError;

use actix_web::{
    Error, HttpRequest, HttpResponse, HttpMessage, AsyncResponder, FutureResponse, error,
};

use futures::Future;
use futures::stream::{Stream, iter_ok};
use bytes::Bytes;

mod test;
mod utils;

#[derive(Debug, Deserialize)]
struct JsonRequest {
    #[serde(with = "utils::base64_image")]
    pub file: Vec<(Bytes, mime::Mime)>,
    pub uri: Vec<String>,
}

pub fn handle(req: &HttpRequest) -> FutureResponse<HttpResponse> {
    req
        .json()
        .map_err(|_| ContentError::Decoding)
        .and_then(|json_request: JsonRequest| {
            create_save_stream(json_request).collect()
        })
        .and_then(|result| {
            Ok(HttpResponse::Ok().json(result))
        })
        .map_err(|e| error::ErrorBadRequest(e))
        .responder()
}

fn create_save_stream(json_request: JsonRequest) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    let file_stream = create_base64_save_stream(json_request.file);
    let uri_stream = create_uri_save_stream(json_request.uri);

    Box::new(
        file_stream.chain(uri_stream)
    )
}

fn create_base64_save_stream(files: Vec<(Bytes, mime::Mime)>) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    Box::new(
        iter_ok::<_, Error>(files)
            .then(|fut| match fut {
                Ok(file) => Ok((file.0, file.1, "base64".to_owned())),
                Err(_) => Err(FileError::UnknownError { name: "base64".to_owned() })
            })
            .then(|payload| {
                service::save_from_file(payload, crate::web::BASE_DIR)
            })
    )
}

fn create_uri_save_stream(uris: Vec<String>) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    Box::new(
        iter_ok::<_, Error>(uris)
            .map_err(|_| UriError::UnknownError)
            .and_then(|uri| {
                service::save_from_uri(uri, crate::web::BASE_DIR)
            })
            .then(|fut| {
                Ok(fut.unwrap_or_else(|e| SaveInfo::from_error(e)))
            })
    )
}

