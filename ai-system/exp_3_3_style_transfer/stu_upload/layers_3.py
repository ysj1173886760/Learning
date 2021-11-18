# coding:utf-8
import numpy as np
import struct
import os
import scipy.io
import time

def computeMse(data1,data2):
    errors = []
    for i in range(len(data1)):
        errors.append(data1[i]-data2[i])

    squared_error = []
    for val in errors:
        squared_error.append(pow(val, 2))
    
    return sum(squared_error) / len(squared_error)

class ContentLossLayer(object):
    def __init__(self):
        print('\tContent loss layer.')
    def forward(self, input_layer, content_layer):
         # TODO： 计算风格迁移图像和目标内容图像的内容损失
        # loss = np.dot((input_layer - content_layer).flatten(), (input_layer - content_layer).flatten().T) / (2 * input_layer.shape[0] * input_layer.shape[1] * input_layer.shape[2] * input_layer.shape[3])
        loss = np.square(input_layer - content_layer).sum() / (2 * input_layer.shape[0] * input_layer.shape[1] * input_layer.shape[2] * input_layer.shape[3])
        return loss
    def backward(self, input_layer, content_layer):
        # TODO： 计算内容损失的反向传播
        bottom_diff = (input_layer - content_layer) / (input_layer.shape[0] * input_layer.shape[1] * input_layer.shape[2] * input_layer.shape[3])
        return bottom_diff

class StyleLossLayer(object):
    def __init__(self):
        print('\tStyle loss layer.')
    def forward(self, input_layer, style_layer):
        # TODO： 计算风格迁移图像和目标风格图像的Gram 矩阵
        style_layer_reshape = np.reshape(style_layer, [style_layer.shape[0], style_layer.shape[1], -1])
        # [N, C, H, W]
        self.gram_style = np.matmul(style_layer_reshape, np.transpose(style_layer_reshape, [0, 2, 1]))

        self.input_layer_reshape = np.reshape(input_layer, [input_layer.shape[0], input_layer.shape[1], -1])
        self.gram_input = np.matmul(self.input_layer_reshape, np.transpose(self.input_layer_reshape, [0, 2, 1]))

        M = input_layer.shape[2] * input_layer.shape[3]
        N = input_layer.shape[1]
        self.div = M * M * N * N
        # TODO： 计算风格迁移图像和目标风格图像的风格损失
        style_diff = np.square(self.gram_style - self.gram_input).sum() / (input_layer.shape[0] * self.div * 4)
        loss = style_diff.sum()
        return loss
    def backward(self, input_layer, style_layer):
        return (np.matmul(np.transpose(self.gram_input - self.gram_style, [0, 2, 1]), self.input_layer_reshape) / (input_layer.shape[0] * self.div)).reshape(input_layer.shape)

