/* Copyright 2015 The TensorFlow Authors. All Rights Reserved.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#ifndef TENSORFLOW_CORE_KERNELS_CWISE_OP_POWER_DIFFERENCE_MLU_H_
#define TENSORFLOW_CORE_KERNELS_CWISE_OP_POWER_DIFFERENCE_MLU_H_
#if CAMBRICON_MLU
#include <string>
#include "tensorflow/core/kernels/cwise_ops_common.h"
#include "tensorflow/core/kernels/cwise_ops.h"
#include "tensorflow/core/framework/register_types.h"
#include "tensorflow/core/framework/mlu_op_kernel.h"
#include "tensorflow/stream_executor/mlu/mlu_stream.h"
namespace tensorflow {
template <typename T>
class MLUPowerDifferenceOp : public MLUOpKernel {
 public:
  explicit MLUPowerDifferenceOp(OpKernelConstruction* ctx) :
          MLUOpKernel(ctx) {}

  void ComputeOnMLU(OpKernelContext* ctx) override {
    //auto* stream = ctx->op_device_context()->mlu_stream();
    //auto* mlustream_exec = ctx->op_device_context()->mlu_stream()->parent();
    se::mlu::MLUStream* stream = static_cast<se::mlu::MLUStream*>(
        ctx->op_device_context()->stream()->implementation());
    const Tensor& a = ctx->input(0);
    const Tensor& b = ctx->input(1);
    const Tensor& c = ctx->input(2);
    string op_parameter = ctx->op_kernel().type_string()
                          + "/" + a.shape().DebugString()
                          + "/" + b.shape().DebugString();
    //MLU_OP_CHECK_UNSUPPORTED(mlustream_exec, op_parameter, ctx);
    const string op = ctx->op_kernel().type_string();
    const int dims_a = a.dims();
    const int dims_b = b.dims();
    // get the output shape
    TensorShape shape;
    int max_dims = dims_a >= dims_b ? dims_a : dims_b;
    for (int i = 0; i < max_dims; i++) {
      // if the dim exist in one of the input tensor, set the dim size of
      // the tensor to the output shape
      // e.g. tensora: n h w c   tensorb: w c   output: (n h) w c
      if (max_dims - i > dims_a) {
        shape.InsertDim(i, b.dim_size(i));
      } else if (max_dims - i > dims_b) {
        shape.InsertDim(i, a.dim_size(i));
      } else {
        // if the dim exist in both of the input tensor, set the bigger dim size
        // to the output shape
        int idx_a = dims_b >= dims_a ? (i + dims_a - dims_b) : i;
        int idx_b = dims_a >= dims_b ? (i + dims_b - dims_a) : i;
        int dim_a = a.dim_size(idx_a);
        int dim_b = b.dim_size(idx_b);
        // if the dim size of input tensors not equal and
        // nor of them equal to 1, input tensors is invalid
        OP_REQUIRES(ctx, dim_a == dim_b || dim_a == 1 || dim_b == 1,
            errors::InvalidArgument(
              "PowerDifferenceOp: Not supported tensor shape, input1: ",
              a.shape().DebugString(),
              ", input2: ", b.shape().DebugString()));
        int min_dim = dim_a < dim_b ? dim_a : dim_b;
        if (min_dim == 0) {
          shape.InsertDim(i, 0);
        } else {
          shape.InsertDim(i, dim_a > dim_b ? dim_a : dim_b);
        }
      }
    }

    //bool power_check = (c.dims() == 1 && c.dim_size(0) == 1) || c.dims() == 0;
    bool power_check = (c.dims() == 1) || c.dims() == 0;
    OP_REQUIRES(ctx, power_check, errors::InvalidArgument("power should be [1] or scalar"));

//    MLULOG(3) << "chengxinchao test: "
//              << "input3";
//     
//    LOG(INFO) << "a debugstring =" << &a;
//    LOG(INFO) << "c debugstring =" << &c;
//    int power_value_ = static_cast<int>(c.flat<Eigen::half>()(0));
//    //MLULOG(3) << "chengxinchao input1: "
//    //          << "input3: " << power_value;
//    LOG(INFO) << "power value cast =" << power_value_;
    //auto input_pow = c.SummarizeValue(1,true);//flat<T>();
    //LOG(INFO) << "power NumElements value =" << c.NumElements();
    //LOG(INFO) << "power summarize value =" << input_pow;
    //const int power_value = input_pow;//(0);
    
    //int power_value = 3;
    //float pow_c_value = static_cast<float>(a.flat<Eigen::half>()(0));
    //LOG(INFO) << "power value float =" << pow_c_value;
    //int tmp = (int)pow_c_value;
    int power_value = c.dim_size(0);
    //LOG(INFO) << "power value =" << power_value;

    Tensor* output = nullptr;
    OP_REQUIRES_OK(ctx, ctx->allocate_output(0, shape, &output));
    //if (output->NumElements() > 0) {
    //  OP_REQUIRES_OK(ctx, stream->PowerDifference(ctx,
    //        const_cast<Tensor *>(&a), const_cast<Tensor *>(&b), output, power_value));
    //  // MLU_OP_CHECK_STATUS(status, mlustream_exec, op_parameter, ctx, a, b);
    //} else {
    //  mlustream_exec->insert_unsupported_op(ctx, op_parameter);
    //}
    OP_REQUIRES_OK(ctx, stream->PowerDifference(ctx,
          const_cast<Tensor *>(&a), const_cast<Tensor *>(&b), output, power_value));
  }
};

}  // namespace tensorflow

#endif  // CAMBRICON_MLU
#endif  // TENSORFLOW_CORE_KERNELS_CWISE_OP_SQUARED_DIFFERENCE_MLU_H_
