---
title: forestmodel给多水平变量添加整体p值
author: 王诗翔
date: '2021-08-31'
slug: forestmode-set-overall-pva-for-variable-with-multiple-levels
categories:
  - Blog
tags:
  - R
  - forestmodel
description: 我不是作者，但你问对了人
---

前段时间收到来信：

```
Hi Shixiang 

I am writing to you about the forestmodel package in R. 

Thank you so much for the wonderful package that you created. 

I was wondering if there is a way to display the wald test p-value which is important for variables that have more than two levels. I tried to work around the code but did not find a way out. 

Best 
Aniket 
```

我不是作者，搞错了人，问我干啥呢～自个提问嘛

```
Hi Aniket,

I am not the author of forestmodel, you can see from https://github.com/NikNakk/forestmodel/ or CRAN package info. As for your question, I cannot understand it based on the current info you provided. Could you post an issue on https://github.com/NikNakk/forestmodel/issues?

Best,
```

很明白的回复吧。这位一点也不尴尬，继续问我。。。还仔细说问题了。

```
Hi Shixiang 

I posted the issue on github. There is another user who has requested same help. 

It’s basically about displaying a global p-value (from ward test or likelihood ratio test) when there are multiple categories (more than two, for e.g race- Asian, black, Caucasian, Etc) in a variable, in addition to individual p-value against the reference. 

Best 
Aniket 
```

好吧，这下让我有点兴趣了。我仔细看了下issue（<https://github.com/NikNakk/forestmodel/issues/31>），发现提问人是想要把多水平变量的p值展示在森林图上。

我觉得挺没有必要的，而且有点奇怪，为什么呢？我展示了一个示例：

```R
> library("survival")
> library("dplyr")
> pretty_lung <- lung %>%
+   transmute(time,
+             status,
+             Age = age,
+             Sex = factor(sex, labels = c("Male", "Female")),
+             ECOG = factor(lung$ph.ecog),
+             `Meal Cal` = meal.cal
+   )
> table(lung$ph.ecog)

  0   1   2   3 
 63 113  50   1 
> coxph(Surv(time, status) ~ ., pretty_lung)
Call:
coxph(formula = Surv(time, status) ~ ., data = pretty_lung)

                 coef  exp(coef)   se(coef)      z       p
Age         7.848e-03  1.008e+00  1.080e-02  0.727 0.46727
SexFemale  -5.050e-01  6.035e-01  1.913e-01 -2.640 0.00829
ECOG1       2.756e-01  1.317e+00  2.221e-01  1.241 0.21462
ECOG2       7.852e-01  2.193e+00  2.543e-01  3.087 0.00202
ECOG3       1.792e+00  6.004e+00  1.036e+00  1.731 0.08348
`Meal Cal` -6.031e-05  9.999e-01  2.300e-04 -0.262 0.79313

Likelihood ratio test=21.56  on 6 df, p=0.001453
n= 180, number of events= 133 
   (48 observations deleted due to missingness)
```

这种多变量的cox回归中，p值展示的是整个模型的结果，而ECOG这个因子变量本身建模时被拆分成了3个变量，是没法得到一个p值的。那为啥搞这个呢？

继续的交流了解到他们就是想要进行批量的单变量分析，想要展示整个变量的p值，还给我用图形举例说明了。

![image-20210831195841536](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108311958610.png)

![image-20210831195908154](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108311959191.png)



了解了问题，再加上我很久之前修改过这个包的源代码，提交过PR，所以大体感觉这个问题不难。想办法把上图中右侧的`reference`在需要时右侧添加文字就好了。通过添加一个新的参数来控制这种行为。

安装包：

```R
remotes::install_github("ShixiangWang/forestmodel")
```

看看效果：

```R
library("survival")
library("dplyr")
library("forestmodel")

pretty_lung <- lung %>%
  transmute(time,
            status,
            Age = age,
            Sex = factor(sex, labels = c("Male", "Female")),
            ECOG = factor(lung$ph.ecog),
            `Meal Cal` = meal.cal
  )

print(forest_model(coxph(Surv(time, status) ~ ., pretty_lung), show_global_p = "bottom"))
print(forest_model(coxph(Surv(time, status) ~ ., pretty_lung), show_global_p = "aside"))
```

![image-20210831202115822](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108312021863.png)

> 在实现的过程中发现将global p值加到最下方也是有益的，并不仅限于单因素模型使用。



![image-20210831200453864](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108312004901.png)

```R
print(forest_model(coxph(Surv(time, status) ~ ECOG, pretty_lung), show_global_p = "aside"))
```



![image-20210831200526104](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/202108312005144.png)



由于我之前开发的ezcox包可以直接接收传到forestmodel的其他参数，所以只要无需改动ezcox就完成了对该包该特性的支持。有使用ezcox包的读者可以试试。

由于做了不少维护和开发工作，就这两天forestmodel作者将我加入了[作者列表](https://github.com/NikNakk/forestmodel/blob/d001c80f5b001ff8c4c82201b1f223be64415b8b/DESCRIPTION#L9)。这算是无心插柳吗？
