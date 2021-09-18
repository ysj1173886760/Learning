#include "Renderer.hpp"
#include "Scene.hpp"
#include "Triangle.hpp"
#include "Sphere.hpp"
#include "Vector.hpp"
#include "global.hpp"
#include <chrono>

// In the main function of the program, we create the scene (create objects and
// lights) as well as set the options for the render (image width and height,
// maximum recursion depth, field-of-view, etc.). We then call the render
// function().
int main(int argc, char** argv)
{

    // Change the definition here to change resolution
    Scene scene(1080, 1080);

    Material* light = new Material(MICROFACET, (8.0f * Vector3f(0.747f+0.058f, 0.747f+0.258f, 0.747f) + 15.6f * Vector3f(0.740f+0.287f,0.740f+0.160f,0.740f) + 18.4f *Vector3f(0.737f+0.642f,0.737f+0.159f,0.737f)));
    light->albedo = Vector3f(0.65f);
    
    // https://github.com/UnderSilence/ComputerGraphicsLearning/blob/master/lectures/GAMES101/Assignment7/Assignment7/main.cpp
    Material* red_plastic = new Material(MICROFACET, Vector3f(0), Vector3f(1.0f, 0.05f, 0.04f), 0.8, 0);
    Material* white_plastic = new Material(MICROFACET, Vector3f(0), Vector3f(0.875f, 0.81f, 0.78f), 0.8, 0);
    Material* white_marble = new Material(MICROFACET, Vector3f(0), Vector3f(0.875f, 0.83f, 0.82f), 0.001, 0);
    Material* green_plastic = new Material(MICROFACET, Vector3f(0), Vector3f(0.14f, 1.0f, 0.091f), 0.8, 0);
    Material* copper = new Material(MICROFACET, Vector3f(0), Vector3f(0.95f, 0.64f, 0.54f), 0.1, 1.0);
    Material* silver = new Material(MICROFACET, Vector3f(0), Vector3f(0.95f, 0.93f, 0.88f), 0.01, 1.0);
    Material* gold = new Material(MICROFACET, Vector3f(0), Vector3f(1.00f, 0.71f, 0.29f), 0.0001, 1.0);
    Material* test = new Material(MICROFACET, Vector3f(0), Vector3f(0.95f, 0.95f, 0.95f), 0.0001, 1.0);

    MeshTriangle floor("../models/cornellbox/floor.obj", white_marble);
    MeshTriangle shortbox("../models/cornellbox/shortbox.obj", white_marble);
    MeshTriangle tallbox("../models/cornellbox/tallbox.obj", silver);
    MeshTriangle left("../models/cornellbox/left.obj", red_plastic);
    MeshTriangle right("../models/cornellbox/right.obj", green_plastic);
    MeshTriangle light_("../models/cornellbox/light.obj", light);
    MeshTriangle bunny("../models/bunny/bunny_big.obj", copper);
    Sphere ball1(Vector3f(138, 120, 380), 120, gold);
    Sphere ball2(Vector3f(400, 120, 212), 120, silver);
    Sphere ball3(Vector3f(250, 180, 100), 30, light);

    scene.Add(&floor);
    // scene.Add(&shortbox);
    scene.Add(&bunny);
    scene.Add(&ball1);
    // scene.Add(&ball2);
    scene.Add(&tallbox);
    scene.Add(&left);
    scene.Add(&right);
    scene.Add(&light_);

    scene.buildBVH();

    Renderer r;

    auto start = std::chrono::system_clock::now();
    r.Render(scene);
    auto stop = std::chrono::system_clock::now();

    std::cout << "Render complete: \n";
    std::cout << "Time taken: " << std::chrono::duration_cast<std::chrono::hours>(stop - start).count() << " hours\n";
    std::cout << "          : " << std::chrono::duration_cast<std::chrono::minutes>(stop - start).count() << " minutes\n";
    std::cout << "          : " << std::chrono::duration_cast<std::chrono::seconds>(stop - start).count() << " seconds\n";

    return 0;
}