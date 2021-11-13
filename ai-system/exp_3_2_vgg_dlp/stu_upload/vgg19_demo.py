# coding:utf-8
import pycnml
import time
import numpy as np
import os
import scipy.io

class VGG19(object):
    def __init__(self):
        # set up net
        self.net = pycnml.CnmlNet(16)
        self.input_quant_params = []
        self.filter_quant_params = []

    def build_model(self, 
                    param_path='../../imagenet-vgg-verydeep-19.mat', 
                    quant_param_path='../../vgg19_quant_param_new.npz'):
        self.param_path = param_path

        # loading quant params
        # before creating layers, you should run through the net with cpu and get positions and scales used for quantizing input data. you can get quant params by using pycnml.QuantTool
        # only conv and mlp layer need to be quantized
        # in this experiment these quant params have already been created and saved into local files. 
        params = np.load(quant_param_path)
        input_params = params['input']
        filter_params = params['filter']
        for i in range(0, len(input_params), 2):
            self.input_quant_params.append(pycnml.QuantParam(int(input_params[i]), float(input_params[i+1])))
        for i in range(0, len(filter_params), 2):
            self.filter_quant_params.append(pycnml.QuantParam(int(filter_params[i]), float(filter_params[i+1])))
        
        # TODO: 使用net的createXXXLayer接口搭建VGG19网络
        # creating layers
        self.net.setInputShape(1, 3, 224, 224)
        # conv1_1
        self.net.createConvLayer('conv1_1', 64, 3, 1, 1, 1, self.input_quant_params[0])
        # relu1_1
        self.net.createReLuLayer('relu1_1')
        # conv1_2
        self.net.createConvLayer('conv1_2', 64, 3, 1, 1, 1, self.input_quant_params[1])
        # relu1_2
        self.net.createReLuLayer('relu1_2')
        self.net.createPoolingLayer('pool1', 2, 2)

        self.net.createConvLayer('conv2_1', 128, 3, 1, 1, 1, self.input_quant_params[2])
        self.net.createReLuLayer('relu2_1')
        self.net.createConvLayer('conv2_2', 128, 3, 1, 1, 1, self.input_quant_params[3])
        self.net.createReLuLayer('relu2_2')
        self.net.createPoolingLayer('pool2', 2, 2)

        self.net.createConvLayer('conv3_1', 256, 3, 1, 1, 1, self.input_quant_params[4])
        self.net.createReLuLayer('relu3_1')
        self.net.createConvLayer('conv3_2', 256, 3, 1, 1, 1, self.input_quant_params[5])
        self.net.createReLuLayer('relu3_2')
        self.net.createConvLayer('conv3_3', 256, 3, 1, 1, 1, self.input_quant_params[6])
        self.net.createReLuLayer('relu3_3')
        self.net.createConvLayer('conv3_4', 256, 3, 1, 1, 1, self.input_quant_params[7])
        self.net.createReLuLayer('relu3_4')
        self.net.createPoolingLayer('pool3', 2, 2)

        self.net.createConvLayer('conv4_1', 512, 3, 1, 1, 1, self.input_quant_params[8])
        self.net.createReLuLayer('relu4_1')
        self.net.createConvLayer('conv4_2', 512, 3, 1, 1, 1, self.input_quant_params[9])
        self.net.createReLuLayer('relu4_2')
        self.net.createConvLayer('conv4_3', 512, 3, 1, 1, 1, self.input_quant_params[10])
        self.net.createReLuLayer('relu4_3')
        self.net.createConvLayer('conv4_4', 512, 3, 1, 1, 1, self.input_quant_params[11])
        self.net.createReLuLayer('relu4_4')
        self.net.createPoolingLayer('pool4', 2, 2)

        self.net.createConvLayer('conv5_1', 512, 3, 1, 1, 1, self.input_quant_params[12])
        self.net.createReLuLayer('relu5_1')
        self.net.createConvLayer('conv5_2', 512, 3, 1, 1, 1, self.input_quant_params[13])
        self.net.createReLuLayer('relu5_2')
        self.net.createConvLayer('conv5_3', 512, 3, 1, 1, 1, self.input_quant_params[14])
        self.net.createReLuLayer('relu5_3')
        self.net.createConvLayer('conv5_4', 512, 3, 1, 1, 1, self.input_quant_params[15])
        self.net.createReLuLayer('relu5_4')
        self.net.createPoolingLayer('pool5', 2, 2)
        
        # flatten
        self.net.createFlattenLayer('flatten', [1, 512 * 7 * 7, 1, 1])

        self.net.createMlpLayer('fc6', 4096, self.input_quant_params[16])
        self.net.createReLuLayer('relu6')
        self.net.createMlpLayer('fc7', 4096, self.input_quant_params[17])
        self.net.createReLuLayer('relu7')
        # fc8
        self.net.createMlpLayer('fc8', 1000, self.input_quant_params[18])
        # softmax
        self.net.createSoftmaxLayer('softmax', 1)
    
    def load_model(self):
        # loading params ... 
        print('Loading parameters from file ' + self.param_path)
        params = scipy.io.loadmat(self.param_path)
        self.image_mean = params['normalization'][0][0][0]
        self.image_mean = np.mean(self.image_mean, axis=(0, 1))
        
        count = 0
        for idx in range(self.net.size()):
            if 'conv' in self.net.getLayerName(idx):
                weight, bias = params['layers'][0][idx][0][0][0][0]
                # TODO：调整权重形状
                # matconvnet: weights dim [height, width, in_channel, out_channel]
                # ours: weights dim [out_channel, in_channel, height, width]
                weight = np.transpose(weight, [3, 2, 0, 1]).flatten().astype(np.float)
                bias = bias.reshape(-1).astype(np.float)
                self.net.loadParams(idx, weight, bias, self.filter_quant_params[count])
                count += 1
            if 'fc' in self.net.getLayerName(idx):
                # Loading params may take quite a while. Please be patient.
                weight, bias = params['layers'][0][idx-1][0][0][0][0]
                weight = np.transpose(np.reshape(weight, (-1, weight.shape[-1])), [1, 0]).flatten().astype(np.float)
                bias = bias.reshape(-1).astype(np.float)
                self.net.loadParams(idx, weight, bias, self.filter_quant_params[count])
                count += 1

    def load_image(self, image_dir):
        # loading image
        self.image = image_dir
        image_mean = np.array([123.68, 116.779, 103.939])
        print('Loading and preprocessing image from ' + image_dir)
        input_image = scipy.misc.imread(image_dir)
        input_image = scipy.misc.imresize(input_image,[224,224,3])
        input_image = np.array(input_image).astype(np.float32)
        input_image -= image_mean
        input_image = np.reshape(input_image, [1]+list(input_image.shape))
        # input dim [N, channel, height, width]
        # TODO：调整输入数据的形状
        # input_image = _______________________
        input_data = np.moveaxis(input_image, 3, 1).flatten().astype(np.float)
        self.net.setInputData(input_data)

    def forward(self):
        return self.net.forward()
    
    def get_top5(self, label):
        start = time.time()
        self.forward()
        end = time.time()

        result = self.net.getOutputData()

        # loading labels
        labels = []
        with open('../synset_words.txt', 'r') as f:
            labels = f.readlines()

        # print results
        top1 = False
        top5 = False
        print('------ Top 5 of ' + self.image + ' ------')
        prob = sorted(list(result), reverse=True)[:6]
        if result.index(prob[0]) == label:
            top1 = True
        for i in range(5):
            top = prob[i]
            idx = result.index(top)
            if idx == label:
                top5 = True
            print('%f - '%top + labels[idx].strip())

        print('inference time: %f'%(end - start))
        return top1,top5
    
    def evaluate(self, file_list):
        top1_num = 0
        top5_num = 0
        total_num = 0

        start = time.time()
        with open(file_list, 'r') as f:
            file_list = f.readlines()
            total_num = len(file_list)
            for line in file_list:
                image = line.split()[0].strip()
                label = int(line.split()[1].strip())
                vgg.load_image(image)
                top1,top5 = vgg.get_top5(label)
                if top1 :
                    top1_num += 1
                if top5 :
                    top5_num += 1
        end = time.time()

        print('Global accuracy : ')
        print('accuracy1: %f (%d/%d) '%(float(top1_num)/float(total_num), top1_num, total_num))
        print('accuracy5: %f (%d/%d) '%(float(top5_num)/float(total_num), top5_num, total_num))
        print('Total execution time: %f'%(end - start))


if __name__ == '__main__':
    vgg = VGG19()
    vgg.build_model()
    vgg.load_model()
    vgg.evaluate('../file_list')
