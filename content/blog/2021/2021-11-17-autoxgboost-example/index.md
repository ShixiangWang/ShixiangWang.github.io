---
title: autoxgboost例子
author: 王诗翔
date: '2021-11-17'
slug: autoxgboost-example
categories:
  - Blog
tags:
  - 笔记
  - 机器学习
description: 摘录
---

```r
library(OpenML)
library(autoxgboost)
data = getOMLDataSet(31)
GermanCredit = convertOMLDataSetToMlr(data)
# reg_task <- makeRegrTask(data = data_train, target = "Share_Temporary")
# reg_task <- makeRegrTask(data = data_train, target = "Share_Temporary")
autoxgbparset.mixed = makeParamSet(
  makeDiscreteParam("booster", values = c("gbtree", "gblinear", "dart")),
  makeDiscreteParam("sample_type", values = c("uniform", "weighted"), requires = quote(booster == "dart")),
  makeDiscreteParam("normalize_type", values = c("tree", "forest"), requires = quote(booster == "dart")),
  makeNumericParam("rate_drop", lower = 0, upper = 1, requires = quote(booster == "dart")),
  makeNumericParam("skip_drop", lower = 0, upper = 1, requires = quote(booster == "dart")),
  makeLogicalParam("one_drop", requires = quote(booster == "dart")),
  makeDiscreteParam("grow_policy", values = c("depthwise", "lossguide")),
  makeIntegerParam("max_leaves", lower = 0, upper = 8, trafo = function(x) 2^x, requires = quote(grow_policy == "lossguide")),
  makeIntegerParam("max_bin", lower = 2L, upper = 9, trafo = function(x) 2^x),
  makeNumericParam("eta", lower = 0.01, upper = 0.2),
  makeNumericParam("gamma", lower = -7, upper = 6, trafo = function(x) 2^x),
  makeIntegerParam("max_depth", lower = 3, upper = 20),
  makeNumericParam("colsample_bytree", lower = 0.5, upper = 1),
  makeNumericParam("colsample_bylevel", lower = 0.5, upper = 1),
  makeNumericParam("lambda", lower = -10, upper = 10, trafo = function(x) 2^x),
  makeNumericParam("alpha", lower = -10, upper = 10, trafo = function(x) 2^x),
  makeNumericParam("subsample", lower = 0.5, upper = 1)
)
xgb = autoxgboost(GermanCredit, par.set = autoxgbparset.mixed, iterations = 20)

Param_chosen <- mlr::getHyperPars(xgb$final.learner)
print(unlist(Param_chosen))
```

另外：<https://www.simoncoulombe.com/2019/01/bayesian/>