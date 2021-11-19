# coding=utf-8
import numpy as np
import struct
import os
import time

def computeMse(data1,data2):
    errors = []
    for i in range(len(data1)):
        errors.append(data1[i]-data2[i])

    squared_error = []
    for val in errors:
        squared_error.append(pow(val, 2))
    
    return sum(squared_error) / len(squared_error)

def get_indices(X_shape, HF, WF, stride, pad):
    """
        Returns index matrices in order to transform our input image into a matrix.

        Parameters:
        -X_shape: Input image shape.
        -HF: filter height.
        -WF: filter width.
        -stride: stride value.
        -pad: padding value.

        Returns:
        -i: matrix of index i.
        -j: matrix of index j.
        -d: matrix of index d. 
            (Use to mark delimitation for each channel
            during multi-dimensional arrays indexing).
    """
    # get input size
    m, n_C, n_H, n_W = X_shape

    # get output size
    out_h = int((n_H + 2 * pad - HF) / stride) + 1
    out_w = int((n_W + 2 * pad - WF) / stride) + 1
  
    # ----Compute matrix of index i----

    # Level 1 vector.
    level1 = np.repeat(np.arange(HF), WF)
    # Duplicate for the other channels.
    level1 = np.tile(level1, n_C)
    # Create a vector with an increase by 1 at each level.
    everyLevels = stride * np.repeat(np.arange(out_h), out_w)
    # Create matrix of index i at every levels for each channel.
    i = level1.reshape(-1, 1) + everyLevels.reshape(1, -1)

    # ----Compute matrix of index j----
    
    # Slide 1 vector.
    slide1 = np.tile(np.arange(WF), HF)
    # Duplicate for the other channels.
    slide1 = np.tile(slide1, n_C)
    # Create a vector with an increase by 1 at each slide.
    everySlides = stride * np.tile(np.arange(out_w), out_h)
    # Create matrix of index j at every slides for each channel.
    j = slide1.reshape(-1, 1) + everySlides.reshape(1, -1)

    # ----Compute matrix of index d----

    # This is to mark delimitation for each channel
    # during multi-dimensional arrays indexing.
    d = np.repeat(np.arange(n_C), HF * WF).reshape(1, -1)

    i = np.transpose(i, [1, 0])
    j = np.transpose(j, [1, 0])
    # print(i.shape, j.shape)
    # print(X_shape, stride, pad)
    # print("i", i)
    # print("j", j)
    # print("d", d)
    return i, j, d

