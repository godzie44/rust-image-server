use crate::infrastructure::service;
use crate::infrastructure::service::ContentError;
use crate::infrastructure::SaveInfo;
use crate::infrastructure::service::FileError;
use crate::infrastructure::service::UriError;

use actix_web::{
    HttpRequest, HttpResponse, HttpMessage, AsyncResponder, FutureResponse, multipart, dev, error,
};

use futures::Future;
use futures::stream::Stream;
use futures::future::ok as fut_ok;

mod test;

#[derive(Debug, Fail)]
pub enum MultipartFieldError {
    #[fail(display = "Only file and url fields supported {} !", name)]
    UnknownField {
        name: String
    },

    #[fail(display = "Unknown field type!")]
    UnknownFieldType,

    #[fail(display = "Nested fields unsupported!")]
    NestedField,
}

// проверка is_multipart сдесь тк actix-web pred не может отфильтровать multipart/form-data из-за boundary
pub fn handle(req: &HttpRequest) -> FutureResponse<HttpResponse> {
    let is_multipart = req.headers()
        .get("content-type")
        .and_then(|content_type| Some(
            content_type.to_str().unwrap_or("").contains("multipart/form-data"))
        );

    match is_multipart {
        Some(true) => Box::new(
            req
                .multipart()
                .map_err(|_| ContentError::Decoding)
                .map(create_save_stream)
                .flatten()
                .collect()
                .map(|res| {
                    HttpResponse::Ok().json(res)
                })
                .map_err(|e| error::ErrorBadRequest(e))
        ),
        _ => fut_ok(HttpResponse::from_error(error::ErrorMethodNotAllowed(""))).responder()
    }
}

fn create_save_stream(item: multipart::MultipartItem<dev::Payload>) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    let save_stream = match item {
        multipart::MultipartItem::Field(field) => {
            let name = field
                .content_disposition()
                .and_then(|disposition| {
                    disposition.get_name().map(|name| name.to_owned().clone())
                });

            match name {
                Some(field_name) => match field_name.as_str() {
                    "file[]" => create_file_save_stream(field),
                    "uri[]" => create_uri_save_stream(field),
                    _ => Box::new(fut_ok(SaveInfo::from_error(MultipartFieldError::UnknownField { name: field_name })).into_stream())
                }
                None => Box::new(fut_ok(SaveInfo::from_error(MultipartFieldError::UnknownFieldType)).into_stream())
            }
        }

        _ => Box::new(fut_ok(SaveInfo::from_error(MultipartFieldError::NestedField)).into_stream()),
    };

    Box::new(save_stream)
}


fn create_file_save_stream(file_field: multipart::Field<dev::Payload>) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    let original_file_name = match file_field.content_disposition() {
        Some(content_disp) => content_disp.get_filename().unwrap_or("unknown").to_owned(),
        None => "unknown".to_owned()
    };
    let original_file_name_copy = original_file_name.clone();

    let mime = file_field.content_type().clone();

    Box::new(
        file_field
            .concat2()
            .map(move |bytes| (bytes, mime, original_file_name))
            .map_err(move |_| FileError::UnknownError { name: original_file_name_copy })
            .then(|file_data| service::save_from_file(file_data, crate::web::BASE_DIR))
            .into_stream()
    )
}

fn create_uri_save_stream(field: multipart::Field<dev::Payload>) -> Box<Stream<Item=SaveInfo, Error=ContentError>> {
    Box::new(
        field
            .concat2()
            .map_err(|_| UriError::UnknownError)
            .and_then(|bytes| {
                ::std::str::from_utf8(&bytes).map(|uri| uri.to_owned()).map_err(|_| UriError::UnknownError)
            })
            .and_then(|uri| {
                service::save_from_uri(uri, crate::web::BASE_DIR)
            })
            .then(|fut| {
                Ok(fut.unwrap_or_else(|e| SaveInfo::from_error(e)))
            })
            .into_stream()
    )
}
