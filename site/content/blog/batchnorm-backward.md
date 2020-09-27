<!--
title: BatchNorm计算中，均值和方差是否参与梯度计算？
desc: 这是一个我非常喜欢的面试题目，通过率1/5不到
template: blog
target: artical
date: 2020-04-15
-->

[^note1]: Wiki https://en.wikipedia.org/wiki/Batch_normalization 

Batch normalization是一个常见的计算，用于增加训练稳定，并且提供一定的正则化能力(regularization) ， 它的公式也非常简单，不用赘述了[^note1]。这里的有一个有趣的问题，那就是在训练的时候，BN计算的均值和方差，是常量还是变量，即是否参与梯度计算当中？

很多人非常简单认为均值和方差不参与梯度计算，当作常量对待即可，然而这是不对的，因为均值和方差是由batch输入计算得到，如果不参与梯度求导，就破坏了整个链式法则。

为了确认这么一个简单的问题，我查看了几个框架的代码实现，都100%肯定，BN计算中，均值和方差参与梯度计算：

### Caffe中的实现

Caffe中的实现，参见这个 [链接地址](https://github.com/BVLC/caffe/blob/master/src/caffe/layers/batch_norm_layer.cpp#L169), 具体代码摘抄如下：

```clike
  // if Y = (X-mean(X))/(sqrt(var(X)+eps)), then
  //
  // dE(Y)/dX =
  //   (dE/dY - mean(dE/dY) - mean(dE/dY \cdot Y) \cdot Y)
  //     ./ sqrt(var(X) + eps)
  //
  // where \cdot and ./ are hadamard product and elementwise division,
  // respectively, dE/dY is the top diff, and mean/var/sum are all computed
  // along all dimensions except the channels dimension.  In the above
  // equation, the operations allow for expansion (i.e. broadcast) along all
  // dimensions except the channels dimension where required.

  // sum(dE/dY \cdot Y)
  caffe_mul(temp_.count(), top_data, top_diff, bottom_diff);
  caffe_cpu_gemv<Dtype>(CblasNoTrans, channels_ * num, spatial_dim, 1.,
      bottom_diff, spatial_sum_multiplier_.cpu_data(), 0.,
      num_by_chans_.mutable_cpu_data());
  caffe_cpu_gemv<Dtype>(CblasTrans, num, channels_, 1.,
      num_by_chans_.cpu_data(), batch_sum_multiplier_.cpu_data(), 0.,
      mean_.mutable_cpu_data());

  // .... 
```

从上面的注释可以看到，求解BN的梯度，必须严格按照除法的导数计算法则，将`^ mean(x) var(x)` 看作变量对待。 

### Mxnet 中的实现

我们再来看看MxNet中的代码实现，参见这个 [链接地址](https://github.com/apache/incubator-mxnet/blob/e06ee4e1a725aed62a66b525036676500010e7f0/src/operator/nn/batch_norm.cc#L291), 具体代码摘抄：

```clike
if (is_train_and_not_global_stats) {
        // when in training mode
        // Q(X) = X - E[x] ; i.e. input centered to zero mean
        // Y = Q(X) / σ    ; i.e. BN output before weight and bias
        // dL/dX = (Q(dL/dY) - dot(Y, dL/dY) * Y) / σ * w

        // projection of gradOutput on to output scaled by std
        const AccReal k = dotp * invstd * invstd / itemCount;
        const AccReal iw = invstd * w;
        const AccReal gradMean = sumGradOut / itemCount;
        if (req[batchnorm::kData] != kAddTo) {
          ForEachFast(inputData, gradIn, static_cast<size_t>(channel),
                      [&mean, &k](const DType *inputDataPtr, DType *gradIn_data) {
                        *gradIn_data = (*inputDataPtr - mean) * k;
                      });

          ForEachFast(gradOut, gradIn, static_cast<size_t>(channel),
                      [iw, gradMean](const DType *gradOut_data, DType *gradIn_data) {
                        *gradIn_data = (*gradOut_data - gradMean - *gradIn_data) * iw;
                      });
        } else {
          ForEachFast(inputData, gradOut, gradIn, static_cast<size_t>(channel),
                      [&mean, &k, iw, gradMean](const DType *inputDataPtr,
                                                const DType *gradOut_data,
                                                DType *gradIn_data) {
                        DType normal_val = (*inputDataPtr - mean) * k;
                        *gradIn_data += (*gradOut_data - gradMean -
                            normal_val) * iw;
                      });
        }
```

注意上面的注释公式，`^ dL/dX = (Q(dL/dY) - dot(Y, dL/dY) * Y) / σ * w `，也是明确说明当处于训练模式下，且均值和方差来自于当前batch统计，是不能将均值和方差看作常量的。


就这么一个简单的问题，很多人没有深究过，我非常喜欢这个面试题目，大概通过率不到1/5。