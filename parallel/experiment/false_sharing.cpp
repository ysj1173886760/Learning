#include <thread>
#include <chrono>
#include <iostream>
#include <vector>

#define NUM_OF_THREADS 12
#define WORK_LOAD 1000000000

struct counter {
    int counter;
    char padding[60];
};

void worker(int *addr) {
    for (int i = 0; i < WORK_LOAD; i++) {
        (*addr)++;
    }
}

int counter1[NUM_OF_THREADS];
void test1() {
    std::vector<std::thread> workers;
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        workers.push_back(std::thread(worker, &(counter1[i])));
    }

    for (auto &x : workers) {
        x.join();
    }

}

counter counter2[NUM_OF_THREADS];
void test2() {
    std::vector<std::thread> workers;
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        workers.push_back(std::thread(worker, &(counter2[i].counter)));
    }

    for (auto &x : workers) {
        x.join();
    }

}

int main() {
    auto start = std::chrono::steady_clock::now();
    test1();
    auto end = std::chrono::steady_clock::now();
    std::chrono::duration<double> elapsed_seconds = end - start;
    std::cout << "test1 elapsed time: " << elapsed_seconds.count() << "s\n";

    start = std::chrono::steady_clock::now();
    test2();
    end = std::chrono::steady_clock::now();
    elapsed_seconds = end - start;
    std::cout << "test2 elapsed time: " << elapsed_seconds.count() << "s\n";
    return 0;
}