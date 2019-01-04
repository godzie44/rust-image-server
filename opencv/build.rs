extern crate cc;

#[cfg(unix)]
fn main() {

    let mut opencv_config = cc::Build::new();
    opencv_config
        .cpp(true)
        .files(vec!["c_files/mat.cc"])
        .include("c_files")
        .include("/usr/local/include")
        .flag("--std=c++11");

    opencv_config.compile("libopencv-rust.a");

    println!("cargo:rustc-link-search=c_files=/usr/local/lib");
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_imgproc");
}