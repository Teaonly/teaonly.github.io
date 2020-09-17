<!--
title: Softmax公式的来由
desc: 本文介绍了指数函数分布族，以及通用线性模型，以及sigmoid/softmax公式的推导
template: blog
target: artical
date: 2020-04-13
-->

softmax公式是大家都熟悉的公式，形式如下:

```^
p(y_i|x) = \frac{e^{\theta_i^Tx}} { \sum_{j=1}^{K} e^{\theta_j^Tx} }

```

关于这个公式的含义是，K表示类别，`^y_i`表示第i个类别是否为1,  `^x`表示特征，`^\theta_j`参数每个类别一个和`^x`一样维度的向量，整个公式返回`^y_i`的概率 估计。softmax有很多的优势，比如数值计算上比较接近hardmax (所谓hardmax，即0/1，哪个分量最大返回1，其他为0)，梯度计算简单，通过公式简的变形，也可以做到数值计算稳定，另外，配合cross entropy函数，其梯度计算也比较简单。

[^note1]: 本文内容，主要参考了CS229的Lecture3, 地址：http://cs229.stanford.edu/notes/cs229-notes1.pdf

但是为什么选择了softmax这样的形式来表示条件概率？或者说为什么在多分类问题中，使用了softmax函数。这是一个非常有意思的问题，很多AI工程师都搞不清楚为什么要用这样的形式，或者说弄不清楚，softmax公式的由来。本文介绍一种推导路径，即从GLM(Generalized Linear Model，通用线性模型)的多分类PDF函数推导得到softmax公式[^note1]。

### 指数函数分布族

首先引入指数函数分布族，指数函数分布族，指的是随机变量的PDF函数(或者离散随机变量的PMF)，可以用下面的指数函数来表示：

```^
p(y;\eta) = b(y) e^{\eta^T T(y) - a(\eta)}
```

上面这个公式，就是指数函数分布族(exponential family distributions), 表示的是随机变量Y的取值为`^y`时的PDF(概率密度函数). 公式中参数`^\eta`也叫做自然参数（natural paramter, 有时候也叫做canonical paramter). `^T(y)` 是充分统计量(sufficient statistic), 最常见的形式是 `^T(y)=y`. `^a(\eta)` 指的是对数配分函数(log partition function), 用来满足PDF函数积分为1的约束. T,a,b的定义形式，决定了分布函数的类型。

#### 高斯分布就属于指数函数分布族

为了简化形式，我们首先假设高斯分布中的方差为1，即`^\delta`=1. 从高斯原始定义出发，进行简单变形，可以得到上面得指数函数分布族得形式。

```^
\begin{aligned}
p(y;\mu) &= \frac{1}{\sqrt{2\pi}} exp(-\frac{1}{2}(y-\mu)^2) \\
         &= \frac{1}{\sqrt{2\pi}} exp(-\frac{1}{2}y^2) exp(\mu y - \frac{1}{2}\mu^2) \\
thus,\\
T(y) &= y  \\
\eta &= \mu \\ 
a(\eta) &= \mu^2/2 \\
        &= \eta^2/2 \\
b(y) &= (1/\sqrt{2\pi})exp(-y^2/2) 
\end{aligned}
```

除了高斯分布，其他常见的分布，比如二分类的伯努利分布，泊松分布都属于这一类分布，他们的PDF函数都可以变形到指数函数族。

### 构造通用线性模型 (Generalized Linear Model)

在各种预测和回归问题中，我们需要的条件概率，即`^p(y|x; \theta)`, 通用线性模型就是假设条件概率下，随机变量Y是指数函数族分布，我们最终需要的预测函数`^h_\theta(x)` 他返回的是函数T(y)的数学期望。此外，我们指定自然参数`^\eta` 和 x 之间关系是线性的，用公式描述如下：

```^
\begin{aligned}
p(y|x) &= ExponentialFamily(\eta) \\
h_\theta(x) &= E(p(T(y)|x)) \\
     &= E(p(y|x)) \\
\eta &= \theta^Tx
\end{aligned}
```

上面公式中，第二个等式的意思是，大多数情况下T(y)=y， 通过前面两个假设，我们就建立了 `^h_\theta, \eta` 之间的关系，就可以根据指定的ExponentialFamily，来推导出预测函数。他们之间的关系是：

1. 条件概率分布属于指数函数分布族
1. 目标函数(预测函数)输出为条件概率的期望，根据分布可以表达为自然参数`^\eta`的函数
1. 通过`^\eta = \theta^Tx`, 得到目标函数 `^h_\theta(x)` 在输入x和`^\theta`下的表达式


### 案例：逻辑回归的推导

#### 二分类的伯努利分布，指数函数分布族形式

首先我们看看逻辑回归对应的二分类问题，首先我们写出伯努利分布的pdf，其中`^\phi`是唯一参数，将他改写为 `^T,a,b,\eta`的形式。

```^
\begin{aligned}
p(y;\phi) &= \phi^y(1-\phi)^y \\
          &= exp(y.log(\phi) + (1-y).log(1-\phi)) \\
          &= exp( log(\frac{\phi}{1-\phi}).y + log(1-\phi)) \\
thus,\\
T(y) &= y  \\
b(y) &= 1   \\
\eta &= log(\frac{\phi}{1-\phi}) => \phi = 1 / (1 + e^\eta) \\
a(\eta) &= - log(1-\phi)) \\
        &= log (1+e^\eta) 
\end{aligned}
```

