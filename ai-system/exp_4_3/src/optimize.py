# coding=utf-8
from __future__ import print_function
import functools
import vgg, pdb, time
import tensorflow as tf, numpy as np, os
import transform
from utils import get_img

STYLE_LAYERS = ('relu1_1', 'relu2_1', 'relu3_1', 'relu4_1', 'relu5_1')
CONTENT_LAYER = 'relu4_2'
DEVICES = '/cpu:0'  #'CUDA_VISIBLE_DEVICES'

os.putenv('MLU_VISIBLE_DEVICES','')

def loss_function(net, content_features, style_features, content_weight, style_weight, tv_weight, preds, batch_size):
    # 损失函数构建，net 为特征提取网络，content_features 为内容图像特征，style_features 为风格图像特征，content_weight、
    # style_weight 和 tv_weight 分别为特征重建损失、风格重建损失的权重和全变分正则化损失的权重

    batch_shape = (batch_size,256,256,3)

    # 计算内容损失
    # content_loss
    content_size = _tensor_size(content_features[CONTENT_LAYER])*batch_size
    assert _tensor_size(content_features[CONTENT_LAYER]) == _tensor_size(net[CONTENT_LAYER])
    content_loss = 2 * content_weight * tf.nn.l2_loss(net[CONTENT_LAYER] - content_features[CONTENT_LAYER]) / content_size

    # 计算风格损失
    # style_loss
    style_losses = []
    for style_layer in STYLE_LAYERS:
        layer = net[style_layer]
        bs, height, width, filters = map(lambda i:i.value,layer.get_shape())
        size = height * width * filters
        feats = tf.reshape(layer, (bs, height * width, filters))
        feats_T = tf.transpose(feats, perm=[0,2,1])
        grams = tf.matmul(feats_T, feats) / size
        style_gram = style_features[style_layer]
        # TODO: 计算 style_losses
        style_losses.append(2 * tf.nn.l2_loss(grams - style_gram) / size)
    style_loss = style_weight * functools.reduce(tf.add, style_losses) / batch_size

    # 使用全变分正则化方法定义损失函数 tv_loss
    # tv_loss
    tv_y_size = _tensor_size(preds[:,1:,:,:])
    tv_x_size = _tensor_size(preds[:,:,1:,:])
    # TODO：将图像 preds 向水平和垂直方向各平移一个像素，分别与原图相减，分别计算二者的 𝐿2 范数 x_tv 和 y_tv
    # Hint: use tf.nn.l2_loss
    y_tv = tf.nn.l2_loss(preds[:, 1:, :, :] - preds[:, :preds.shape[1] - 1, :, :])
    x_tv = tf.nn.l2_loss(preds[:, :, 1:, :] - preds[:, :, :preds.shape[2] - 1, :])
    tv_loss = tv_weight*2*(x_tv/tv_x_size + y_tv/tv_y_size)/batch_size

    loss = content_loss + style_loss + tv_loss
    return content_loss, style_loss, tv_loss, loss

     
     
