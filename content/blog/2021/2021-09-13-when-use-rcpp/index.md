---
title: Rcpp：什么时候使用Rcpp
author: 王诗翔
date: '2021-09-13'
slug: when-use-rcpp
categories:
  - Blog
tags:
  - R
  - Rcpp
description: +++++++++++++++++
---

来源：<https://teuder.github.io/rcpp4everyone_en/010_Rcpp_merit.html>


## 什么时候使用

- 后面的迭代依赖于前面的迭代的循环操作。
- 需要访问向量/矩阵的每个元素。
- 在循环中循环调用函数。
- 动态更改向量的大小。
- 需要高级数据结构和算法的操作。

## 怎么配置

除了Windows需要安装Rtools，其他系统中一般已经装好了。

如果我们要自定义C++的配置，如更改编译器，需要使用到配置文件`.R/Makevars`。

下面是一个示例：

```r
CC=/opt/local/bin/gcc-mp-4.7
CXX=/opt/local/bin/g++-mp-4.7
CPLUS_INCLUDE_PATH=/opt/local/include:$CPLUS_INCLUDE_PATH
LD_LIBRARY_PATH=/opt/local/lib:$LD_LIBRARY_PATH
CXXFLAGS= -g0 -O2 -Wall
MAKE=make -j4
```

> 包括编译器位置、头文件位置、动态库位置、编译参数等。

## 安装Rcpp

```r
install.packages("Rcpp")
```