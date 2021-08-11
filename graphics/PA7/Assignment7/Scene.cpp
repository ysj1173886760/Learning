//
// Created by Göksu Güvendiren on 2019-05-14.
//

#include "Scene.hpp"


void Scene::buildBVH() {
    printf(" - Generating BVH...\n\n");
    this->bvh = new BVHAccel(objects, 1, BVHAccel::SplitMethod::NAIVE);
}

Intersection Scene::intersect(const Ray &ray) const
{
    return this->bvh->Intersect(ray);
}

void Scene::sampleLight(Intersection &pos, float &pdf) const
{
    float emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
        }
    }
    float p = get_random_float() * emit_area_sum;
    emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
            if (p <= emit_area_sum){
                objects[k]->Sample(pos, pdf);
                break;
            }
        }
    }
}

bool Scene::trace(
        const Ray &ray,
        const std::vector<Object*> &objects,
        float &tNear, uint32_t &index, Object **hitObject)
{
    *hitObject = nullptr;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        float tNearK = kInfinity;
        uint32_t indexK;
        Vector2f uvK;
        if (objects[k]->intersect(ray, tNearK, indexK) && tNearK < tNear) {
            *hitObject = objects[k];
            tNear = tNearK;
            index = indexK;
        }
    }


    return (*hitObject != nullptr);
}

// Implementation of Path Tracing
Vector3f Scene::castRay(const Ray &ray, int depth) const
{
    if (depth > maxDepth)
        return Vector3f(0.0, 0.0, 0.0);

    Intersection intec = intersect(ray);
    Vector3f hitColor = this->backgroundColor;
    Material *m = intec.m;
    Vector3f normal = intec.normal;

    if (intec.happened) {
        Intersection pos;
        float pdf;
        auto L_dir = Vector3f(0.0, 0.0, 0.0);
        auto L_indir = Vector3f(0.0, 0.0, 0.0);
        sampleLight(pos, pdf);

        auto ws = (pos.coords - intec.coords).normalized();
        bool notBlocked = (intersect(Ray(intec.coords, ws)).coords - pos.coords).norm() < EPSILON;
        auto dist = pow((pos.coords - intec.coords).norm(), 2);

        if (notBlocked) {
            L_dir = pos.emit * m->eval(ray.direction, ws, normal) * dotProduct(ws, normal) * dotProduct(-ws, pos.normal) / pdf / dist;
        }

        if (get_random_float() <= RussianRoulette) {
            auto wi = m->sample(ray.direction, normal);

            Ray ray2(intec.coords, wi);
            Intersection pos2 = intersect(ray2);

            if (pos2.happened) {
                L_indir = castRay(ray2, depth + 1) * m->eval(ray.direction, wi, normal) * dotProduct(wi, normal) / m->pdf(ray.direction, wi, normal) / RussianRoulette;
            }
        }

        hitColor = L_dir + L_indir;
    }

    return hitColor;
    // TO DO Implement Path Tracing Algorithm here
}