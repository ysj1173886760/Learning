/*************************************************************************
 * Copyright (C) [2019] by Cambricon, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *************************************************************************/
#include <float.h>
#include <math.h>
#include <memory.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <vector>
#include "cnrt.h"
#include "gemm16Kernel.h"

#define PAD_UP(x, m) ((x + m - 1) / m * m)
#define MP_SELECT 16
#define MP1 ((MP_SELECT & 1))
#define MP4 ((MP_SELECT & 4))
#define MP8 ((MP_SELECT & 8))
#define MP16 ((MP_SELECT & 16))
#define MP32 ((MP_SELECT & 32))

//int Mlu_gemm(float *A, const float *B, float *C, int M, int N, int K) {
int Mlu_gemm(int8_t *A, int8_t *B, float *C, int32_t M, int32_t N, int32_t K,
    int16_t pos1, int16_t pos2, float scale1, float scale2,float &return_time) {
  struct timeval start;
  struct timeval end;
  float time_use;
  int N_align = N;
//  int N_align = PAD_UP(N, 16);
//  int K_align = PAD_UP(K, 64);
  cnrtRet_t ret;
  gettimeofday(&start, NULL);

  cnrtQueue_t pQueue;
  CNRT_CHECK(cnrtCreateQueue(&pQueue));

  cnrtDim3_t dim;   
  cnrtFunctionType_t func_type = CNRT_FUNC_TYPE_BLOCK;   // CNRT_FUNC_TYPE_BLOCK=1  
  dim.x = 1;
  dim.y = 1;
  dim.z = 1;


  if (MP1) {
    dim.x = 1;
    func_type = CNRT_FUNC_TYPE_BLOCK;
  } else if (MP4) {
    dim.x = 4;
    func_type = CNRT_FUNC_TYPE_UNION1;
    //printf("UNION1!\n");
  } else if (MP8) {
    dim.x = 8;
    func_type = CNRT_FUNC_TYPE_UNION2;
  } else if (MP16) {  
    dim.x = 16;
    func_type = CNRT_FUNC_TYPE_UNION4;
    //printf("16\n");              
  } else if (MP32) {
    dim.x = 32;
    func_type = CNRT_FUNC_TYPE_UNION8;
  } else {
    //printf("MP select is wrong! val = %d, use default setting,mp=1\n",
    //       MP_SELECT);
    // return -1;
  }

  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
  //printf("cnrt init time use %f ms\n", time_use);

  gettimeofday(&start, NULL);
  float *h_f32b = (float *)malloc(K * sizeof(float));
  half *h_c = (half *)malloc(M * N_align * sizeof(half));
  // float* h_a =(float*)malloc(M * K_align * sizeof(float));
  //half *h_w = (half *)malloc(K_align * N_align * sizeof(half));
  //memset(h_w, 0, sizeof(half) * K_align * N_align);
//  int8_t *h_w = (int8_t *)malloc(K_align * N_align * sizeof(int8_t));
//  memset(h_w, 0, sizeof(int8_t) * K_align * N_align);
#if 0
  half *h_w_reshape = (half *)malloc(K_align * N_align * sizeof(half));
  half *h_b = (half *)malloc(K_align * sizeof(half));
  for (int j = 0; j < K; j++) {
    h_f32b[j] = 0.0;
    CNRT_CHECK(cnrtConvertFloatToHalf(&h_b[j], h_f32b[j]));
    for (int i = 0; i < N; i++) {
      CNRT_CHECK(cnrtConvertFloatToHalf(&h_w[i * K_align + j],
                                        B[j * N + i]));  // transpose
    }
  }
#endif
#if 0
  for (int i =0; i < K * N; i++)
  {
    if (i % N == 0) //printf("\n");
    //printf("%.1f ", B[i]);
  }
#endif
  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
  //printf("convert data time use %f ms\n", time_use);
  gettimeofday(&start, NULL);
#if 0
#if __BANG_ARCH__ == 100
  int Tn = N_align / 256;
  int Ren = N_align % 256;
  for (int i = 0; i < Tn; i++) {
    CNRT_CHECK(cnrtFilterReshape(h_w_reshape + i * 256 * K_align,
                                 h_w + i * 256 * K_align, 256, K_align, 1, 1,
                                 CNRT_FLOAT16));
  }
  if (Ren != 0) {
    CNRT_CHECK(cnrtFilterReshape(h_w_reshape + Tn * 256 * K_align,
                                 h_w + Tn * 256 * K_align, Ren, K_align, 1, 1,
                                 CNRT_FLOAT16));
  }
#else
  int Tn = N_align / 1024;
  int Ren = N_align % 1024;
  for (int i = 0; i < Tn; i++) {
    CNRT_CHECK(cnrtFilterReshape(h_w_reshape + i * 1024 * K_align,
                                 h_w + i * 1024 * K_align, 1024, K_align, 1, 1,
                                 CNRT_FLOAT16));
  }
  if (Ren != 0) {
    CNRT_CHECK(cnrtFilterReshape(h_w_reshape + Tn * 1024 * K_align,
                                 h_w + Tn * 1024 * K_align, Ren, K_align, 1, 1,
                                 CNRT_FLOAT16));
  }
#endif
#endif
  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
  //printf("reshape filter time use %f ms\n", time_use);

  half *d_c = NULL;
  int8_t *d_a = NULL;
  int8_t *d_w = NULL;
  int16_t pos = pos1 + pos2;

  gettimeofday(&start, NULL);
  
  //在mlu上为输入输出开辟空间
  CNRT_CHECK(cnrtMalloc((void **)&d_c, sizeof(half) * M * N_align));
  CNRT_CHECK(cnrtMalloc((void **)&d_a, sizeof(int8_t) * M * K));
  CNRT_CHECK(cnrtMalloc((void **)&d_w, sizeof(int8_t) * K * N_align));

  //将cpu上的输入拷贝给mlu上的输入
  CNRT_CHECK(cnrtMemcpy(d_a, A, sizeof(int8_t) * M * K, CNRT_MEM_TRANS_DIR_HOST2DEV));
  CNRT_CHECK(cnrtMemcpy(d_w, B, sizeof(int8_t) * K * N_align, CNRT_MEM_TRANS_DIR_HOST2DEV));

  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
  //printf("malloc &copyin time use %f ms\n", time_use);

  cnrtKernelParamsBuffer_t params;
  CNRT_CHECK(cnrtGetKernelParamsBuffer(&params));     // Gets a parameter buffer for cnrtInvokeKernel_V2 or cnrtInvokeKernel_V3. 
  //向params添加参数
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &d_c, sizeof(half *)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &d_a, sizeof(int8_t *)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &d_w, sizeof(int8_t *)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &M, sizeof(uint32_t)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &K, sizeof(uint32_t)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &N_align, sizeof(uint32_t)));
  CNRT_CHECK(cnrtKernelParamsBufferAddParam(params, &pos, sizeof(uint16_t)));

 
  cnrtKernelInitParam_t init_param;
  CNRT_CHECK(cnrtCreateKernelInitParam(&init_param));
  CNRT_CHECK(cnrtInitKernelMemory((const void *)gemm16Kernel, init_param));

  cnrtNotifier_t notifier_start;   // A pointer which points to the struct describing notifier.
  cnrtNotifier_t notifier_end;
  CNRT_CHECK(cnrtCreateNotifier(&notifier_start));
  CNRT_CHECK(cnrtCreateNotifier(&notifier_end));
  float timeTotal = 0.0;

  //printf("start invoke  : \n");
  gettimeofday(&start, NULL);

  CNRT_CHECK(cnrtPlaceNotifier(notifier_start, pQueue));   // Places a notifier in specified queue

  // 启动激活函数
  CNRT_CHECK( cnrtInvokeKernel_V3((void *)&gemm16Kernel, init_param, dim, params, func_type, pQueue, NULL));   // Invokes a kernel written in Bang with given params on MLU
 
  CNRT_CHECK(cnrtPlaceNotifier(notifier_end, pQueue));     // Places a notifier in specified queue

  CNRT_CHECK(cnrtSyncQueue(pQueue));   // Function should be blocked until all precedent tasks in the queue are completed.  同步Queue
  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
