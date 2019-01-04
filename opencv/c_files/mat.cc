#include "mat.h"

extern "C" {
void* opencv_imdecode(const uint8_t* const buffer, size_t len, int mode) {
    cv::Mat* result = new cv::Mat();
    std::vector<uchar> input(buffer, buffer + len);
    cv::imdecode(cv::Mat(input), mode, result);
    return result;
}

uchar opencv_imwrite(const char* const file_name, const cv::Mat* const mat) {
    try {
        cv::imwrite(file_name, *mat);
        return 1;
    } catch (cv::Exception& ex) {
        return 0;
    }
}

uchar opencv_resize(cv::Mat* from, cv::Mat* to, int width, int height, int inter_mode) {
    cv::Size cv_sz(width, height);

    try {
        cv::resize(*from, *to, cv_sz, 0.0, 0.0, inter_mode);
        return 1;
    } catch (cv::Exception& ex) {
        return 0;
    }
}

void* opencv_create_empty_mat() {
    cv::Mat* mat = new cv::Mat();

    return mat;
}

int opencv_get_cols(const cv::Mat* const mat) {
    return mat->cols;
}

int opencv_get_rows(const cv::Mat* const mat) {
    return mat->rows;
}

void opencv_delete_mat(cv::Mat* mat) {
    delete mat;
    mat = nullptr;
}
}
