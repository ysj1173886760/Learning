# coding=utf-8
import tensorflow as tf, pdb

WEIGHTS_INIT_STDEV = .1

def net(image, type=0):
    # 该函数构建图像转换网络，image 为步骤 1 中读入的图像 ndarray 阵列，返回最后一层的输出结果
    # TODO：构建图像转换网络，每一层的输出作为下一层的输入
    conv1 = _conv_layer(image, 32, 9, 1, True, type)
    conv2 = _conv_layer(conv1, 64, 3, 2, True, type)
    conv3 = _conv_layer(conv2, 128, 3, 2, True, type)
    res1 = _residual_block(conv3, 3, type)
    res2 = _residual_block(res1, 3, type)
    res3 = _residual_block(res2, 3, type)
    res4 = _residual_block(res3, 3, type)
    res5 = _residual_block(res4, 3, type)
    tran_conv1 = _conv_tranpose_layer(res5, 64, 3, 2, type)
    tran_conv2 = _conv_tranpose_layer(tran_conv1, 32, 3, 2, type)
    conv4 = _conv_layer(tran_conv2, 3, 9, 1, False)

    #TODO：最后一个卷积层的输出再经过 tanh 函数处理，最后的输出张量 preds 像素值需限定在 [0,255] 范围内
    # [-1, 1] -> [0, 2] -> [0, 255]
    # preds = tf.nn.tanh(conv4) * 150 * (255./2)
    preds = (tf.nn.tanh(conv4) + 1) * (255. / 2.)
    return preds

def _conv_layer(net, num_filters, filter_size, strides, relu=True, type=0):
    # 该函数定义了卷积层的计算方法，net 为该卷积层的输入 ndarray 数组，num_filters 表示输出通道数，filter_size 表示卷积核尺
    # 寸，strides 表示卷积步长，该函数最后返回卷积层计算的结果

    # TODO：准备好权重的初值
    weights_init = _conv_init_vars(net, num_filters, filter_size)

    # TODO：输入的 strides 参数为标量，需将其处理成卷积函数能够使用的数据形式
    strides_shape = [1, strides, strides, 1]

    # TODO：进行卷积计算
    net = tf.nn.conv2d(net, weights_init, strides_shape, 'SAME')

    # 对卷积计算结果进行批归一化处理
    if type == 0:
        net = _batch_norm(net)
    elif type == 1:
        net = _instance_norm(net)

    if relu:
        # TODO：对归一化结果进行 ReLU 操作
        net = tf.nn.relu(net)

    return net

def _conv_tranpose_layer(net, num_filters, filter_size, strides, type=0):
    # TODO：准备好权重的初值
    weights_init = _conv_init_vars(net, num_filters, filter_size, True)

    # TODO：输入的 num_filters、strides 参数为标量，需将其处理成转置卷积函数能够使用的数据形式
    batch_size, h, w, _ = [i for i in net.get_shape()]
    new_h = strides * h
    new_w = strides * w
    strides_shape = [1, strides, strides, 1]

    # TODO：进行转置卷积计算
    net = tf.nn.conv2d_transpose(net, weights_init, (batch_size, new_h, new_w, num_filters), strides_shape, 'SAME')
    # 对卷积计算结果进行批归一化处理
    if type == 0:
        net = _batch_norm(net)
    elif type == 1:
        net = _instance_norm(net)
    
    # TODO：对归一化结果进行 ReLU 操作
    net = tf.nn.relu(net)

    return net

def _residual_block(net, filter_size=3, type=0):
    # TODO：调用之前实现的卷积层函数，实现残差块的计算
    return net + _conv_layer(_conv_layer(net, 128, filter_size, 1, True, type), 128, filter_size, 1, False, type)

def _batch_norm(net, train=True):
    # 整体都做norm
    batch, rows, cols, channels = [i.value for i in net.get_shape()]
    axes=list(range(len(net.get_shape())-1))
    mu, sigma_sq = tf.nn.moments(net, axes, keep_dims=True)
    var_shape = [channels]
    shift = tf.Variable(tf.zeros(var_shape))
    scale = tf.Variable(tf.ones(var_shape))
    epsilon = 1e-3
    return tf.nn.batch_normalization(net, mu, sigma_sq, shift, scale, epsilon)

def _instance_norm(net, train=True):
    # 对每一个通道做norm，不同通道不关联
    batch, rows, cols, channels = [i.value for i in net.get_shape()]
    var_shape = [channels]
    mu, sigma_sq = tf.nn.moments(net, [1,2], keep_dims=True)
    shift = tf.Variable(tf.zeros(var_shape))
    scale = tf.Variable(tf.ones(var_shape))
    epsilon = 1e-3
    normalized = (net-mu)/(sigma_sq + epsilon)**(.5)
    return scale * normalized + shift

def _conv_init_vars(net, out_channels, filter_size, transpose=False):
    _, rows, cols, in_channels = [i.value for i in net.get_shape()]
    if not transpose:
        weights_shape = [filter_size, filter_size, in_channels, out_channels]
    else:
        weights_shape = [filter_size, filter_size, out_channels, in_channels]

    weights_init = tf.Variable(tf.truncated_normal(weights_shape, stddev=WEIGHTS_INIT_STDEV, seed=1), dtype=tf.float32)
    return weights_init
