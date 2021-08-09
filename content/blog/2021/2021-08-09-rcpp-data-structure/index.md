---
title: Rcpp：数据结构
author: 王诗翔
date: '2021-08-09'
slug: rcpp-data-structure
categories:
  - Blog
tags:
  - R
  - Rcpp
  - 学习
description: post description
keywords: rstats
editor_options:
  chunk_output_type: console
---

## RObject类

RObject类在Rcpp类系统中占核心地位。虽然它不是面向用户的，但为接下来的所有类提供了公共的数据结构，它是构建Rcpp API的基础类。

每一个RObject类实例都封装了一个R对象，而每个对象在内部可以表示为一个SEXP：一个指向S表达式对象的指针。

基于它的用户可见（可使用）的类：

- IntegerVector对应整型向量。

- NumericVector对应数值向量。

- LogicalVector对应逻辑值向量。

- CharacterVector对应Character向量。

- GenericVector对应List类型的泛型向量。

- ExpressionVector对应表达式类型向量。

- RawVector对应raw类型向量。

对于整型和数值型，我们还有IntegerMatrix和NumericMatrix对应R中的数值矩阵。

下面我们通过整型向量来了解它们。


## IntegerVector类

模板函数`as<>()`用于从R转换到C++，而`wrap()`函数的方向相反。

> 实际大多数使用情况下，我们已经不需要显式地进行转换处理，该过程会在底层自动完成。


### 示例：返回完美数

什么是完美数：

[https://baike.baidu.com/item/完全数/370913?fromtitle=完美数&fromid=871560&fr=aladdin](https://baike.baidu.com/item/%E5%AE%8C%E5%85%A8%E6%95%B0/370913?fromtitle=%E5%AE%8C%E7%BE%8E%E6%95%B0&fromid=871560&fr=aladdin)

代码：

```R
library(Rcpp)
library(inline)

src <- '
  Rcpp::IntegerVector epn(4);
  epn[0] = 6;
  epn[1] = 14;
  epn[2] = 496;
  epn[3] = 8182;
  return epn;
'

func <- cxxfunction(signature(), src, plugin = "Rcpp")
```


调用：

```R
> func()
[1]    6   14  496 8182
```


### 示例：使用输入

求乘积。

```R
src <- '
  Rcpp::IntegerVector vec(vx);
  int prod = 1;
  for (int i=0; i<vec.size(); i++) {
  prod *= vec[i];
  }
  return Rcpp::wrap(prod);
'

func <- cxxfunction(signature(vx="integer"), src, plugin = "Rcpp")
```


调用：

```R
> func(1:3)
[1] 6
> func(1:10)
[1] 3628800
```


### 示例：使用错误的输入

当我们使用浮点数据时，看看结果是怎样的：

```R
> func(seq(1.0, 1.9, by = 0.1))
[1] 1
```


我们得到了一个错误的结果！原因是输入的浮点数被强制转换为了整数，因此1.0到1.9都变成了1。

## Named类

Named类是一个辅助类，**用于设定键值对中的键** 。我们简单看看它在R中的示例：

```R
> someVec <- c(mean = 1.2, sd = 3, dim = 4)
> someVec
mean   sd  dim 
 1.2  3.0  4.0 
```


向量中的每一个元素被赋予了一个对应的标签。Named类允许我们对C++创建的对象做类似的事情。

```R
> src <- '
+   Rcpp::NumericVector x =
+     Rcpp::NumericVector::create(
+     Rcpp::Named("mean") = 1.2,
+     Rcpp::Named("sd") = 3,
+     Rcpp::Named("dim") = 4);
+   return x;
+ '
> 
> func <- cxxfunction(signature(), src, plugin = "Rcpp")
> func()
mean   sd  dim 
 1.2  3.0  4.0 
```


上述构造可以简写为：

```R
src <- '
  Rcpp::NumericVector x =
    Rcpp::NumericVector::create(
    _["mean"] = 1.2,
    _["sd"] = 3,
    _["dim"] = 4);
  return x;
'
```


## List类，又名GenericVector类

这是最一般的数据类型，它可以包含其他数据类型，就行R中的list()函数一样。

由于可以保存不同类型的数据对象，List很适合用于R到C++之间的参数传递。


## DataFrame类

数据框是R中最常用的数据类型，在Rcpp中当然也进行了支持。下面是一个示例：

```R
> src <- '
+   Rcpp::IntegerVector v = Rcpp::IntegerVector::create(7,8,9);
+   std::vector<std::string> s(3);
+   s[0] = "x";
+   s[1] = "y";
+   s[2] = "z";
+   return Rcpp::DataFrame::create(
+     Rcpp::Named("a") = v,
+     Rcpp::Named("b") = s);
+ '
> 
> func <- cxxfunction(signature(), src, plugin = "Rcpp")
> func()
  a b
1 7 x
2 8 y
3 9 z
```


## 其他复杂类型

其他比较复杂的R对象类型在Rcpp中也能找到对应的类型，包括函数、环境、S4对象等。其声明示例大致如下：

```C++
Rcpp::Function rt("rt"); // 声明rt函数为R中的rt函数
Rcpp::Environment stats("package:stats"); // 使用R的stats环境并获取其中的rnorm函数
Rcpp::Function rnorm = stats["rnorm"]; 

S4 foo(x); // S4 类
foo.slot(".Data") = "foooo";
// 一些检测方法
RObject y(x);
y.isS4();
y.hasSlot("z");
y.slot("z"); // 获取z数据槽 

```
