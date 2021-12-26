#include <time.h>
#include <stdio.h>
#include <pthread.h>
#include <stdlib.h>

#define NUM_OF_THREADS 12
#define WORK_LOAD 100000000

int counter1[NUM_OF_THREADS];

struct counter {
    int counter;
    char padding[60];
} counter2[NUM_OF_THREADS];

void *worker(void *arg) {
    int *addr = (int *)arg;
    for (int i = 0; i < WORK_LOAD; i++) {
        (*addr)++;
    }
}

void test1() {
    pthread_t workers[NUM_OF_THREADS];
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_create(&workers[i], NULL, worker, &(counter1[i]));
    }

    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_join(workers[i], NULL);
    }
}

void test2() {
    pthread_t workers[NUM_OF_THREADS];
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_create(&workers[i], NULL, worker, &(counter2[i].counter));
    }

    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_join(workers[i], NULL);
    }
}

int main() {
    struct timespec end, start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    test1();
    clock_gettime(CLOCK_MONOTONIC, &end);
    double elapsed = (end.tv_sec - start.tv_sec);
    elapsed += (end.tv_nsec - start.tv_nsec) / 1000000000.0;
    printf("test1 elapsed time %.2fs\n", elapsed);

    clock_gettime(CLOCK_MONOTONIC, &start);
    test2();
    clock_gettime(CLOCK_MONOTONIC, &end);
    elapsed = (end.tv_sec - start.tv_sec);
    elapsed += (end.tv_nsec - start.tv_nsec) / 1000000000.0;
    printf("test2 elapsed time %.2fs\n", elapsed);

    return 0;
}


// clock_t start = clock();
// test1();
// clock_t end = clock();
// double time = ((double)(end - start)) / CLOCKS_PER_SEC;
// printf("test1 elapsed time %.2lfs\n", time);

// start = clock();
// test2();
// end = clock();
// time = ((double)(end - start)) / CLOCKS_PER_SEC;
// printf("test2 elapsed time %.2lfs\n", time);
