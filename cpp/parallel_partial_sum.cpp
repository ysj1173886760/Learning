#include <iostream>
#include <thread>
#include <chrono>
#include <vector>

const int maxx = 1e8;
std::vector<int> n;
std::vector<int> res1;
std::vector<int> res2;

void job1(int start, int end) {
    res1[start] = n[start];
    for (int i = start + 1; i < end; i++) {
        res1[i] = res1[i - 1] + n[i];
    }
}

void job2(int start, int end, int base) {
    for (int i = start; i < end - 1; i++) {
        res1[i] += base;
    }
}

void work() {
    int numThreads = 10;
    int rangePerThread = maxx / numThreads;
    std::vector<std::thread> workers;
    workers.reserve(numThreads);
    for (int i = 0; i < numThreads; i++) {
        int start = i * rangePerThread;
        int end = (i + 1) * rangePerThread;
        workers.push_back(std::thread(job1, start, end));
    }
    for (auto &worker : workers) {
        worker.join();
    }
    for (int i = 1; i < numThreads; i++) {
        res1[(i + 1) * rangePerThread - 1] += res1[i * rangePerThread - 1];
    }
    workers.clear();
    for (int i = 1; i < numThreads; i++) {
        int start = i * rangePerThread;
        int end = start + rangePerThread;
        workers.push_back(std::thread(job2, start, end, res1[i * rangePerThread - 1]));
    }
    for (auto &worker : workers) {
        worker.join();
    }
}

int main() {
    // init
    n.resize(maxx);
    res1.resize(maxx);
    res2.resize(maxx);

    for (int i = 0; i < maxx; i++) {
        n[i] = i;
    }

    auto start = std::chrono::high_resolution_clock::now();
    work();
    auto end = std::chrono::high_resolution_clock::now();

    std::chrono::duration<double> dur = end - start;
    std::chrono::milliseconds d = std::chrono::duration_cast<std::chrono::milliseconds>(dur);
    std::cout << "parallel: " << d.count() << "ms\n";

    start = std::chrono::high_resolution_clock::now();
    res2[0] = n[0];
    for (int i = 1; i < maxx; i++) {
        res2[i] = res2[i - 1] + n[i];
    }
    end = std::chrono::high_resolution_clock::now();

    dur = end - start;
    d = std::chrono::duration_cast<std::chrono::milliseconds>(dur);
    std::cout << "sequential: " << d.count() << "ms\n";

    bool flag = true;
    std::cout << "sanity check ";
    for (int i = 0; i < maxx; i++) {
        if (res1[i] != res2[i]) {
            std::cout << "failed on " << i << std::endl;
            flag = false;
            break;
        }
    }
    if (flag) {
        std::cout << "success" << std::endl;
    }

    return 0;
}