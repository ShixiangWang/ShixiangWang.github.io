---
title: Flux Overview：建立一个简单的预测
author: 王诗翔
date: '2021-12-31'
slug: flux-overview
categories:
  - Blog
tags:
  - Flux
  - 机器学习
  - julia
description: 来自官方文档的概览
---


```julia
julia> using Flux

julia> actual(x) = 4x + 2
actual (generic function with 1 method)
```

### 提供训练和测试集

```julia
julia> x_train, x_test = hcat(0:5...), hcat(6:10...)
([0 1 … 4 5], [6 7 … 9 10])

julia> y_train, y_test = actual.(x_train), actual.(x_test)
([2 6 … 18 22], [26 30 … 38 42])
```

通常，你的训练和测试数据来自真实世界的观察，但这个函数将模拟真实世界的观察。


### 构建一个模型预测

```julia
julia> model = Dense(1, 1)
Dense(1, 1)         # 2 parameters

julia> model.weight
1×1 Matrix{Float32}:
 -1.0924082

julia> model.bias
1-element Vector{Float32}:
 0.0
```

在底层，一个全连接层是一个含有`weight`和`bias`的结构体。`weight`代表权重矩阵，`bias`代表偏置向量。
我们可以使用其他方式思考一个模型。在Flux中，模型是概念上的预测函数：

```julia
julia> predict = Dense(1, 1)
Dense(1, 1) 
```

`Dense(1, 1)`实现了函数`σ(Wx+b)`，$\sigma$是激活函数。我们这里的模型
只有一个权重和偏置，但典型的模型一般有更多。把权重和偏差想象成Flux可以用来调整预测的把手和杠杆。激活函数是你需要定制模型的转换。

这个模型已经可以预测了，尽管还不准确：

```julia
julia> predict(x_train)
1×6 Matrix{Float32}:
 0.0  -0.747322  -1.49464  -2.24196  -2.98929  -3.73661
```

为了做出更好的预测，你需要提供一个损失函数来告诉Flux如何客观地评估预测的质量。损失函数计算实际值和预测值之间的累积距离。

```julia
julia> loss(x, y) = Flux.Losses.mse(predict(x), y)
loss (generic function with 1 method)

julia> loss(x_train, y_train)
258.06296f0
```

预测越准确，损失就越小。你可以编写自己的损失函数或依赖Flux已经提供的函数。这个损失函数叫做均方误差。Flux的工作原理是通过训练迭代地减少损失。


### 提升预测

底层实现Flux的`train!`函数使用损失函数和训练数据用来基于一个优化器来更新参数。

```julia
julia> using Flux: train!

julia> opt = Descent()
Descent(0.1)

julia> data = [(x_train, y_train)]
1-element Vector{Tuple{Matrix{Int64}, Matrix{Int64}}}:
 ([0 1 … 4 5], [2 6 … 18 22])
```

现在，我们有了传给`train!`的优化器和数据。剩下的就是模型的参数了。记住，每个模型都是一个具有函数和可配置参数的Julia结构。记住，密集层的权重和偏差取决于输入和输出的尺寸：

```julia
julia> predict.weight
1×1 Matrix{Float32}:
 -0.7473216

julia> predict.bias
1-element Vector{Float32}:
 0.0
```

这些模型参数的尺寸取决于输入和输出的数量。由于模型可以有数百个输入和多个层次，有一个函数来收集参数到Flux期望的数据结构中是有用的：

```julia
parameters = params(predict)
Params([Float32[-0.7473216;;], Float32[0.0]])
```

这些是Flux将会改变的参数，一步一次，来改进预测。每个参数都来自预测模型：

```julia
predict.weight in parameters, predict.bias in parameters
(true, true)
```

这个优化器实现了经典的梯度下降策略。现在我们使用下面的方式更新模型参数：

```julia
train!(loss, parameters, data, opt)
```

检查损失：

```julia
julia> loss(x_train, y_train)
244.93066f0
```

它下降了，为什么？

```julia
julia> parameters
Params([Float32[8.956102;;], Float32[2.7736611]])
```

参数发生了改变。这一步就是机器学习的本质。

### 迭代训练模型

在前一节中，我们只进行了一次训练。它在我们传入的数据上迭代一次。`epoch`指的是对数据集的一次训练。通常情况下，我们会进行多个epoch的训练，以进一步降低损失。让我们再运行几次：

```julia
julia> for epoch in 1:200
         train!(loss, parameters, data, opt)
       end

julia> loss(x_train, y_train)
0.007161801f0

julia> parameters
Params([Float32[4.0259266;;], Float32[2.0073032]])
```

训练步数达到200步后，损失下降，参数逐渐接近模型预测函数中的参数。

### 验证预测

现在，让我们验证预测：

```julia
julia> predict(x_test)
1×5 Matrix{Float32}:
 26.1629  30.1888  34.2147  38.2406  42.2666

julia> y_test
1×5 Matrix{Int64}:
 26  30  34  38  42
```

预测结果是很好的。下面介绍我们是如何得到的。

首先，我们把真实数据的数据存储到变量`x_train`，`y_train`，`x_test`和`y_test`中。`x_*`数据定义了输入，`y_*`数据定义输出。`*_train`数据用来训练模型，`*_test`数据用来验证模型。我们的数据基于函数`4x + 2`。

接下来，我们构建了一个单个输入、单个输出的预测模型，`predict = Dense(1, 1)`。因为我们还没有训练模型，所以初始的预测不准确。

在构建模型之后，我们使用`train!(loss, parameters, data, opt)`进行训练。首先是损失函数，然后是存储权重和偏置的`parameters`、训练数据以及Flux提供的梯度下降优化器。我们执行训练过程一次，观察到参数的改变和损失的下降。然后，我们多次运行`train!`完成训练过程。

在训练模型之后，我们使用测试数据验证模型。

这整个工作流展示了Flux是如何工作的，让我们让我们进一步深入了解Flux的各个层内部发生了什么。

