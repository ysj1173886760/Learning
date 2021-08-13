#pragma once
#include <iostream>
#include <cmath>
#include <random>
#include "RandomGenerator.hpp"

#undef M_PI
#define M_PI 3.141592653589793f

extern const float  EPSILON;
const float kInfinity = std::numeric_limits<float>::max();

inline float clamp(const float &lo, const float &hi, const float &v)
{ return std::max(lo, std::min(hi, v)); }

inline  bool solveQuadratic(const float &a, const float &b, const float &c, float &x0, float &x1)
{
    float discr = b * b - 4 * a * c;
    if (discr < 0) return false;
    else if (discr == 0) x0 = x1 = - 0.5 * b / a;
    else {
        float q = (b > 0) ?
                  -0.5 * (b + sqrt(discr)) :
                  -0.5 * (b - sqrt(discr));
        x0 = q / a;
        x1 = c / q;
    }
    if (x0 > x1) std::swap(x0, x1);
    return true;
}

inline float get_random_float()
{
    thread_local static RandomGenerator randomGenerator;
    return randomGenerator.get_random_number();
}

inline void UpdateProgress(float progress)
{
    int barWidth = 70;

    std::cout << "[";
    int pos = barWidth * progress;
    for (int i = 0; i < barWidth; ++i) {
        if (i < pos) std::cout << "=";
        else if (i == pos) std::cout << ">";
        else std::cout << " ";
    }
    std::cout << "] " << int(progress * 100.0) << " %\r";
    std::cout.flush();
};

inline double distributionGGX(const Vector3f &normal, const Vector3f &h, double rough_ness) {
    double a2 = rough_ness * rough_ness;
    double nDotH = std::max(dotProduct(normal, h), 0.0f);
    double nDotH2 = nDotH * nDotH;

    double denom = nDotH2 * (a2 - 1.0f) + 1.0f;
    denom = M_PI * denom * denom;
    return a2 / denom;
}

inline double geometrySchlickGGX(double nDotV, double k) {
    double denom = nDotV * (1.0f - k) + k;
    return nDotV / denom;
}

inline double geometrySmith(const Vector3f &normal, const Vector3f &v, const Vector3f &l, double k) {
    double nDotV = std::max(dotProduct(normal, v), 0.0f);
    double nDotL = std::max(dotProduct(normal, l), 0.0f);
    double ggx1 = geometrySchlickGGX(nDotV, k);
    double ggx2 = geometrySchlickGGX(nDotL, k);

    return ggx1 * ggx2;
}

inline Vector3f fresnelSchlick(double cosTheta, const Vector3f &F0) {
    return F0 + (Vector3f(1.0f) - F0) * pow(1.0 - cosTheta, 5.0);
}