use mime::Mime;
use uuid::Uuid;
use opencv::mat::Mat;

type FileName = String;

#[derive(Debug, Fail)]
pub enum WriterError {
    #[fail(display = "Unknown save error!")]
    Opencv,
}

pub struct Writer;

impl Writer {
    const RESIZE_WIDTH:i32 = 100;
    const RESIZE_HEIGHT:i32 = 100;

    pub fn save(data: &[u8], m_type: &Mime, dir: &str) -> Result<(FileName, FileName), WriterError> {
        let file_path = format!("{}/{}", dir, Self::generate_fp(m_type.subtype().as_ref()));

        let mat = Mat::from_bytes(data);
        mat.save(&file_path).map_err(|_| WriterError::Opencv)?;

        let resized_file_path = format!("{}/{}", dir, Self::generate_fp(m_type.subtype().as_ref()));
        let resized_mat = mat.resize(Self::RESIZE_WIDTH, Self::RESIZE_HEIGHT).map_err(|_| WriterError::Opencv)?;

        resized_mat.save(&resized_file_path).map_err(|_| WriterError::Opencv)?;

        Ok((file_path, resized_file_path))
    }

    fn generate_fp(ext: &str) -> String {
        format!("{}.{}", Uuid::new_v4(), ext)
    }
}

/// ```
/// use image_pool::infrastructure::utils::guard_mime;
/// use mime::Mime;
///
/// let res = guard_mime(&mime::IMAGE_JPEG, vec![mime::IMAGE_JPEG, mime::IMAGE_PNG]);
/// assert!(res.is_ok());
///
/// let res = guard_mime(&mime::APPLICATION_JAVASCRIPT, vec![mime::IMAGE_JPEG, mime::IMAGE_PNG]);
/// assert!(res.is_err());
/// ```
pub fn guard_mime(mime: &Mime, allowed_types: Vec<mime::Mime>) -> Result<(), String> {
    let result = allowed_types.iter().any(|allowed| {
        mime == allowed
    });

    match result {
        true => Ok(()),
        false => Err("Unsupported mime type".to_owned())
    }
}

#[cfg(test)]
mod writer_test {
    use std::fs;
    use crate::infrastructure::utils::Writer;
    use std::path::Path;
    use opencv::mat::Mat;

    #[test]
    fn test_create_original_and_resize_image() {
        const TEST_DIR: &str = "./temp_test_dir/writer-temp-test";

        setup(TEST_DIR);

        let data = fs::read("./test_data/2d.png").expect("Cant read img file");
        let files = Writer::save(&data, &mime::IMAGE_PNG, TEST_DIR).unwrap();

        check_exists_and_size(&files.0, 69, 94);
        check_exists_and_size(&files.1, 100, 100);

        teardown(TEST_DIR);
    }

    #[test]
    fn test_error_throw_on_bad_ext() {
        const TEST_DIR: &str = "./temp_test_dir/writer-temp-test2";

        setup(TEST_DIR);

        let data = fs::read("./test_data/2d.png").expect("Cant read img file");
        let result = Writer::save(&data, &mime::IMAGE_SVG, TEST_DIR);
        assert!(result.is_err());

        teardown(TEST_DIR);
    }

    fn check_exists_and_size(fp: &str, w: i32, h: i32) {
        assert!(Path::new(fp).exists());
        let img = fs::read(fp).expect("Cant read img file");
        let img = Mat::from_bytes(&img);
        assert_eq!(w, img.cols);
        assert_eq!(h, img.rows);
    }

    fn setup(dir: &str) {
        fs::create_dir_all(dir).unwrap();
    }

    fn teardown(dir: &str) {
        fs::remove_dir_all(dir).unwrap();
    }
}