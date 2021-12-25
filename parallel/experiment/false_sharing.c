#include <time.h>
#include <stdio.h>
#include <pthread.h>
#include <stdlib.h>

#define NUM_OF_THREADS 12
#define WORK_LOAD 1000000000

int counter1[NUM_OF_THREADS];

struct counter {
    int counter;
    char padding[60];
} counter2[NUM_OF_THREADS];

void *worker1(void *args) {
    int worker_id = *((int *)args);
    for (int i = 0; i < WORK_LOAD; i++) {
        counter1[worker_id]++;
    }
}

void *worker2(void *args) {
    int worker_id = *((int *)args);
    for (int i = 0; i < WORK_LOAD; i++) {
        counter2[worker_id].counter++;
    }
}

void test1() {
    pthread_t workers[NUM_OF_THREADS];
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        int *param = malloc(sizeof(int));
        *param = i;
        pthread_create(&workers[i], NULL, worker1, param);
    }

    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_join(workers[i], NULL);
    }
}

void test2() {
    pthread_t workers[NUM_OF_THREADS];
    for (int i = 0; i < NUM_OF_THREADS; i++) {
        int *param = malloc(sizeof(int));
        *param = i;
        pthread_create(&workers[i], NULL, worker2, param);
    }

    for (int i = 0; i < NUM_OF_THREADS; i++) {
        pthread_join(workers[i], NULL);
    }
}

int main() {
    clock_t start = clock();
    test1();
    clock_t end = clock();
    double time = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("test1 elapsed time %.2lfs\n", time);

    start = clock();
    test2();
    end = clock();
    time = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("test2 elapsed time %.2lfs\n", time);
    return 0;
}

// struct timespec t1, t2;
// clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t1);
// test1();
// clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t2);
// double time = ((double)(t2.tv_sec - t1.tv_sec));
// printf("test1 elapsed time %.2lfs\n", time);

// clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t1);
// test2();
// clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t2);
// time = ((double)(t2.tv_sec - t1.tv_sec));
// printf("test2 elapsed time %.2lfs\n", time);


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
