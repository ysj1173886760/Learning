//
// Created by LEI XU on 5/16/19.
//

#ifndef RAYTRACING_MATERIAL_H
#define RAYTRACING_MATERIAL_H

#include "Vector.hpp"

enum MaterialType { DIFFUSE, MICROFACET};

class Material{
private:

    // Compute reflection direction
    Vector3f reflect(const Vector3f &I, const Vector3f &N) const
    {
        return I - 2 * dotProduct(I, N) * N;
    }

    // Compute refraction direction using Snell's law
    //
    // We need to handle with care the two possible situations:
    //
    //    - When the ray is inside the object
    //
    //    - When the ray is outside.
    //
    // If the ray is outside, you need to make cosi positive cosi = -N.I
    //
    // If the ray is inside, you need to invert the refractive indices and negate the normal N
    Vector3f refract(const Vector3f &I, const Vector3f &N, const float &ior) const
    {
        float cosi = clamp(-1, 1, dotProduct(I, N));
        float etai = 1, etat = ior;
        Vector3f n = N;
        if (cosi < 0) { cosi = -cosi; } else { std::swap(etai, etat); n= -N; }
        float eta = etai / etat;
        float k = 1 - eta * eta * (1 - cosi * cosi);
        return k < 0 ? 0 : eta * I + (eta * cosi - sqrtf(k)) * n;
    }

    // Compute Fresnel equation
    //
    // \param I is the incident view direction
    //
    // \param N is the normal at the intersection point
    //
    // \param ior is the material refractive index
    //
    // \param[out] kr is the amount of light reflected
    void fresnel(const Vector3f &I, const Vector3f &N, const float &ior, float &kr) const
    {
        float cosi = clamp(-1, 1, dotProduct(I, N));
        float etai = 1, etat = ior;
        if (cosi > 0) {  std::swap(etai, etat); }
        // Compute sini using Snell's law
        float sint = etai / etat * sqrtf(std::max(0.f, 1 - cosi * cosi));
        // Total internal reflection
        if (sint >= 1) {
            kr = 1;
        }
        else {
            float cost = sqrtf(std::max(0.f, 1 - sint * sint));
            cosi = fabsf(cosi);
            float Rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
            float Rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
            kr = (Rs * Rs + Rp * Rp) / 2;
        }
        // As a consequence of the conservation of energy, transmittance is given by:
        // kt = 1 - kr;
    }

    Vector3f toWorld(const Vector3f &a, const Vector3f &N){
        Vector3f B, C;
        if (std::fabs(N.x) > std::fabs(N.y)){
            float invLen = 1.0f / std::sqrt(N.x * N.x + N.z * N.z);
            C = Vector3f(N.z * invLen, 0.0f, -N.x *invLen);
        }
        else {
            float invLen = 1.0f / std::sqrt(N.y * N.y + N.z * N.z);
            C = Vector3f(0.0f, N.z * invLen, -N.y *invLen);
        }
        B = crossProduct(C, N);
        return a.x * B + a.y * C + a.z * N;
    }


    Vector3f importanceSampling(const Vector3f &wo, const Vector3f &normal) {
        double r0 = get_random_float();
        double r1 = get_random_float();
        double a2 = roughness * roughness;
        double theta = acos(sqrt((1 - r0) / ((a2 - 1) * r0 + 1)));
        double phi = 2 * M_PI * r1;
        
        // double x = sin(theta) * cos(phi);
        // double y = cos(theta);
        // double z = sin(theta) * sin(phi);
        double x = sin(theta) * cos(phi);
        double y = sin(theta) * sin(phi);
        double z = cos(theta);
        Vector3f wm = Vector3f(x, y, z);
        Vector3f wm_w = toWorld(wm, normal);
        return reflect(wo, wm_w);
    }

    double importancePDF(const Vector3f &wo, const Vector3f &wi, const Vector3f &normal) {
        Vector3f h = normalize(wo + wi);
        double cosTheta = dotProduct(normal, h);
        double D = distributionGGX(normal, h, roughness);
        return (D * cosTheta) / (4.0f * dotProduct(wo, h));
    }

public:
    MaterialType m_type;
    //Vector3f m_color;
    Vector3f m_emission;
    float ior;
    Vector3f albedo;
    Vector3f F0;
    float specularExponent;
    float roughness;
    float metallic;
    //Texture tex;

