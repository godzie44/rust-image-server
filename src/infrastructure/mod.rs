pub mod service;
pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveInfo {
    pub ok: bool,
    pub success_info: Option<SuccessInfo>,
    pub fail_info: Option<FailInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SuccessInfo {
    pub original_path: String,
    pub resize_path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FailInfo {
    pub reason: String
}

impl SaveInfo {
    /// ```
    /// use image_pool::infrastructure::SaveInfo;
    ///
    /// let sr = SaveInfo::from_files("./1.png".to_owned(), "./2.png".to_owned());
    ///
    /// assert_eq!(true, sr.ok);
    /// assert_eq!(Option::None, sr.fail_info);
    /// assert_eq!("./1.png".to_owned(), sr.success_info.unwrap().original_path);
    /// ```
    pub fn from_files(file_original: String, file_resized: String) -> Self {
        SaveInfo {
            ok: true,
            success_info: Some(SuccessInfo {
                original_path: file_original,
                resize_path: file_resized,
            }),
            fail_info: None,
        }
    }

    /// ```
    /// use image_pool::infrastructure::SaveInfo;
    /// use failure::Fail;
    ///
    ///#[derive(Debug, Fail)]
    ///pub enum MyError {
    ///    #[fail(display = "unknown error!")]
    ///    UnknownError,
    ///}
    ///
    /// let sr = SaveInfo::from_error(MyError::UnknownError);
    ///
    /// assert_eq!(false, sr.ok);
    /// assert_eq!(Option::None, sr.success_info);
    /// assert_ne!(Option::None, sr.fail_info);
    /// ```
    pub fn from_error<E: failure::Fail>(error: E) -> Self {
        SaveInfo {
            ok: false,
            success_info: None,
            fail_info: Some(FailInfo {
                reason: error.to_string()
            }),
        }
    }
}