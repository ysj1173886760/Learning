//
// Created by goksu on 2/25/20.
//

#include <fstream>
#include <thread>
#include "Scene.hpp"
#include "Renderer.hpp"


inline float deg2rad(const float& deg) { return deg * M_PI / 180.0; }

const float EPSILON = 0.00001;
int renderProgress;
std::mutex mutex;

static void RenderThread(const Scene &scene, std::vector<Vector3f> &frameBuffer, int yStart, int yEnd, int spp) {

    float scale = tan(deg2rad(scene.fov * 0.5));
    float imageAspectRatio = scene.width / (float)scene.height;
    Vector3f eye_pos(278, 273, -800);
    int m = 0;

    // change the spp value to change sample ammount
    for (uint32_t j = yStart; j < yEnd; ++j) {
        for (uint32_t i = 0; i < scene.width; ++i) {
            // generate primary ray direction
            for (int k = 0; k < spp; k++){
                float x = (2 * (i + get_random_float()) / (float)scene.width - 1) *
                        imageAspectRatio * scale;
                float y = (1 - 2 * (j + get_random_float()) / (float)scene.height) * scale;

                Vector3f dir = normalize(Vector3f(-x, y, 1));
                frameBuffer[j * scene.height + i] += scene.castRay(Ray(eye_pos, dir), 0) / spp;  
            }
            // if (frameBuffer[j * scene.height + i].x > 1)
            //     std::cout << frameBuffer[j * scene.height + i].x << std::endl;
            // if (frameBuffer[j * scene.height + i].y > 1)
            //     std::cout << frameBuffer[j * scene.height + i].y << std::endl;
            // if (frameBuffer[j * scene.height + i].z > 1)
            //     std::cout << frameBuffer[j * scene.height + i].z << std::endl;
        }
        mutex.lock();
        renderProgress++;
        UpdateProgress(renderProgress / (float)scene.height);
        mutex.unlock();
    }
}

// The main render function. This where we iterate over all pixels in the image,
// generate primary rays and cast these rays into the scene. The content of the
// framebuffer is saved to a file.
void Renderer::Render(const Scene& scene)
{
    std::vector<Vector3f> framebuffer(scene.width * scene.height);
    renderProgress = 0;

    int spp = 64;
    std::cout << "SPP: " << spp << "\n";

    // referring https://github.com/Quanwei1992/GAMES101/blob/master/07/Renderer.cpp
    mutex.unlock();
    int numThreads = std::thread::hardware_concurrency();
    int lineEveryThread = scene.height / numThreads + (scene.height % numThreads > 0);
    std::cout << "Threads: " << numThreads << std::endl;

    std::vector<std::thread> workers;
    for (int i = 0; i < numThreads; i++) {
        int yStart = i * lineEveryThread;
        int yEnd = std::min(yStart + lineEveryThread, scene.height);
        std::cout << "Thread id: " << i << " start " << yStart << " end " << yEnd << std::endl;
        workers.push_back(std::thread(RenderThread, std::ref(scene), std::ref(framebuffer), yStart, yEnd, spp));
    }

    for (auto &x : workers) {
        x.join();
    }

    UpdateProgress(1.f);

    // save framebuffer to file
    FILE* fp = fopen("binary.ppm", "wb");
    (void)fprintf(fp, "P6\n%d %d\n255\n", scene.width, scene.height);
    for (auto i = 0; i < scene.height * scene.width; ++i) {
        static unsigned char color[3];
        color[0] = (unsigned char)(255 * std::pow(clamp(0, 1, framebuffer[i].x), 0.6f));
        color[1] = (unsigned char)(255 * std::pow(clamp(0, 1, framebuffer[i].y), 0.6f));
        color[2] = (unsigned char)(255 * std::pow(clamp(0, 1, framebuffer[i].z), 0.6f));
        fwrite(color, 1, 3, fp);
    }
    fclose(fp);    
}
