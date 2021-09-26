---
title: '处理glm.fit: fitted probabilities numerically 0 or 1 occurred'
author: 王诗翔
date: '2021-09-26'
slug: process-glm-logistic-warning
categories:
  - Blog
tags:
  - R
  - 机器学习
description: 遇到警告，刚好看到一篇英文介绍，翻译下
---

> 原文：<https://www.statology.org/glm-fit-fitted-probabilities-numerically-0-or-1-occurred/>

在建立逻辑回归模型时遇到这个警告：

```
Warning message:
glm.fit: fitted probabilities numerically 0 or 1 occurred 
```

当拟合逻辑回归模型，且数据框中一个或多个观测值的预测概率与0或1难以区分时，会出现此警告。

值得注意的是，这是一个警告消息，而不是一个错误。即使你收到这个错误，你的逻辑回归模型仍然是合适的，但是可能值得分析原始数据框，看看是否有任何异常值导致此警告消息出现。

本教程将分享如何在实践中处理此警告消息。

### 重复警告

假设我们将logistic回归模型拟合到R中的以下数据框：

```R
#create data frame
df <- data.frame(y = c(0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1),
                 x1 = c(3, 3, 4, 4, 3, 2, 5, 8, 9, 9, 9, 8, 9, 9, 9),
                 x2 = c(8, 7, 7, 6, 5, 6, 5, 2, 2, 3, 4, 3, 7, 4, 4))

#fit logistic regression model
model <- glm(y ~ x1 + x2, data=df, family=binomial)

#view model summary
summary(model)

Warning message:
glm.fit: fitted probabilities numerically 0 or 1 occurred 

Call:
glm(formula = y ~ x1 + x2, family = binomial, data = df)

Deviance Residuals: 
       Min          1Q      Median          3Q         Max  
-1.729e-05  -2.110e-08   2.110e-08   2.110e-08   1.515e-05  

Coefficients:
              Estimate Std. Error z value Pr(>|z|)
(Intercept)    -75.205 307338.933       0        1
x1              13.309  28512.818       0        1
x2              -2.793  37342.280       0        1

(Dispersion parameter for binomial family taken to be 1)

    Null deviance: 2.0728e+01  on 14  degrees of freedom
Residual deviance: 5.6951e-10  on 12  degrees of freedom
AIC: 6

Number of Fisher Scoring iterations: 24
```

如果我们使用拟合的logistic回归模型对原始数据框中观测值的响应值进行预测，我们可以看到几乎所有的预测概率都与0和1没有区别：

```R
#use fitted model to predict response values
df$y_pred = predict(model, df, type="response")

#view updated data frame
df

   y x1 x2       y_pred
1  0  3  8 2.220446e-16
2  0  3  7 2.220446e-16
3  0  4  7 2.220446e-16
4  0  4  6 2.220446e-16
5  0  3  5 2.220446e-16
6  0  2  6 2.220446e-16
7  0  5  5 1.494599e-10
8  1  8  2 1.000000e+00
9  1  9  2 1.000000e+00
10 1  9  3 1.000000e+00
11 1  9  4 1.000000e+00
12 1  8  3 1.000000e+00
13 1  9  7 1.000000e+00
14 1  9  4 1.000000e+00
15 1  9  4 1.000000e+00
```

### 如何处理警告

有三种方法来处理这个警告信息：


**(1) 忽略它** 

在某些情况下，你可以简单地忽略此警告消息，因为它不一定表明逻辑回归模型有问题。它仅仅意味着数据框中的一个或多个观察结果具有与0或1不可区分的预测值。


**(2) 增加样本量**

在其他情况下，当您使用小数据框时，如果没有足够的数据来提供可靠的模型匹配，则会出现此警告消息。要解决这个错误，只需增加你输入模型的观察的样本量。


**(3) 移除离群值**

在其他情况下，当原始数据框架中存在异常值，且只有少量观测值拟合的概率接近0或1时，就会出现这种错误。通过去除这些异常值，警告信息通常就消失了。

### **其他资源**

下面的教程解释了如何处理R中的其他警告和错误：

- [How to Fix in R: invalid model formula in ExtractVars](https://www.statology.org/r-invalid-model-formula-in-extractvars/)
- [How to Fix in R: argument is not numeric or logical: returning na](https://www.statology.org/r-argument-is-not-numeric-or-logical/)
- [How to Fix: randomForest.default(m, y, …) : Na/NaN/Inf in foreign function call](https://www.statology.org/randomforest-na-nan-inf-in-foreign-function-call/)