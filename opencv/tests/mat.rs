extern crate opencv;

#[cfg(test)]
mod test {
    use std::fs;
    use opencv::mat;
    use std::path::Path;

    #[test]
    fn test_mat_create() {
        let data = fs::read("./test_data/2d.png").expect("Cant read img file");

        let mat = mat::Mat::from_bytes(&data);
        assert_eq!(69, mat.cols);
        assert_eq!(94, mat.rows);
    }

    #[test]
    fn test_bad_data_resize_err() {
        let data = vec![1,2,3,4,5];

        let mat = mat::Mat::from_bytes(&data);

        let mat = mat.resize(100, 100);

        assert!(mat.is_err());
    }

    #[test]
    fn test_saving_success() {
        let data = fs::read("./test_data/2d.png").expect("Cant read img file");

        let mat = mat::Mat::from_bytes(&data);
        mat.save("./test_data/from_tests.png").unwrap();

        assert!(Path::new("./test_data/from_tests.png").exists());
        fs::remove_file("./test_data/from_tests.png").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_saving_fail_for_bad_ext() {
        let data = fs::read("./test_data/2d.png").expect("Cant read img file");

        let mat = mat::Mat::from_bytes(&data);
        mat.save("./test_data/from_tests.aaabbbccc").unwrap();
    }

    #[test]
    fn test_resize() {
        let data = fs::read("./test_data/2d.png").expect("Cant read img file");

        let mat = mat::Mat::from_bytes(&data);
        let resized_mat = mat.resize(50, 100).unwrap();

        assert_eq!(50, resized_mat.cols);
        assert_eq!(100, resized_mat.rows);
    }

    #[test]
    fn test_resize_and_save_after() {
        let data = fs::read("./test_data/2d.png").expect("Cant read img file");

        let mat = mat::Mat::from_bytes(&data);
        mat.resize(200, 40).unwrap().save("./test_data/resized.png").unwrap();

        assert!(Path::new("./test_data/resized.png").exists());
        fs::remove_file("./test_data/resized.png").unwrap();
    }
}