class ConvolutionalLayer(object):
    def __init__(self, kernel_size, channel_in, channel_out, padding, stride, type=1):
        self.kernel_size = kernel_size
        self.channel_in = channel_in
        self.channel_out = channel_out
        self.padding = padding
        self.stride = stride
        self.forward = self.forward_raw
        self.backward = self.backward_raw
        if type == 1:
            self.forward = self.forward_speedup
            self.backward = self.backward_speedup
        print('\tConvolutional layer with kernel size %d, input channel %d, output channel %d.' % (self.kernel_size, self.channel_in, self.channel_out))
    def init_param(self, std=0.01):
        self.weight = np.random.normal(loc=0.0, scale=std, size=(self.channel_in, self.kernel_size, self.kernel_size, self.channel_out))
        self.bias = np.zeros([self.channel_out])
    def forward_raw(self, input):
        start_time = time.time()
        self.input = input # [N, C, H, W]
        height = self.input.shape[2] + self.padding * 2
        width = self.input.shape[3] + self.padding * 2
        self.input_pad = np.zeros([self.input.shape[0], self.input.shape[1], height, width])
        self.input_pad[:, :, self.padding:self.padding+self.input.shape[2], self.padding:self.padding+self.input.shape[3]] = self.input
        height_out = (height - self.kernel_size) / self.stride + 1
        width_out = (width - self.kernel_size) / self.stride + 1
        self.output = np.zeros([self.input.shape[0], self.channel_out, height_out, width_out])
        for idxn in range(self.input.shape[0]):
            for idxc in range(self.channel_out):
                for idxh in range(height_out):
                    for idxw in range(width_out):
                        bias_x = idxh * self.stride
                        bias_y = idxw * self.stride
                        self.output[idxn, idxc, idxh, idxw] = np.dot(self.input_pad[idxn, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].flatten(), self.weight[:, :, :, idxc].flatten()) + self.bias[idxc]
        self.forward_time = time.time() - start_time
        return self.output

    def forward_speedup(self, input):
        # TODO: 改进forward函数，使得计算加速
        start_time = time.time()
        self.input = input # [N, C, H, W]
        # TODO: 边界扩充
        height = input.shape[2] + 2 * self.padding
        width = input.shape[3] + 2 * self.padding
        self.input_pad = np.zeros([self.input.shape[0], self.input.shape[1], height, width])
        self.input_pad[:, :, self.padding: self.padding + input.shape[2], self.padding: self.padding + input.shape[3]] = self.input
        height_out = (height - self.kernel_size) / self.stride + 1
        width_out = (width - self.kernel_size) / self.stride + 1
        mat_w = self.kernel_size * self.kernel_size * self.channel_in
        mat_h = height_out * width_out

        self.col = np.empty((input.shape[0], mat_h, mat_w))
        cur = 0
        for x in range(height_out):
            for y in range(width_out):
                bias_x = x * self.stride
                bias_y = y * self.stride
                self.col[:, cur, :] = self.input_pad[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].reshape(input.shape[0], -1)
                cur = cur + 1
        output = np.matmul(self.col, self.weight.reshape(-1, self.weight.shape[-1])) + self.bias
        self.output = np.moveaxis(output.reshape(input.shape[0], height_out, width_out, self.channel_out), 3, 1)

        # self.col = np.empty((input.shape[0], mat_h, mat_w))
        # i, j, d = get_indices(self.input.shape, self.kernel_size, self.kernel_size, self.stride, self.padding)

        # self.col = self.input_pad[:, d, i, j]

        # output = np.matmul(self.col, self.weight.reshape(-1, self.weight.shape[-1])) + self.bias
        # self.output = np.moveaxis(output.reshape(input.shape[0], height_out, width_out, self.channel_out), 3, 1)

        self.forward_time = time.time() - start_time
        return self.output

    def backward_speedup(self, top_diff):
        # TODO: 改进backward函数，使得计算加速
        # top_diff batch, cout, h, w
        start_time = time.time()

        height_out = (self.input.shape[2] + 2 * self.padding - self.kernel_size) / self.stride + 1
        width_out = (self.input.shape[3] + 2 * self.padding - self.kernel_size) / self.stride + 1

        # cout, batch, h, w
        top_diff_col = np.transpose(top_diff, [1, 0, 2, 3]).reshape(top_diff.shape[1], -1)
        # self.col batch, (h * w), (cin * k * k)

        # what we want, cin, k, k, cout
        tmp = np.transpose(self.col.reshape(-1, self.col.shape[-1]), [1, 0])
        self.d_weight = np.matmul(tmp, top_diff_col.T).reshape(self.channel_in, self.kernel_size, self.kernel_size, self.channel_out)
        self.d_bias = top_diff_col.sum(axis=1)
        
        backward_col = np.empty((top_diff.shape[0], self.input.shape[2] * self.input.shape[3], self.kernel_size * self.kernel_size * self.channel_out))
        pad_height = ((self.input.shape[2] - 1) * self.stride + self.kernel_size - height_out) / 2
        pad_width = ((self.input.shape[3] - 1) * self.stride + self.kernel_size - width_out) / 2
        top_diff_pad = np.zeros((top_diff.shape[0], top_diff.shape[1], height_out + 2 * pad_height, width_out + 2 * pad_width))
        top_diff_pad[:, :, pad_height: height_out + pad_height, pad_width: width_out + pad_width] = top_diff
        cur = 0
        for x in range(self.input.shape[2]):
            for y in range(self.input.shape[3]):
                bias_x = x * self.stride
                bias_y = y * self.stride
                backward_col[:, cur, :] = top_diff_pad[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].reshape(top_diff.shape[0], -1)
                cur = cur + 1

        # backward_col [batch, height * width, cout * k * k]
        # try to draw a draft and you will know the reason.
        # you shall consider the contribution from top_diff to the original dx
        # if x * kernel[i] has contribution to y, then dy * kernel[size - i] will have contribution
        weight_tmp = np.transpose(self.weight, [3, 1, 2, 0]).reshape(self.channel_out, -1, self.channel_in)[:, ::-1, :].reshape(-1, self.channel_in)
        bottom_diff = np.matmul(backward_col, weight_tmp)
        # [batch, height, width, cin]
        bottom_diff = np.transpose(bottom_diff.reshape(top_diff.shape[0], self.input.shape[2], self.input.shape[3], self.input.shape[1]), [0, 3, 1, 2])
        
        self.backward_time = time.time() - start_time
        return bottom_diff

    def backward_raw(self, top_diff):
        start_time = time.time()
        self.d_weight = np.zeros(self.weight.shape)
        self.d_bias = np.zeros(self.bias.shape)
        bottom_diff = np.zeros(self.input_pad.shape)
        for idxn in range(top_diff.shape[0]):
            for idxc in range(top_diff.shape[1]):
                for idxh in range(top_diff.shape[2]):
                    for idxw in range(top_diff.shape[3]):
                        # TODO： 计算卷积层的反向传播， 权重、偏置的梯度和本层损失
                        bias_x = idxh * self.stride
                        bias_y = idxw * self.stride
                        self.d_weight[:, :, :, idxc] += top_diff[idxn, idxc, idxh, idxw] * self.input_pad[idxn, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size]
                        self.d_bias[idxc] += top_diff[idxn, idxc, idxh, idxw]
                        bottom_diff[idxn, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size] += top_diff[idxn, idxc, idxh, idxw] * self.weight[:, :, :, idxc]
        bottom_diff = bottom_diff[:, :, self.padding: self.input_pad.shape[2] - self.padding, self.padding: self.input_pad.shape[3] - self.padding]
        self.backward_time = time.time() - start_time
        return bottom_diff

    def get_gradient(self):
        return self.d_weight, self.d_bias
    def update_param(self, lr):
        self.weight += - lr * self.d_weight
        self.bias += - lr * self.d_bias
    def load_param(self, weight, bias):
        assert self.weight.shape == weight.shape
        assert self.bias.shape == bias.shape
        self.weight = weight
        self.bias = bias
    def get_forward_time(self):
        return self.forward_time
    def get_backward_time(self):
        return self.backward_time

