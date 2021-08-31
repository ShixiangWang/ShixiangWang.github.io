---
title: 《R语言数据科学导论》笔记
author: 王诗翔
date: '2021-08-30'
slug: note-for-r-data-science-intro
categories:
  - Blog
tags:
  - R
  - 机器学习
description: 之前视频介绍的笔记后续
---

原始资料来源：<https://github.com/leovan/data-science-introduction-with-r>



## 特征工程

[特征工程是将原始数据转换成特征的过程](http://machinelearningmastery.com/discover-feature-engineering-how-to-engineer-features-and-how-to-get-good-at-it/)。更通俗地说，[特征工程就是人工设计模型的输入变量 x的过程](https://www.quora.com/What-is-feature-engineering)。

主要分为：

- 数据预处理
- 特征提取和选择
- 特征变换和编码
- 特征监控



### 数据预处理

对赃数据进行清洗，包括括缺失，噪声，不一致等等一系列问题数据。

剔除处理：

- 样本去重。同一个ID出现多次重复记录。
- 特征去重。例如月收入和年收入，它们都是用于表征收入特征，关系只差常数倍。
- 常量特征剔除。即常量或方差近似为0的特征。`caret`包中的`nearZeroVar()`可以帮助我们识别该类特征。

![image-20210830143700530](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301437688.png)

缺失值处理：

- 探索缺失值：`mice`包的`md.pattern()`，`VIM`包的`aggr()/marginplot()`。
- 处理：
  - 删除法，可以直接使用`na.omit()`。
  - 插补法，如果该特征对最终的预测结果影响较小，则我们可以直接删除该特征；相反如果该特征对预测结果影响较大，直接删除会对模型造成较大的影 响，此时我们需要利用其它的方法对该特征的缺失值进行填补。其中**最简单的方式是利用均值，中位数或众数等统计量对其进行简单插补**。这种插补方法是建立在完全随机缺失的前提假设下，同时会造成变量方差变小。



异常值是指样本中存在的同样本整体差异较大的数据。

分为2类：

![image-20210830144222267](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301442306.png)


采样是一种常见的预处理技术。

- 随机采样。每个样本单位被抽中的概率相等，样本的每个单位完全独立，彼此间无一定的关联性和排斥性。
- 分层采样。将抽样单位按某种特征或某种规则划分为不同的层，然后从不同的层中独立、随机地抽取样本。从而保证样本的结构与总体的结构比较相近，从而提高估计的精度。可以利用`sampling::strata()`。
- 欠采样和过采样。我们经常会碰到不同分类的样本比例相差较大的问题，这种问题会对我们构建模型造成很大的影响，因此从数据角度出发，我们可以利用欠采样或过采样处理这种现象。可以利用`ROSE::ovun.sample()`。

### 特征变换和编码

#### 无量纲化

通过归一化，我们可以消除不同量纲下的数据对最终结果的影响。

![image-20210830150239893](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301502946.png)

```R
normalize <- function(x) {
  # 计算极值
  x_min <- min(x)
  x_max <- max(x)
  # 归一化
  x_n <- (x - x_min) /
  (x_max - x_min)
  # 将极值作为结果的属性
  attr(x_n,
  'min') <- x_min
  attr(x_n,
  'max') <- x_max
  # 返回归一化后结果
  x_n
}
```

**标准化。**

![image-20210830150431122](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301504156.png)



注意标准化在机器学习中的作用。

![image-20210830150640191](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301506237.png)

**离散化。**

![image-20210830150739289](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301507333.png)

**哑变量化（也称热编码，one-hot encoding）**

在R语言中对包括因子类型变量数据建模时，一般会将其自动处理为虚拟变量或哑变 量，这样我们就可以将因子类型的数据转化为数值型数据使用。

使用`caret::dummyVars()`和`mltools::one_hot()`可以进行编码。



### 特征提取、选择和监控

提取：

![image-20210830151058102](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301510152.png)



选择：

![image-20210830151156466](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301511506.png)

![image-20210830151427543](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301514590.png)

监控：

![image-20210830151504088](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301515126.png)



## 模型评估和超参数优化



### 模型性能评估

对学习器的泛化性能进行评估，不仅需要有效可行的实验评估方法，还需要有衡量模型泛化能力的评价标准，也就是性能度量（performance measure）。

注意，模型的“好坏”是相对的，什么样的模型时好的，不仅仅取决于算法和数据，还决定于任务需求。



#### 回归

![image-20210830152129078](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301521133.png)

![image-20210830152154964](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301521004.png)

![image-20210830152318825](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301523867.png)

#### 分类

分类问题可以划分为两类：二分类问题和多分类问题。两种不同的分类问题的性能评估方法略有不同，相对而言二分类问题的评估指标体系更为复杂一些。

![image-20210830152425137](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301524182.png)

![image-20210830152453581](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301524620.png)

![image-20210830152618119](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301526152.png)

![image-20210830152715277](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301527310.png)

![image-20210830152739393](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301527423.png)

![image-20210830152800453](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301528494.png)

![image-20210830152833365](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301528401.png)

![image-20210830152852762](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301528792.png)

#### 聚类

![image-20210830152926518](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301529564.png)

![image-20210830152951463](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301529490.png)

![image-20210830153010085](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301530118.png)

![image-20210830153049450](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301530497.png)



### 模型生成和选择

#### 拟合

![image-20210830153139617](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301531666.png)



#### 评估

通常，我们可以通过实验测试来对学习器的泛化误差进行评估并进而做出选择。为此，需使用一个测试集（testing set）来测试学习器对新样本的判别能力，然后**以测试集上的测试误差（testing error）作为泛化误差的近似**。通常，我们假设测试样本也是 从样本真实分布中独立同分布采样而来。但需要注意，测试集应该尽可能与训练集互斥，**即测试样本尽量不在训练集中出现、未在训练过程中使用过。若测试样本被用作训练了，则得到的将是过于“乐观”的估计结果。**

方法：

- 留出法 hold-out
- 交叉验证法 cross-validation，特例为留一法 leave-one-out
- 自助法 bootstrapping（有放回采样到原始数据集大小）

![image-20210830154138560](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301541618.png)

#### 偏差和方差

对学习算法除了通过实验估计其泛化性能，我们往往还希望了解它“为什么”具有这样 的性能。偏差-方差分解（bias-variance decomposition）是解释学习算法泛化性能的一 种重要工具，它试图对学习算法的期望泛化错误率进行拆解。

![image-20210830154315241](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301543288.png)

![image-20210830154418622](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301544664.png)

![image-20210830154522924](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301545958.png)

### 超参数优化

模型的参数和超参数二者有着本质上的区别：**模型参数是模型内部的配置变量，可以用数据估计模型参数的值，例如：回归中的权重，决策树分类点的阈值等；模型超参数是模型外部的配置，必须手动设置参数的值**，例如：随机森林树的个数，聚类方法 里面类的个数，或者主题模型里面主题的个数等。

常用的超参数优化方法有： 

- 搜索算法：网格搜索，随机搜索等 
- 进化和群体算法：遗传算法，粒子群算法等
- 贝叶斯优化

#### 网格搜索

网格搜索法算法就是通过交叉验证的方法去寻找最优的模型参数。模型的每个参数有很多个候选值，我们每个参数组合做一次交叉验证，最后得出交叉验证分数最高的，就是我们的最优参数。

![image-20210830154746534](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301547592.png)



## 分类算法



### 逻辑回归

方程：

![image-20210830154955269](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301549332.png)

![image-20210830155021964](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301550989.png)

![image-20210830155139809](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301551840.png)

### 决策树

![image-20210830155354135](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301553175.png)

决策树（Decision Tree）是机器学习和数据挖掘中的一套分类和回归方法。决策树是由节点和有向边构成的树形分类模型，**其中树的叶子节点表示某个分类，非叶子结点表示一个用于树枝分叉的特征属性**。

决策树的生成主要有三个步骤：

- 特征的选择：特征选择是指从数据集中的多个属性特种中选择具有分类能力的特 征。不同的特征选择策略将导致不同决策树的生成。
- 决策树生成：决策树生成是指利用选择的特征，递归的构建决策树。
- 决策树剪枝：决策树剪枝是指为了 防止过拟合现象，对于过于复杂的决策树进行简化的过程。



#### 特征选择

在构建决策树时，会有两个基本问题： 

1. 依次选择哪些特征作为划分节点，才能够保证每个节点都能够具有最好的分类能力？ 
2. 对于连续型变量，选择什么值作为划分依据？

决策树主要从信息论的角度处理这两个问题，具体的选择依据有**信息增益，信息增益率和Gini系数**等。



#### 熵 Entropy

![image-20210830155554091](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301555139.png)

#### 信息增益

![image-20210830155655322](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301556391.png)



![image-20210830160127323](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301601363.png)

#### Gini指数

![image-20210830160233813](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301602868.png)

#### 连续型数据处理

把需要处理的样本（对应根节点）或样本子集（对应子 树）按照连续变量的大小从小到大进行排序，假设该属性对应的不同的属性值一共有N个，那么总共有N-1个可能的候选分割阈值点，每个候选的分割阈值点的值为上述排序后的属性值链表中两两前后连续元素的中点，那么我们的任务就是从这个N-1个候选分割阈值点中选出一个，使得前面提到的信息论标准最大。



#### 决策树生成 CART

![image-20210830160525019](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301605069.png)



#### 剪枝

决策树对于训练样本来说，可以得到一个100%准确的分类器。算法生成的决策树非常的详细而且庞大，每个属性都被详细地加以考虑。但是如果训练样本中包含了一些错误，按照前面的算法，这些错误也会100%被决策树学习了，这就**产生了过拟合现象**。 因此，为了解决这个问题，**我们需要对生成的决策树进行简化，这个简化的过程就称之为剪枝。**

![image-20210830160650700](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301606762.png)



### 集成学习

传统的机器学习算法（例如：决策树，人工神经网络，支持向量机，朴素贝叶斯等） 的目标都是寻找一个最优的分类器尽可能的将训练数据分开。**集成学习（Ensemble Learning）算法的基本思想就是通过将多个分类器组合，从而实现一个预测效果更好的集成分类器**。集成算法可以说从一方面验证了中国的一句老话：三个臭皮匠，赛过诸葛亮。

![image-20210830161053614](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301610683.png)

（纠正：是算法不是算分）

![image-20210830161128156](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301611212.png)

#### Bagging

![image-20210830161249666](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301612736.png)

![image-20210830161328306](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301613363.png)

**随机森林**

随机森林（Random Forests）是一种利用决策树作为基学习器的Bagging集成学习算法。

过程：

![image-20210830161500768](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301615808.png)

![image-20210830161526190](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301615222.png)



#### Boosting

Boosting是一种提升算法，可以将弱的学习算法提升（boost）为强的学习算法。其基本思路如下：

1. 利用初始训练样本集训练得到一个基学习器。
2. 提高被基学习器误分的样本的权重，使得那些被错误分类的样本在下一轮训练中可以得到更大的关注，利用调整后的样本训练得到下一个基学习器。
3. 重复上述步骤，直到得出M个学习器。
4. 对于分类问题，采用有权重的投票方式；对于回归问题，采用加权平均得到预测值。



**Adaboost**

![image-20210830161946718](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301619769.png)

**GBM**

GBM（Gradient Boosting Machine）是另一种基于Boosting思想的集成算法，GBM还有很多其他的叫法，例如：GBDT，GBRT，MART等等。GBM算法由3个主要概念构成： Gradient Boosting（GB），Regression Decision Tree（DT或RT）和Shrinkage。

**从GBM的众多别名中可以看出，GBM中使用的决策树并非我们最常用的分类树，而是回归树。**

对于Gradient Boosting而言，首先，Boosting并不是Adaboost中的Boost的概念，也不是 Random Forest中的重抽样。在Adaboost中，Boost是指在生成每个新的基学习器，根据上一轮基学习器分类对错对训练集设置不同的权重，**使得在上一轮中分类错误的样本在生成新的基学习器时更被重视**。GBM中在应用Boost概念时，每一轮所使用的数据集**没有经过重抽样，也没有更新样本的权重，而是每一轮选择了不同的回归的目标值**，即上一轮计算得出的残差（Residual）。其次，Gradient是指在新一轮中在残差减少的梯度（Gradient）上建立新的基学习器。

![image-20210830163219346](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301632426.png)

![image-20210830163345971](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301633020.png)

![image-20210830163406919](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301634957.png)

![image-20210830163459847](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301634900.png)



#### Stacking

![image-20210830163550353](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108301635429.png)



## 时间序列算法

### 时间序列



### ARIMA模型



### 季节性分析



## 聚类算法



### K-means



### 层次聚类



### 基于密度的聚类



