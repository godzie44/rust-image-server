#include <iostream>
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>


extern "C" {
void* opencv_imdecode(const uint8_t* const buffer, size_t len, int mode);
void opencv_delete_mat(cv::Mat* mat);
uchar opencv_imwrite(const char* const file_name, const cv::Mat* const mat);
uchar opencv_resize(cv::Mat* from, cv::Mat* to, int width, int height, int inter_mode);
void* opencv_create_empty_mat();
int opencv_get_rows(const cv::Mat* const mat);
int opencv_get_cols(const cv::Mat* const mat);

}