#np arr, np arr
def optimize(content_targets, style_target, content_weight, style_weight,
                 tv_weight, vgg_path, epochs=2, print_iterations=1000,
                 batch_size=4, save_path='saver/fns.ckpt', slow=False,
                 learning_rate=1e-3, debug=False, type=0, save=True, load=True):
    # 实时风格迁移训练方法定义，content_targets 为内容图像, style_target 为风格图像, content_weight、style_weight 和 tv_weight 分别为
    # 特征重建损失、风格重建损失和全变分正则化项的权重，vgg_path 为保存 VGG19 网络参数的文件路径
    if slow:
        batch_size = 1
    mod = len(content_targets) % batch_size
    if mod > 0:
        print("Train set has been trimmed slightly..")
        content_targets = content_targets[:-mod] 
    
    # 风格特征预处理
    style_features = {}

    batch_shape = (batch_size,256,256,3)
    style_shape = (1,) + style_target.shape
    print(style_shape)

    # precompute style features
    with tf.Graph().as_default(), tf.device('/cpu:0'), tf.Session() as sess:
        # 使用 numpy 库在 CPU 上处理
        # TODO：使用占位符来定义风格图像 style_image
        style_image = tf.placeholder(tf.float32, style_shape)

        #TODO: 依次调用 vgg.py 文件中的 preprocess()、net() 函数对风格图像进行预处理，并将此时得到的特征提取网络传递给 net
        net = vgg.net(vgg_path, vgg.preprocess(style_image))

        # 使用 numpy 库对风格图像进行预处理，定义风格图像的格拉姆矩阵
        style_pre = np.array([style_target])
        for layer in STYLE_LAYERS:
            features = net[layer].eval(feed_dict={style_image:style_pre})
            features = np.reshape(features, (-1, features.shape[3]))
            gram = np.matmul(features.T, features) / features.size
            style_features[layer] = gram

        #TODO：先使用占位符来定义内容图像 X_content，再调用 preprocess() 函数对 X_content 进行预处理，生成 X_pre
        X_content = tf.placeholder(tf.float32, batch_shape)
        X_pre = vgg.preprocess(X_content)

        # 提取内容特征对应的网络层
        # precompute content features
        content_features = {}
        content_net = vgg.net(vgg_path, X_pre)
        content_features[CONTENT_LAYER] = content_net[CONTENT_LAYER]

        if slow:
            preds = tf.Variable(
                tf.random_normal(X_content.get_shape()) * 0.256
            )
            preds_pre = preds
        else:
            # TODO: 内容图像经过图像转换网络后输出结果 preds，并调用 preprocess() 函数对 preds 进行预处理, 生成 preds_pre
            preds = transform.net(X_content / 255., type)
            preds_pre = vgg.preprocess(preds)

        # TODO：preds_pre 输入到特征提取网络，并将此时得到的特征提取网络传递给 net
        net = vgg.net(vgg_path, preds_pre)

        # TODO：计算内容损失 content_loss, 风格损失 style_loss, 全变分正则化项 tv_loss, 损失函数 loss
        content_loss, style_loss, tv_loss, loss = loss_function(net, content_features, style_features, \
                                                                content_weight, style_weight, tv_weight, \
                                                                preds_pre, batch_size)

        # TODO：创建 Adam 优化器，并定义模型训练方法为最小化损失函数方法，返回 train_step
        train_step = tf.train.AdamOptimizer(learning_rate).minimize(loss)

        # TODO：初始化所有变量
        checkpoint_dir = './ckp_temp/fns.ckpt'
        if load:
            print('loading checkpoint')
            saver = tf.train.Saver()
            if os.path.isdir(checkpoint_dir):
                ckpt = tf.train.get_checkpoint_state(checkpoint_dir)
                if ckpt and ckpt.model_checkpoint_path:
                    saver.restore(sess, ckpt.model_checkpoint_path)
                else:
                    raise Exception("No checkpoint found...")
            else:
                saver.restore(sess, checkpoint_dir)
        else:
            sess.run(tf.global_variables_initializer())

        import random
        uid = random.randint(1, 100)
        print("UID: %s" % uid)
        for epoch in range(epochs):
            num_examples = len(content_targets)
            iterations = 0
            while iterations * batch_size < num_examples:
                start_time = time.time()
                curr = iterations * batch_size
                step = curr + batch_size
                X_batch = np.zeros(batch_shape, dtype=np.float32)
                for j, img_p in enumerate(content_targets[curr:step]):
                    X_batch[j] = get_img(img_p, (256,256,3)).astype(np.float32)

                iterations += 1
                assert X_batch.shape[0] == batch_size

                feed_dict = {
                    X_content:X_batch
                }

                train_step.run(feed_dict=feed_dict)
                end_time = time.time()
                delta_time = end_time - start_time
                if debug:
                    print("UID: %s, batch time: %s" % (uid, delta_time))
                print('iteration: %d'%iterations)
                is_print_iter = int(iterations) % print_iterations == 0
                if slow:
                    is_print_iter = epoch % print_iterations == 0
                is_last = epoch == epochs - 1 and iterations * batch_size >= num_examples
                should_print = is_print_iter
                if (iterations == 1 and epoch == 0):
                    to_get = [style_loss, content_loss, tv_loss, loss, preds]
                    test_feed_dict = {
                        X_content:X_batch
                    }

                    tup = sess.run(to_get, feed_dict = test_feed_dict)
                    _style_loss,_content_loss,_tv_loss,_loss,_preds = tup
                    print('Epoch %d, Iteration: %d, Loss: %s' % (epoch, iterations, _loss))
                    to_print = (_style_loss, _content_loss, _tv_loss)
                    print('style: %s, content:%s, tv: %s' % to_print)

                if should_print:
                    to_get = [style_loss, content_loss, tv_loss, loss, preds]
                    test_feed_dict = {
                        X_content:X_batch
                    }

                    tup = sess.run(to_get, feed_dict = test_feed_dict)
                    _style_loss,_content_loss,_tv_loss,_loss,_preds = tup
                    losses = (_style_loss, _content_loss, _tv_loss,_loss)
                    
                    if slow:
                        _preds = vgg.unprocess(_preds)
                    elif save:
                        # TODO：将模型参数保存到 save_path，并将训练的次数 save_id 作为后缀加入到模型名字中
                        saver = tf.train.Saver()
                        res = saver.save(sess, save_path)
                    # 将相关计算结果返回
                    yield(_preds, losses, iterations, epoch)

def _tensor_size(tensor):
    # 对张量进行切片操作，将 NHWC 格式的张量，切片成 HWC，再计算 H、W、C 的乘积
    # 其实就是返回 H * W * C， 利用mul进行reduce
    from operator import mul
    return functools.reduce(mul, (d.value for d in tensor.get_shape()[1:]), 1)
