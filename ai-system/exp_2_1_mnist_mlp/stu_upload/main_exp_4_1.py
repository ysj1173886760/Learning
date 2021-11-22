import numpy as np
import struct
import os
import scipy.io
import time
import tensorflow as tf
from tensorflow.python.framework import graph_util
from stu_upload.evaluate_cpu import *

def computeMse(data1,data2):
    errors = []
    for i in range(len(data1)):
        errors.append(data1[i]-data2[i])

    squared_error = []
    for val in errors:
        squared_error.append(pow(val, 2))

    return sum(squared_error) / len(squared_error)

def test_cpu():
    input_image = load_image(IMAGE_PATH)
    standard_image = np.load('standard_image.npy')

    load_image_mse = computeMse(input_image.flatten(), standard_image.flatten())
    print('load image mse: %f'%load_image_mse)

    # if load_image_mse < 0.003:
    #     print('TEST LOAD IMAGE PASS.')
    # else:
    #     print('TEST LOAD IMAGE FAILED.')
    #     exit()

    print('------------------------------------')

    with tf.Session() as sess:
        img_placeholder = tf.placeholder(tf.float32, shape=(1,224,224,3),
                                         name='img_placeholder')
        nets = net(VGG_PATH, img_placeholder)
        for i in range(1):
            start = time.time()
            preds = sess.run(nets, feed_dict={img_placeholder:input_image})
            end = time.time()
            delta_time = end - start	
            print("processing time: %s" % delta_time)

        conv1_1 = preds['conv1_1']
        standard_conv1_1 = np.load('standard_conv1_1.npy')
        # for idx in range(standard_conv1_1.shape[3]):
        #     scipy.misc.imsave('./pic/conv{}.jpg'.format(idx), standard_conv1_1[0, :, :, idx])
        # for idx in range(standard_conv1_1.shape[3]):
        #     print(idx, computeMse(conv1_1[0, :, :, idx].flatten(), standard_conv1_1[0, :, :, idx].flatten()))
        #     scipy.misc.imsave('./pic/my{}.jpg'.format(idx), conv1_1[0, :, :, idx])
        conv_mse = computeMse(conv1_1.flatten(), standard_conv1_1.flatten())
        print('conv mse: %f'%conv_mse)

        # if conv_mse < 0.003:
        #     print('TEST CONV PASS.')
        # else:
        #     print('TEST CONV FAILED.')
        #     exit()

        pool1 = preds['pool1']
        standard_pool1 = np.load('standard_pool1.npy')
        # for idx in range(standard_pool1.shape[3]):
        #     scipy.misc.imsave('./pic/pool{}.jpg'.format(idx), standard_pool1[0, :, :, idx])
        pool_mse = computeMse(pool1.flatten(), standard_pool1.flatten())
        print('pool mse: %f'%pool_mse)

        # if pool_mse < 0.003:
        #     print('TEST POOL PASS.')
        # else:
        #     print('TEST POOL FAILED.')
        #     exit()

        prob = preds['softmax'][0]
        top1 = np.argmax(prob)

        print('Classification result: id = %d, prob = %f' % (top1, prob[top1]))
        if top1 == 281:
            print('TEST RESULT PASS.')
        else:
            print('TEST RESULT FAILED.')
            exit()

def test_mlu():
    os.system('./run_mlu.sh')

if __name__ == '__main__':
    test_cpu()
    print('------------------------------------')
    test_mlu()