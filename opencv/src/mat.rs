use std::os::raw::c_int;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_uchar;

#[derive(Clone, Copy, Debug)]
pub enum MatPointer {}

extern "C" {
    fn opencv_imdecode(buf: *const u8, l: usize, m: c_int) -> *mut MatPointer;
    fn opencv_delete_mat(mat: *mut MatPointer);
    fn opencv_imwrite(ext: *const c_char, pointer: *const MatPointer) -> c_uchar;
    fn opencv_create_empty_mat() -> *mut MatPointer;
    fn opencv_resize(from: *const MatPointer, to: *mut MatPointer, width: c_int, height: c_int, inter_mode: c_int) -> c_uchar;
    fn opencv_get_rows(mat: *const MatPointer) -> c_int;
    fn opencv_get_cols(mat: *const MatPointer) -> c_int;
}

#[derive(Debug)]
pub struct Mat {
    pointer: *mut MatPointer,
    pub cols: i32,
    pub rows: i32,
}

impl Mat {
    fn from_pointer(pointer: *mut MatPointer) -> Mat {
        Mat {
            pointer,
            cols: unsafe { opencv_get_cols(pointer) } as i32,
            rows: unsafe { opencv_get_rows(pointer) } as i32,
        }
    }

    fn update(&mut self) {
        self.cols = unsafe { opencv_get_cols(self.pointer) } as i32;
        self.rows = unsafe { opencv_get_rows(self.pointer) } as i32;
    }

    pub fn from_bytes(bytes: &[u8]) -> Mat {
        let pointer = unsafe { opencv_imdecode(bytes.as_ptr(), bytes.len(), -1) };

        Self::from_pointer(pointer)
    }

    pub fn save(&self, file_path: &str) -> Result<(), String> {
        let file_path = CString::new(file_path).unwrap();
        let res = unsafe { opencv_imwrite(file_path.into_raw(), self.pointer) };

        match res {
            0 => Err("Error while saving file to disk".to_owned()),
            _ => Ok(()),
        }
    }

    pub fn resize(&self, width: i32, height: i32) -> Result<Mat, String> {
        let mut new_mat = Mat::from_pointer(unsafe { opencv_create_empty_mat() });

        let result = unsafe { opencv_resize(self.pointer, new_mat.pointer, width as c_int, height as c_int, 3) };

        match result {
            0 => Err("Error while resizing".to_owned()),
            _ => {new_mat.update(); Ok(new_mat)},
        }
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            opencv_delete_mat(self.pointer);
        }
    }
}
