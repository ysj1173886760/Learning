# coding=utf-8
import numpy as np
import struct
import os
import time
import threading

def show_matrix(mat, name):
    #print(name + str(mat.shape) + ' mean %f, std %f' % (mat.mean(), mat.std()))
    pass

def show_time(time, name):
    #print(name + str(time))
    pass


class ConvolutionalLayer(object):
    def __init__(self, kernel_size, channel_in, channel_out, padding, stride):
        # 卷积层的初始化
        self.kernel_size = kernel_size
        self.channel_in = channel_in
        self.channel_out = channel_out
        self.padding = padding
        self.stride = stride
        print('\tConvolutional layer with kernel size %d, input channel %d, output channel %d.' % (self.kernel_size, self.channel_in, self.channel_out))
    def init_param(self, std=0.01):  # 参数初始化
        self.weight = np.random.normal(loc=0.0, scale=std, size=(self.channel_in, self.kernel_size, self.kernel_size, self.channel_out))
        self.bias = np.zeros([self.channel_out])

    def job(self, x, y, width, batch):
        cur = x * width + y
        bias_x = x * self.stride
        bias_y = y * self.stride
        self.col[:, cur, :] = self.input_pad[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].reshape(batch, -1)

    def forward(self, input):  # 前向传播的计算
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
        workers = []
        for x in range(height_out):
            for y in range(width_out):
                bias_x = x * self.stride
                bias_y = y * self.stride
                self.col[:, cur, :] = self.input_pad[:, :, bias_x: bias_x + self.kernel_size, bias_y: bias_y + self.kernel_size].reshape(input.shape[0], -1)
                cur = cur + 1
                # workers.append(threading.Thread(target = self.job, args = (x, y, width_out, input.shape[0])))
                # workers[-1].start()

        # for worker in workers:
        #     worker.join()
        # print(col.shape, self.weight.reshape(-1, self.weight.shape[-1]).shape)
        output = np.matmul(self.col, self.weight.reshape(-1, self.weight.shape[-1])) + self.bias
        # print(output.shape)
        self.output = np.moveaxis(output.reshape(input.shape[0], height_out, width_out, self.channel_out), 3, 1)
        return self.output

    def load_param(self, weight, bias):  # 参数加载
        assert self.weight.shape == weight.shape, "{} {}".format(self.weight.shape, weight.shape)
        assert self.bias.shape == bias.shape
        self.weight = weight
        self.bias = bias

class MaxPoolingLayer(object):
    def __init__(self, kernel_size, stride):  # 最大池化层的初始化
        self.kernel_size = kernel_size
        self.stride = stride
        print('\tMax pooling layer with kernel size %d, stride %d.' % (self.kernel_size, self.stride))
    def forward(self, input):  # 前向传播的计算
        start_time = time.time()
        self.input = input # [N, C, H, W]
        self.max_index = np.zeros(self.input.shape)
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

        self.output = col.max(axis=3).reshape(input.shape[0], input.shape[1], height_out, width_out)
        return self.output

class FlattenLayer(object):
    def __init__(self, input_shape, output_shape):  # 扁平化层的初始化
        self.input_shape = input_shape
        self.output_shape = output_shape
        assert np.prod(self.input_shape) == np.prod(self.output_shape)
        print('\tFlatten layer with input shape %s, output shape %s.' % (str(self.input_shape), str(self.output_shape)))
    def forward(self, input):  # 前向传播的计算
        assert list(input.shape[1:]) == list(self.input_shape), "{} {}".format(input.shape[1:], self.input_shape)
        # matconvnet feature map dim: [N, height, width, channel]
        # ours feature map dim: [N, channel, height, width]
        # TODO：转换 input 维度顺序
        self.input = np.moveaxis(input, 1, 3)
        # print(self.output_shape)
        # print([self.input.shape[0]] + list(self.output_shape))
        self.output = self.input.reshape([self.input.shape[0]] + list(self.output_shape))
        show_matrix(self.output, 'flatten out ')
        return self.output
