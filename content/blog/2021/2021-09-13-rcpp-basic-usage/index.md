---
title: Rcpp：基本用法
author: 王诗翔
date: '2021-09-13'
slug: rcpp-basic-usage
categories:
  - Blog
tags:
  - R
  - Rcpp
description: +++++++++++
---

来源：<https://teuder.github.io/rcpp4everyone_en/030_basic_usage.html>

使用Rcpp函数只需要3步：

1. 编写Rcpp源代码。
2. 编译代码。
3. 执行函数。

## 编写Rcpp代码

下面是一个对向量求和的Rcpp函数：

```cpp
//sum.cpp
#include <Rcpp.h>
using namespace Rcpp;

// [[Rcpp::export]]
double rcpp_sum(NumericVector v){
    double sum = 0;
    for(int i=0; i<v.length(); ++i){
        sum += v[i];
    }
    return(sum);
}
```

### Rcpp函数定义格式

下面是定义一个Rcpp函数的基本格式：

```cpp
#include<Rcpp.h>
using namespace Rcpp;

// [[Rcpp::export]]
RETURN_TYPE FUNCTION_NAME(ARGUMENT_TYPE ARGUMENT){

    //do something

    return RETURN_VALUE;
}
```

- `#include<Rcpp.h>`：这个句子允许你使用Rcpp包定义的类和函数。
- `// [[Rcpp::export]]`：这个句子下面定义的函数可以从R中访问。
你需要把这个句子附加到你想从R中使用的每个函数中。
- `using namespace Rcpp;`：这个句子是可选的。但是如果你没有写这个句子，
你必须添加前缀`Rcpp::`来指定由Rcpp定义的类和函数。(例如：`Rcpp::NumericVector`)
- `RETURN_TYPE FUNCTION_NAME(ARGUMENT_TYPE ARGUMENT){}`：你需要指定函数和参数的数据类型。
- `return RETURN_VALUE;`：如果函数将返回一个值，`return`语句是强制性的。
然而，如果你的函数没有返回值（即RETURN_TYPE是无效的），返回语句可以省略。

## 编译代码

函数`Rcpp::sourceCpp()`将编译你的源代码，并将定义的函数加载到R。

```R
library(Rcpp)
sourceCpp('sum.cpp')
```

## 使用函数

像正常R函数一样调用它就可以了。

```R
> rcpp_sum(1:10)
[1] 55

> sum(1:10)
[1] 55
```