    inline Material(MaterialType t=DIFFUSE, Vector3f e=Vector3f(0,0,0));
    inline Material(MaterialType t, const Vector3f &e, const Vector3f &albedo, float roughness, float metallic);
    inline MaterialType getType();
    //inline Vector3f getColor();
    inline Vector3f getColorAt(double u, double v);
    inline Vector3f getEmission();
    inline bool hasEmission();

    // sample a ray by Material properties
    inline Vector3f sample(const Vector3f &wi, const Vector3f &N);
    // given a ray, calculate the PdF of this ray
    inline float pdf(const Vector3f &wi, const Vector3f &wo, const Vector3f &N);
    // given a ray, calculate the contribution of this ray
    inline Vector3f eval(const Vector3f &wi, const Vector3f &wo, const Vector3f &N);

};

Material::Material(MaterialType t, Vector3f e){
    m_type = t;
    //m_color = c;
    m_emission = e;
}

Material::Material(MaterialType t, const Vector3f &e, const Vector3f &albedo, float roughness, float metallic) {
    m_type = t;
    m_emission = e;
    this->albedo = albedo;
    this->roughness = roughness;
    this->metallic = metallic;
    Vector3f base(0.04);
    F0 = lerp(base, albedo, metallic);
}

MaterialType Material::getType(){return m_type;}
///Vector3f Material::getColor(){return m_color;}
Vector3f Material::getEmission() {return m_emission;}
bool Material::hasEmission() {
    if (m_emission.norm() > EPSILON) return true;
    else return false;
}

Vector3f Material::getColorAt(double u, double v) {
    return Vector3f();
}


Vector3f Material::sample(const Vector3f &wi, const Vector3f &N){
    switch(m_type){
        case MICROFACET:
        {
            return importanceSampling(wi, N);
        }
        case DIFFUSE:
        {
            // uniform sample on the hemisphere
            float x_1 = get_random_float(), x_2 = get_random_float();
            float z = std::fabs(1.0f - 2.0f * x_1);
            float r = std::sqrt(1.0f - z * z), phi = 2 * M_PI * x_2;
            Vector3f localRay(r*std::cos(phi), r*std::sin(phi), z);
            return toWorld(localRay, N);
            
            break;
        }
    }
}

float Material::pdf(const Vector3f &wi, const Vector3f &wo, const Vector3f &N){
    switch(m_type){
        case MICROFACET:
        {
            return std::max((double)0.0001f, importancePDF(wo, wi, N));
        }
        case DIFFUSE:
        {
            // uniform sample probability 1 / (2 * PI)
            if (dotProduct(wo, N) > 0.0f)
                return 0.5f / M_PI;
            else
                return 0.0f;
            break;
        }
    }
}

Vector3f Material::eval(const Vector3f &wi, const Vector3f &wo, const Vector3f &N){
    switch(m_type){
        case DIFFUSE:
        {
            // calculate the contribution of diffuse   model
            float cosalpha = dotProduct(N, wo);
            if (cosalpha > 0.0f) {
                Vector3f diffuse = albedo / M_PI;
                return diffuse;
            }
            else
                return Vector3f(0.0f);
            break;
        }
        case MICROFACET:
        {
            double cos1 = std::max(dotProduct(N, wo), 0.0f);
            double cos2 = std::max(dotProduct(N, wi), 0.0f);
            if (cos1 > 0.0f && cos2 > 0.0f) {
                Vector3f h = (wi + wo).normalized();
                double k = pow((roughness + 1.0f), 2) / 8.0f;
                double distribute = distributionGGX(N, h, roughness);
                double geometry = geometrySmith(N, wo, wi, k);
                
                Vector3f fresnel = fresnelSchlick(cos1, F0);
                Vector3f Ks = fresnel;
                Vector3f Kd = (Vector3f(1.0f) - Ks) * (1.0f - metallic);
                return Kd * albedo / M_PI + Ks * distribute * geometry / std::max((double)0.0001f, (4.0f * cos1 * cos2));
            }
            else
                return Vector3f(0.0f);
            break;
        }
    }
}

#endif //RAYTRACING_MATERIAL_H
