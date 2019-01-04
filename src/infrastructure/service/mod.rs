use crate::infrastructure::utils;
use crate::infrastructure::SaveInfo;

use bytes::Bytes;
use mime::Mime;
use futures::Future;
use actix_web::{
    HttpMessage, client,
};
use futures::future::err as fut_err;

mod test;

#[derive(Debug, Fail)]
pub enum ContentError {
    #[fail(display = "Decoding or naming error!")]
    Decoding,
}

#[derive(Debug, Fail)]
pub enum FileError {
    #[fail(display = "File unknown error, file {} !", name)]
    UnknownError {
        name: String
    },

    #[fail(display = "Unsupported resource type, file {} !", name)]
    MimeType {
        name: String
    },
}

#[derive(Debug, Fail)]
pub enum UriError {
    #[fail(display = "Url unknown error!")]
    UnknownError,

    #[fail(display = "Invalid url {} !", uri)]
    InvalidUri {
        uri: String
    },

    #[fail(display = "Unavailable resource {} !", uri)]
    RemoteRequestFail {
        uri: String
    },

    #[fail(display = "Unsupported resource {} !", uri)]
    MimeTypeError {
        uri: String
    },
}

fn save_from_bytes_and_mime(data: &[u8], mime: &Mime, dir: &str) -> SaveInfo {
    utils::Writer::save(data, mime, dir)
        .map(|files| SaveInfo::from_files(files.0, files.1))
        .unwrap_or_else(|error| SaveInfo::from_error(error))
}

pub fn save_from_file(file_data: Result<(Bytes, Mime, String), FileError>, dir: &str) -> Result<SaveInfo, ContentError> {
    Ok(
        file_data
            .map(|(bytes, mime, file_name)| {
                match utils::guard_mime(&mime, vec![mime::IMAGE_JPEG, mime::IMAGE_PNG]) {
                    Ok(_) => save_from_bytes_and_mime(&bytes, &mime, dir),
                    Err(_) => SaveInfo::from_error(FileError::MimeType { name: file_name })
                }
            })
            .unwrap_or_else(|e| SaveInfo::from_error(e))
    )
}

pub fn save_from_uri(uri: String, dir: &str) -> Box<Future<Item=SaveInfo, Error=UriError>> {
    let request = match client::get(uri.clone()).finish() {
        Ok(req) => req,
        Err(_) => return Box::new(fut_err(UriError::InvalidUri { uri: uri.clone() })),
    };

    let dir = dir.to_owned().clone();

    let future_result = request
        .send()
        .then(move |response| {
            match response {
                Ok(r) => Ok((uri, dir, r)),
                Err(_) => Err(UriError::RemoteRequestFail { uri })
            }
        })
        .and_then(|(uri, dir, response)| {
            let mime = response
                .mime_type()
                .map_err(|_| UriError::MimeTypeError { uri: uri.clone() })?
                .ok_or(UriError::MimeTypeError { uri: uri.clone() })?;

            match utils::guard_mime(&mime, vec![mime::IMAGE_JPEG, mime::IMAGE_PNG]) {
                Ok(_) => Ok((uri, dir, response, mime)),
                Err(_) => Err(UriError::MimeTypeError { uri: uri.clone() })
            }
        })
        .and_then(|(uri, dir, response, mime)| {
            response
                .body()
                .map_err(|_| UriError::MimeTypeError { uri })
                .and_then(move |bytes: Bytes| {
                    Ok(save_from_bytes_and_mime(&bytes, &mime, &dir))
                })
        });

    Box::new(future_result)
}