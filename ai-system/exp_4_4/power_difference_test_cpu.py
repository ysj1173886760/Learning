# coding=utf-8
import numpy as np
import os
import time
os.environ['MLU_VISIBLE_DEVICES']=""
import tensorflow as tf
np.set_printoptions(suppress=True)
from power_diff_numpy import *

def power_difference_op(input_x,input_y,input_pow):
    with tf.Session() as sess:
        # TODO：完成TensorFlow接口调用
        x = tf.placeholder(tf.float32, input_x.shape)
        y = tf.placeholder(tf.float32, input_y.shape)
        z = tf.placeholder(tf.float32, input_pow.shape)
        out = tf.power_difference(x, y, z)
        return sess.run(out, feed_dict = {x: input_x, y: input_y, z: input_pow})

def main():
    value = 256
 
    start = time.time()
    input_x = np.loadtxt("../data/in_x.txt")
    input_y = np.loadtxt("../data/in_y.txt")
    input_pow = np.loadtxt("../data/in_z.txt")
    output = np.loadtxt("../data/out.txt")
    end = time.time()
    print("load data cost " + str((end - start)*1000) + "ms")

    input_x = np.reshape(input_x,(1,value,value,3))
    input_y = np.reshape(input_y,(1,1,1,3))
    output = np.reshape(output, (-1))

    start = time.time()
    res = power_difference_op(input_x, input_y, input_pow)
    end = time.time() - start
    print("comput C++ op cost "+ str(end*1000) + "ms" )
    res = np.reshape(res,(-1))
    err = sum(abs(res - output))/sum(output)
    print("C++ op err rate= " + str(err*100))

    start = time.time()
    res = power_diff_numpy(input_x, input_y, input_pow)
    end = time.time()
    print("comput numpy op cost "+ str((end-start)*1000) + "ms")
    res = np.reshape(res,(-1))
    err = sum(abs(res - output))/sum(output)
    print("numpy op err rate= "+ str(err*100))


if __name__ == '__main__':
    main()
