/* Copyright 2016 The TensorFlow Authors. All Rights Reserved.

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

#include "tensorflow/core/kernels/cwise_ops_common.h"

#include "tensorflow/core/kernels/broadcast_to_op.h"
#include "tensorflow/core/util/bcast.h"
#include "third_party/eigen3/unsupported/Eigen/CXX11/Tensor"
#include "tensorflow/core/framework/op.h"
//#include "tensorflow/core/framework/shape_inference.h"
#include "tensorflow/core/framework/op_kernel.h"
#if CAMBRICON_MLU
// #include "tensorflow/core/kernels/cwise_op_power_difference_mlu.h"
#endif  // CAMBRICON_MLU
namespace tensorflow {

#if CAMBRICON_MLU
// #define REGISTER_MLU(T)                                         \
//   REGISTER_KERNEL_BUILDER(                                      \
//       Name("PowerDifference")                                 \
//           .Device(DEVICE_MLU)                                   \
//           .TypeConstraint<T>("T"),                              \
//       MLUPowerDifferenceOp<T>);
// TF_CALL_MLU_FLOAT_TYPES(REGISTER_MLU);
#endif  // CAMBRICON_MLU

#if CAMBRICON_MLU
// REGISTER_KERNEL_BUILDER(Name("PowerDifference")
//                          .Device(DEVICE_MLU)
//                          .TypeConstraint<Eigen::half>("T")
//                          .HostMemory("y"),
//                        MLUPowerDifferenceOp<Eigen::half>);
#endif  // CAMBRICON_MLU

template <typename T>
class PowerDifferenceOp : public OpKernel {
  public:
    explicit PowerDifferenceOp(OpKernelConstruction* context)
      : OpKernel(context) {}

    void Compute(OpKernelContext* context) override {
      const Tensor& input_x_tensor = context->input(0);
      const Tensor& input_y_tensor = context->input(1);
      const Tensor& input_pow_tensor = context->input(2);

      const Eigen::ThreadPoolDevice& device = context->eigen_device<Eigen::ThreadPoolDevice>();

      BCast bcast(BCast::FromShape(input_y_tensor.shape()), BCast::FromShape(input_x_tensor.shape()),
                /*fewer_dims_optimization=*/true);

      Tensor* output_tensor = nullptr;
      TensorShape output_shape = BCast::ToShape(bcast.output_shape());

      OP_REQUIRES_OK(context,
                     context->allocate_output(0, output_shape, &output_tensor));
 
      Tensor input_x_broad(input_x_tensor.dtype(), output_shape);
      Tensor input_y_broad(input_y_tensor.dtype(), output_shape);

      OP_REQUIRES_OK(context,
                    context->allocate_temp(input_x_tensor.dtype(),
                                           output_shape,
                                            &input_x_broad));
      OP_REQUIRES_OK(context,
                    context->allocate_temp(input_y_tensor.dtype(),
                                           output_shape,
                                            &input_y_broad));
 
      if (input_x_tensor.IsSameSize(input_x_broad) == false) {
        functor::BroadcastTo<Eigen::ThreadPoolDevice, T>()(device, context, input_x_broad, output_shape,
                                      input_x_tensor, input_x_tensor.shape(), bcast);
      } else {
        input_x_broad = input_x_tensor;
      }
      if (input_y_tensor.IsSameSize(input_y_broad) == false) {
        functor::BroadcastTo<Eigen::ThreadPoolDevice, T>()(device, context, input_y_broad, output_shape,
                                      input_y_tensor, input_y_tensor.shape(), bcast);
       } else {
        input_y_broad = input_y_tensor;
      }

      auto input_x = input_x_broad.flat<T>();
      auto input_y = input_y_broad.flat<T>();
      auto input_pow = input_pow_tensor.flat<T>();
      auto output = output_tensor->flat<T>();

      const int N = input_x.size();
      const int POW = input_pow(0); 
      float tmp = 0;

      // TODO: 补全 power_diference 算子计算部分
      for (int i = 0; i < N; i++) {
        tmp = 1.0;
        int p = POW;
        float x = input_x(i) - input_y(i);
        while (p) {
          if (p & 1) {
            tmp *= x;
          }
          x *= x;
          p >>= 1;
        }
        output(i) = tmp;
      }
    }
};

REGISTER_KERNEL_BUILDER(Name("PowerDifference").Device(DEVICE_CPU), PowerDifferenceOp<float>);

}  // namespace tensorflow