根据前述的GLM方法，我们可以推导预测函数：

```^
\begin{aligned}
h_\theta(y|x; \theta) &= E(y|x; \theta) = \phi \\
        & = 1 / (1 + e^\eta) \\
        & = 1 / (1 + e^{\theta^Tx})
\end{aligned}
```

上面sigmoid函数的推导的基础是，我们假设条件概率分布是伯努利分布，其次我们需要预测函数返回的前面这个分布的数学期望，带参数数学期望最后用`^g(\eta)` 来表示，这个函数g也被称为canonical response function，最后我们直接将`^\eta`用`^\theta^Tx`线性表达即可。

### 案例：Softmax 回归的推导

同样，按照相同的方法来推导softmax公式，这次是多分类问题，那么分布函数是multinomial分布，随机变量y取值从1到K，表示多分类的列别，我们通过函数T(y)把，把标量变成向量，注意这个向量的维度为 k -1 :

```^
T(y=1) = \left[ \begin{aligned} 1 \\ 0 \\ ... \\ 0 \\ 0 \\ 0 \end{aligned} \right],
T(y=2) = \left[ \begin{aligned} 0 \\ 1 \\ ... \\ 0 \\ 0 \\ 0 \end{aligned} \right], ... ,
T(y=k-1) = \left[ \begin{aligned} 0 \\ 0 \\ ... \\ 0 \\ 0 \\ 1 \end{aligned} \right], 
T(y=k) = \left[ \begin{aligned} 0 \\ 0 \\ ... \\ 0 \\ 0 \\ 0 \end{aligned} \right]
```

上面这随机变量在multinomial分布，也可以写成指数函数的形式：

```^
\begin{aligned}
p(y;\phi) &= \phi_1^{1 \left\{ y=1 \right\}} \phi_2^{1 \left\{ y=2 \right\}}...\phi_k^{1 \left\{ y=k \right\}} \\
          &= \phi_1^{1 \left\{ y=1 \right\}} \phi_2^{1 \left\{ y=2 \right\}}...\phi_k^{1 - \sum_i^{k-1}1\left\{ y=k \right\}} \\
          &= \phi_1^{T(y)_1} \phi_2^{T(y)_2} ... \phi_{k-1}^{T(k-1)_{k-1}} \phi_k^{1 - \sum_i^{k-1}T(y)_i} \\
          &= exp( T(y)_1 log(\phi_1) + T(y)_2 log(\phi_2)) + ... + T(y)_{k-1} log(\phi_{k-1}) + (1 - \sum_i^{k-1}T(y)_i) log(\phi_k) ) \\
          &= exp( T(y)_1 log(\phi_1/\phi_k) + T(y)_2 log(\phi_2/\phi_k)) + ... + T(y)_{k-1} log(\phi_{k-1}/\phi_k) + log(\phi_k) ) \\
          &= b(\eta) exp( \eta^T .y - a(\eta)) \\
thus, \\
\eta &= \left[ log(\phi_1/\phi_k) , log(\phi_2/\phi_k) , log(\phi_{k-1}/\phi_k)  \right]^T \\
a(\eta) &= - log(\phi_k) \\
b(y) &= 1
\end{aligned}
```

再接下来，简化一下`^\eta`和`^\phi`之间的关系，注意这里指定了 `^\eta_k=0`, 扩展维度到k.

```^
\begin{aligned} 
\eta_i    &= log \frac{\phi_i}{\phi_{k}}  \\
\phi_{i}  &= e^{\eta_i} \phi_{k}       \\
\phi_{k}\sum_{i=1}^{k}e^{\eta_i} &= \sum_{i=1}^{k}\phi_i = 1 \\
\phi_{k} &= 1 / \sum_{i=1}{k}e^{\eta_i}
\end{aligned}
```
根据上面这几个公式，化简形式可以得到：

```^
\phi_{i} = \frac{e^{\eta_i}}{\sum_{i=1}^{k}e^{\eta_i}}
```

上面这个公式，已经具备了softmax公式, 因此可以直接推导我们的目标函数，为了方便表达，我们指定`^\theta_k=0`，自然满足前面`^\eta_k=\theta_kx=0`的设置：

```^
\begin{aligned}
h_{\theta}(x) &= E(T(y)|x; \theta) \\
              &= [\phi_1, \phi_2, ..., \phi_{k-1}]^T \\
              &= [ \\
              &  \frac{e^{\theta_1x}}{\sum_{i=1}^{k}e^{\theta_ix}}, \\
              &  \frac{e^{\theta_2x}}{\sum_{i=1}^{k}e^{\theta_ix}}, \\
              &  , ...,  \\
              &  \frac{e^{\theta_{k-1}x}}{\sum_{i=1}^{k}e^{\theta_ix}}\\
              & ]^T
\end{aligned} 
```

最后我们依据`^\sum_{i=1}^{k}\phi_i=1`，将`^h_\theta(x)`扩充一个维度，就可以得到标准的softmax公式了。

```^
h_{\theta}^j(x) = \frac{e^{\theta_jx}}{\sum_{i=1}^{k}e^{\theta_ix}} 
```