# coding=utf-8
import numpy as np
import struct
import os
import scipy.io
import time
import tensorflow as tf
from tensorflow.python.framework import graph_util

os.putenv('MLU_VISIBLE_DEVICES','0')

IMAGE_PATH = 'data/cat1.jpg'
VGG_PATH = 'stu_upload/vgg19_int8.pb'

def preprocess(image,mean):
    return image - mean

def load_image(path):
    image = scipy.misc.imread(path)
    image = scipy.misc.imresize(image, (224,224,3))
    mean = np.array([123.68, 116.779, 103.939])
    image = np.array([preprocess(image, mean)]).astype(np.float32)
    image = np.reshape(image, (1, 224, 224, 3))
    return image

if __name__ == '__main__':
    input_image = load_image(IMAGE_PATH)

    g = tf.Graph()

    # setting mlu configurations
    config = tf.ConfigProto(allow_soft_placement=True,
                    inter_op_parallelism_threads=1,
                    intra_op_parallelism_threads=1)
    config.mlu_options.data_parallelism = 1
    config.mlu_options.model_parallelism = 1
    config.mlu_options.core_num = 16 # 1 4 16
    config.mlu_options.core_version = "MLU270"
    config.mlu_options.precision = "int8"
    config.mlu_options.save_offline_model = False

    model = VGG_PATH

    with g.as_default():
        with tf.gfile.FastGFile(model,'rb') as f:
            graph_def = tf.GraphDef()
            graph_def.ParseFromString(f.read())
            tf.import_graph_def(graph_def, name='')

        with tf.Session(config=config) as sess:
            sess.run(tf.global_variables_initializer())
            input_tensor = sess.graph.get_tensor_by_name('img_placeholder:0')
            output_tensor = sess.graph.get_tensor_by_name('Softmax:0')
            
            for i in range(10):
                start = time.time()
                # TODO: 计算 output_tensor
                preds = sess.run(output_tensor, feed_dict={input_tensor: input_image})
                end = time.time()
                delta_time = end - start	
                print("Inference (MLU) processing time: %s" % delta_time)

            prob = preds[0]
            top1 = np.argmax(prob)

            print('Classification result: id = %d, prob = %f' % (top1, prob[top1]))


    
