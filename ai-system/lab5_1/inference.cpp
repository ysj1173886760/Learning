#include "inference.h"
#include "cnrt.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "stdlib.h"
#include <sys/time.h>
#include <time.h>

namespace StyleTransfer{

Inference :: Inference(std::string offline_model){
    offline_model_ = offline_model;
}

typedef unsigned short half;

void cnrtConvertFloatToHalfArray(uint16_t* x, const float* y, int len) {
  for (int i = 0; i < len; i++){
    cnrtConvertFloatToHalf(x+i,y[i]);
  }
}

void cnrtConvertHalfToFloatArray(float* x, const uint16_t* y, int len) {
  for (int i = 0; i < len; i++){
    cnrtConvertHalfToFloat(x+i,y[i]);
  }
}

void cnrtConvertFloatToHalfArray(uint16_t* x, float* y, int len) {
  for (int i = 0; i < len; i++){
    cnrtConvertFloatToHalf(x+i,y[i]);
  }
}

void cnrtConvertHalfToFloatArray(float* x, uint16_t* y, int len) {
  for (int i = 0; i < len; i++){
    cnrtConvertHalfToFloat(x+i,y[i]);
  }
}

void Inference :: run(DataTransfer* DataT){

    // referring https://www.cambricon.com/docs/cnrt/user_guide_html/example/offline_mode.html
    cnrtInit(0);

    // TODO:load model
    cnrtModel_t model;
    cnrtLoadModel(&model, offline_model_.c_str());

    // TODO:set current device
    cnrtDev_t dev;
    cnrtGetDeviceHandle(&dev, 0);
    cnrtSetCurrentDevice(dev);

    // TODO:load extract function
    cnrtFunction_t function;
    cnrtCreateFunction(&function);
    cnrtExtractFunction(&function, model, "subnet0");

    int inputNum, outputNum;
    int64_t *inputSizeS, *outputSizeS;
    cnrtGetInputDataSize(&inputSizeS, &inputNum, function);
    cnrtGetOutputDataSize(&outputSizeS, &outputNum, function);
    int size = 256 * 256 * 3;

    // inputNum is 1, inputSize is 256 * 256 * 3 * sizeof(half)
    void *inputCpuPtrS = (void *)malloc(size * sizeof(half));
    void *outputCpuPtrS = (void *)malloc(size * sizeof(half));

    // void *inputMluPtrS = (void *)malloc(size * sizeof(void *));
    // void *outputMluPtrS = (void *)malloc(size * sizeof(void *));
    void *inputMluPtrS;
    void *outputMluPtrS;
    cnrtMalloc(&inputMluPtrS, size * sizeof(half));
    cnrtMalloc(&outputMluPtrS, size * sizeof(half));

    float* input_data = (float *)(malloc(256 * 256 * 3 * sizeof(float)));
    // C H W -> H W C
    for (int i = 0; i < 256 * 256; i++) {
        for (int j = 0; j < 3; j++) {
            input_data[i * 3 + j] = DataT->input_data[256 * 256 * j + i];
        }
    }

    cnrtConvertFloatToHalfArray((half *)inputCpuPtrS, input_data, 256 * 256 * 3);

    cnrtMemcpy(inputMluPtrS, inputCpuPtrS, size * sizeof(half), CNRT_MEM_TRANS_DIR_HOST2DEV);

    void **param = (void **)malloc(sizeof(void *) * 2);
    param[0] = inputMluPtrS;
    param[1] = outputMluPtrS;

    // setup runtime ctx
    cnrtRuntimeContext_t ctx;
    cnrtCreateRuntimeContext(&ctx, function, NULL);

    // bind device
    cnrtSetRuntimeContextDeviceId(ctx, 0);
    cnrtInitRuntimeContext(ctx, NULL);

    // compute offline
    cnrtQueue_t queue;
    cnrtRuntimeContextCreateQueue(ctx, &queue);

    // invoke
    cnrtInvokeRuntimeContext(ctx, param, queue, NULL);

    // sync
    cnrtSyncQueue(queue);

    cnrtMemcpy(outputCpuPtrS, outputMluPtrS, size * sizeof(half), CNRT_MEM_TRANS_DIR_DEV2HOST);

    float* output_data = (float *)(malloc(256 * 256 * 3 * sizeof(float)));
    DataT->output_data = (float *)(malloc(256 * 256 * 3 * sizeof(float)));
    cnrtConvertHalfToFloatArray(output_data, (half *)outputCpuPtrS, 256 * 256 * 3);
    // H W C -> C H W
    for (int i = 0; i < 256 * 256; i++) {
        for (int j = 0; j < 3; j++) {
            DataT->output_data[256 * 256 * j + i] = output_data[i * 3 + j];
        }
    }

    // free memory space
    cnrtFree(inputMluPtrS);
    cnrtFree(outputMluPtrS);
    free(inputCpuPtrS);
    free(outputCpuPtrS);
    free(param);

    cnrtDestroyQueue(queue);
    cnrtDestroyRuntimeContext(ctx);
    cnrtDestroyFunction(function);
    cnrtUnloadModel(model);
    cnrtDestroy();
}

} // namespace StyleTransfer