class MaxPoolingLayer(object):
    def __init__(self, kernel_size, stride, type=1):
        self.kernel_size = kernel_size
        self.stride = stride
        self.forward = self.forward_raw
        self.backward = self.backward_raw_book
        if type == 1:  # type 设为 1 时，使用优化后的 foward 和 backward 函数
            self.forward = self.forward_speedup
            self.backward = self.backward_speedup
        print('\tMax pooling layer with kernel size %d, stride %d.' % (self.kernel_size, self.stride))
    def forward_raw(self, input):
        self.input = input # [N, C, H, W]
        height_out = (self.input.shape[2] - self.kernel_size) / self.stride + 1
        width_out = (self.input.shape[3] - self.kernel_size) / self.stride + 1
        self.output = np.zeros([self.input.shape[0], self.input.shape[1], height_out, width_out])
        for idxn in range(self.input.shape[0]):
            for idxc in range(self.input.shape[1]):
                for idxh in range(height_out):
                    for idxw in range(width_out):
                        # TODO： 计算最大池化层的前向传播， 取池化窗口内的最大值
                        bias_x = idxh * self.stride
                        bias_y = idxw * self.stride
                        self.output[idxn, idxc, idxh, idxw] = np.max(self.input[idxn, idxc, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size])
        return self.output

    def forward_speedup(self, input):
        # TODO: 改进forward函数，使得计算加速
        self.input = input # [N, C, H, W]
        height_out = (self.input.shape[2] - self.kernel_size) / self.stride + 1
        width_out = (self.input.shape[3] - self.kernel_size) / self.stride + 1
        mat_w = self.kernel_size * self.kernel_size
        mat_h = height_out * width_out

        col = np.empty((input.shape[0], self.input.shape[1], mat_h, mat_w))
        cur = 0
        for x in range(height_out):
            for y in range(width_out):
                bias_x = x * self.stride
                bias_y = y * self.stride
                col[:, :, cur, :] = self.input[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].reshape(input.shape[0], input.shape[1], -1)
                cur = cur + 1

        self.output = np.max(col, axis=3, keepdims=True)
        max_index = np.argmax(col.reshape(input.shape[0], input.shape[1], height_out, width_out, self.kernel_size * self.kernel_size), axis=4)
        self.max_elements = np.zeros((input.shape[0], self.input.shape[1], height_out, width_out, self.kernel_size * self.kernel_size))
        # https://stackoverflow.com/questions/44143438/numpy-indexing-set-1-to-max-value-and-zeros-to-all-others
        # refering the advanced indexing in numpy
        n, c, h, w = self.max_elements.shape[: 4]
        N, C, H, W = np.ogrid[:n, :c, :h, :w]
        self.max_elements[N, C, H, W, max_index] = 1
        self.output = self.output.reshape(input.shape[0], input.shape[1], height_out, width_out)
        return self.output

    def backward_speedup(self, top_diff):
        # TODO: 改进backward函数，使得计算加速
        bottom_diff = np.zeros(self.input.shape)
        contrib = self.max_elements * (top_diff.reshape(list(top_diff.shape) + [1]))
        for x in range(top_diff.shape[2]):
            for y in range(top_diff.shape[3]):
                bias_x = x * self.stride
                bias_y = y * self.stride
                bottom_diff[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size] += contrib[:, :, x, y, :].reshape(top_diff.shape[0], top_diff.shape[1], self.kernel_size, self.kernel_size)
        return bottom_diff

    def backward_raw_book(self, top_diff):
        bottom_diff = np.zeros(self.input.shape)
        for idxn in range(top_diff.shape[0]):
            for idxc in range(top_diff.shape[1]):
                for idxh in range(top_diff.shape[2]):
                    for idxw in range(top_diff.shape[3]):
                        # TODO: 最大池化层的反向传播， 计算池化窗口中最大值位置， 并传递损失
                        bias_x = idxh * self.stride
                        bias_y = idxw * self.stride
                        max_index = np.unravel_index(np.argmax(self.input[idxn, idxc, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size]), [self.kernel_size, self.kernel_size])
                        bottom_diff[idxn, idxc, bias_x + max_index[0], bias_y + max_index[1]] = top_diff[idxn, idxc, idxh, idxw]
        return bottom_diff

class FlattenLayer(object):
    def __init__(self, input_shape, output_shape):
        self.input_shape = input_shape
        self.output_shape = output_shape
        assert np.prod(self.input_shape) == np.prod(self.output_shape)
        print('\tFlatten layer with input shape %s, output shape %s.' % (str(self.input_shape), str(self.output_shape)))
    def forward(self, input):
        assert list(input.shape[1:]) == list(self.input_shape)
        # matconvnet feature map dim: [N, height, width, channel]
        # ours feature map dim: [N, channel, height, width]
        self.input = np.transpose(input, [0, 2, 3, 1])
        self.output = self.input.reshape([self.input.shape[0]] + list(self.output_shape))
        return self.output
    def backward(self, top_diff):
        assert list(top_diff.shape[1:]) == list(self.output_shape)
        top_diff = np.transpose(top_diff, [0, 3, 1, 2])
        bottom_diff = top_diff.reshape([top_diff.shape[0]] + list(self.input_shape))
        return bottom_diff
