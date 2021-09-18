#include <chrono>
#include <iostream>
#include <opencv2/opencv.hpp>

std::vector<cv::Point2f> control_points;

void mouse_handler(int event, int x, int y, int flags, void *userdata) 
{
    if (event == cv::EVENT_LBUTTONDOWN && control_points.size() < 4) 
    {
        std::cout << "Left button of the mouse is clicked - position (" << x << ", "
        << y << ")" << '\n';
        control_points.emplace_back(x, y);
    }     
}

void naive_bezier(const std::vector<cv::Point2f> &points, cv::Mat &window) 
{
    auto &p_0 = points[0];
    auto &p_1 = points[1];
    auto &p_2 = points[2];
    auto &p_3 = points[3];

    for (double t = 0.0; t <= 1.0; t += 0.001) 
    {
        auto point = std::pow(1 - t, 3) * p_0 + 3 * t * std::pow(1 - t, 2) * p_1 +
                 3 * std::pow(t, 2) * (1 - t) * p_2 + std::pow(t, 3) * p_3;

        window.at<cv::Vec3b>(point.y, point.x)[2] = 255;
    }
}

cv::Point2f recursive_bezier(const std::vector<cv::Point2f> &control_points, float t) 
{
    if (control_points.size() == 1)
        return control_points[0];

    std::vector<cv::Point2f> new_list;
    for (int i = 0; i < control_points.size() - 1; i++) {
        auto point = t * (control_points[i + 1] - control_points[i]) + control_points[i];
        new_list.push_back(point);
    }
    return recursive_bezier(new_list, t);
}

void bezier(const std::vector<cv::Point2f> &control_points, cv::Mat &window) 
{
    // TODO: Iterate through all t = 0 to t = 1 with small steps, and call de Casteljau's 
    // recursive Bezier algorithm.
    for (double t = 0.0; t <= 1.0; t += 0.001) {
        auto point = recursive_bezier(control_points, t);
        window.at<cv::Vec3b>(point.y, point.x)[1] = 255;

        float pos_x = (point.x - floor(point.x)) > 0.5 ? 1 : -1;
        float pos_y = (point.y - floor(point.y)) > 0.5 ? 1 : -1;

        std::vector<cv::Point2f> vec;
        vec.push_back(cv::Point2f(floor(point.x + pos_x), floor(point.y)));
        vec.push_back(cv::Point2f(floor(point.x), floor(point.y + pos_y)));
        vec.push_back(cv::Point2f(floor(point.x + pos_x), floor(point.y + pos_y)));

        auto d = cv::Point2f(point.x - floor(point.x) - 0.5, point.y - floor(point.y) - 0.5);
        float dis = sqrt(d.x * d.x + d.y * d.y);

        for (const auto &p : vec) {
            float cx = p.x + 0.5;
            float cy = p.y + 0.5;

            auto d1 = cv::Point2f(cx - floor(point.x) - 0.5, cy - floor(point.y) - 0.5);
            float l = sqrt(d1.x * d1.x + d1.y * d1.y);
            
            auto color = window.at<cv::Vec3b>(cy, cx)[1];
            window.at<cv::Vec3b>(cy, cx)[1] = std::max((int)color, (int)(255 * (dis / l)));
        }
    }

}

int main() 
{
    cv::Mat window = cv::Mat(700, 700, CV_8UC3, cv::Scalar(0));
    cv::cvtColor(window, window, cv::COLOR_BGR2RGB);
    cv::namedWindow("Bezier Curve", cv::WINDOW_AUTOSIZE);

    cv::setMouseCallback("Bezier Curve", mouse_handler, nullptr);

    int key = -1;
    while (key != 27) 
    {
        for (auto &point : control_points) 
        {
            cv::circle(window, point, 3, {255, 255, 255}, 3);
        }

        if (control_points.size() == 4) 
        {
            // naive_bezier(control_points, window);
            bezier(control_points, window);

            cv::imshow("Bezier Curve", window);
            cv::imwrite("my_bezier_curve.png", window);
            key = cv::waitKey(0);

            return 0;
        }

        cv::imshow("Bezier Curve", window);
        key = cv::waitKey(20);
    }

return 0;
}
