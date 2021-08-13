#include <random>

class RandomGenerator {
public:
    RandomGenerator(): dev(), rng(dev()), dist(0.f, 1.f) {}
    float get_random_number() {
        return dist(rng);
    }

private:
    std::random_device dev;
    std::mt19937 rng;
    std::uniform_real_distribution<float> dist;

};