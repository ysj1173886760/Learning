# coding=utf-8
import numpy as np

def power_diff_numpy(input_x,input_y,input_z):
    # TODO:完成numpy实现的过程，参考实验教程示例
    x_shape = np.shape(input_x)
    y_shape = np.shape(input_y)
    x = np.reshape(input_x, (-1, y_shape[-1]))
    x_new_shape = np.shape(x)
    y = np.reshape(input_y, (-1))
    output = []
    for i in range(x_new_shape[0]):
        output.append(np.power(x[i] - y, input_z))
    return np.array(output)