//  //printf("invoke  time use %f ms\n", time_use);
//  cnrtNotifierElapsedTime(notifier_start, notifier_end, &timeTotal);
// get the duration time between notifer_start and notifer_end.
  // cnrtNotifierDuration(notifier_start, notifier_end, &timeTotal);    // Gets duration time of two makers
  CNRT_CHECK(cnrtNotifierDuration(notifier_start, notifier_end, &timeTotal));
  return_time = timeTotal / 1000.0;   
  //printf("hardware total Time: %.3f ms\n", return_time);
  gettimeofday(&start, NULL);

  // 将输出拷回cpu
  CNRT_CHECK(cnrtMemcpy(h_c, d_c, sizeof(half) * M * N_align, 
                        CNRT_MEM_TRANS_DIR_DEV2HOST));
  for (int j = 0; j < M; j++) {
    for (int i = 0; i < N; i++) {
      CNRT_CHECK(cnrtConvertHalfToFloat(&C[j * N + i], h_c[j * N_align + i]));
      C[j * N + i] = C[j * N + i]/(scale1 * scale2);
    }
  }
  gettimeofday(&end, NULL);
  time_use =
      ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_usec - start.tv_usec)) /
      1000.0;
  //printf("copyout &convert time use %f ms\n", time_use);

// free 
  CNRT_CHECK(cnrtFree(d_c));
  CNRT_CHECK(cnrtFree(d_a));
  CNRT_CHECK(cnrtFree(d_w));

  CNRT_CHECK(cnrtDestroyQueue(pQueue));
  CNRT_CHECK(cnrtDestroyKernelParamsBuffer(params));
  CNRT_CHECK(cnrtDestroyNotifier(&notifier_start));
  CNRT_CHECK(cnrtDestroyNotifier(&notifier_end));
  free(h_f32b);
  free(h_c);
  //free(h_w);
  //free(h_w_reshape);
  return 0;
}
