pub mod base64_image {
    extern crate base64;

    use serde::{de, Deserialize, Deserializer};
    use bytes::Bytes;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<(Bytes, mime::Mime)>, D::Error>
        where D: Deserializer<'de>
    {
        let v = <Vec<&str>>::deserialize(deserializer)?;

        let mut result: Vec<(Bytes, mime::Mime)> = Vec::new();

        for image_candidate in v {
            let mime = detect_mime(image_candidate);

            match mime {
                Some(mime) => {
                    let image_candidate_dec = base64::decode(image_candidate);

                    match image_candidate_dec {
                        Ok(img) => result.push((Bytes::from(img), mime)),
                        Err(_) => return Err(de::Error::custom("Broken base64 string"))
                    }
                },
                None => return Err(de::Error::custom("Unsupported mime type"))
            }
        }

        Ok(result)
    }

    fn detect_mime(base64_str: &str) -> Option<mime::Mime> {
        match base64_str.chars().next() {
            Some('/') => Some(mime::IMAGE_JPEG),
            Some('i') => Some(mime::IMAGE_PNG),
            _ => Some(mime::TEXT_PLAIN)
        }
    }
}