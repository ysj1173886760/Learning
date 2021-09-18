//
// Created by LEI XU on 4/27/19.
//

#ifndef RASTERIZER_TEXTURE_H
#define RASTERIZER_TEXTURE_H
#include "global.hpp"
#include <eigen3/Eigen/Eigen>
#include <opencv2/opencv.hpp>
class Texture{
private:
    cv::Mat image_data;

public:
    Texture(const std::string& name)
    {
        image_data = cv::imread(name);
        cv::cvtColor(image_data, image_data, cv::COLOR_RGB2BGR);
        width = image_data.cols;
        height = image_data.rows;
    }

    int width, height;

    Eigen::Vector3f getColor(float u, float v)
    {
        auto u_img = u * width;
        auto v_img = (1 - v) * height;
        auto color = image_data.at<cv::Vec3b>(v_img, u_img);
        return Eigen::Vector3f(color[0], color[1], color[2]);
    }

    Eigen::Vector3f getBilinerColor(float u, float v) {
        auto u_img = u * width;
        auto v_img = (1 - v) * height;

        auto u_min = (int)floor(u_img);
        auto u_max = std::min((int)ceil(u_img), width);
        auto v_min = (int)floor(v_img);
        auto v_max = std::min((int)ceil(v_img), height);

        auto color1 = image_data.at<cv::Vec3b>(v_max, u_min);
        auto color2 = image_data.at<cv::Vec3b>(v_max, u_max);
        auto color3 = image_data.at<cv::Vec3b>(v_min, u_min);
        auto color4 = image_data.at<cv::Vec3b>(v_min, u_max);

        auto ratio_u = (u_img - u_min) / (u_max - u_min);
        auto ratio_v = (v_img - v_min) / (v_max - v_min);
        auto up = (1 - ratio_u) * color3 + ratio_u * color4;
        auto down = (1 - ratio_u) * color1 + ratio_u * color2;
        auto col = (1 - ratio_v) * up + ratio_v * down;

        return Eigen::Vector3f(col[0], col[1], col[2]);
    }

};
#endif //RASTERIZER_TEXTURE_H
