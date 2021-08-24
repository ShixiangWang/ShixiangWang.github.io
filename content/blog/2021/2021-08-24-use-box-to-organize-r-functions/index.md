---
title: 使用modules包来组织R的函数集合
author: 王诗翔
date: '2021-08-24'
slug: use-modules-to-organize-r-functions
categories:
  - Blog
tags:
  - R
  - 开发
description: modules包介绍和使用
rmd_source: ''
keywords: rstats
editor_options:
  chunk_output_type: console
---

接触过Python的朋友肯定对模块很熟悉，R的代码组织方式以包为主。但基于文件的模块形式也是可以实现的，[modules](https://github.com/klmr/modules) 包提供了这种支持。

## 安装和使用

直接从CRAN下载即可：

```r
install.packages("modules")
```

使用了解2个函数的使用就可以了。

一是`import()`，用于替换`library()`加载包。

```r
> library(modules)
> gp = import('ggplot2')
Masking (modules:ggplot2):
  `Position` from: base
> args(gp$ggplot)
function (data = NULL, mapping = aes(), ..., environment = parent.frame()) 
NULL
> args(ggplot)
function (data = NULL, mapping = aes(), ..., environment = parent.frame()) 
NULL
```

这样我们可以直接使用这个函数，也可以通过`gp`这个对象去访问可用的函数。

如果不想要在全局直接访问包内的函数，在导入时设定`attach=FALSE`。

```r
> dp <- import(dplyr, attach = FALSE)
Masking (modules:dplyr):
  `intersect` from: base
  `setdiff` from: base
  `setequal` from: base
  `union` from: base
> select
错误: 找不到对象'select'
> dp$select
function (.data, ...) 
{
    UseMethod("select")
}
<bytecode: 0x7fe5671213f8>
<environment: namespace:dplyr>
```

除了导入包，也可以导入具体的包的某个/些函数：

```r
> slt <- import(dplyr, select)
Replacing attached import/use on search path for: modules:dplyr.
> slt$select
function (.data, ...) 
{
    UseMethod("select")
}
<bytecode: 0x7fe5671213f8>
<environment: namespace:dplyr>
```

我看中的其实不是上面这些特性，而是它可以将文件里写好的函数整体加载为模块对象。

接下来介绍第二个函数。

## `use()`将代码文件加载为模块

最近使用GitHub page的时候发现它的访问速度相当可观，哪怕GitHub主站点本身网络我们国内访问时好时坏。
所以，如果我将代码文件放到GitHub上，国内任何的读者应该是可以快速地通过GitHub page访问到其内容。
那么，利用GitHub page加上这里介绍的`use()`函数构建一个可实时获取的代码库是可能的。对于小的脚本函数，
写一个文件总是比写一个包简单轻量。

基于上面的思想，我将去年写的R包安装以及TCGA样本名重过滤等几个函数单独通过GitHub page进行了部署。

```r
> tcga <- modules::use("https://biosisyphus.github.io/Rlib/tcga.R")
> tcga$filterReplicates(tsb = c("TCGA-55-7913-01B-11D-2237-01", "TCGA-55-7913-01B-11X-2237-01", "TCGA-55-7913-01B-11D-2237-01"))
ooo Filter barcodes successfully!
[1] "TCGA-55-7913-01B-11D-2237-01"
```

目前放置的几个代码文件都可以在代码库<https://github.com/BioSisyphus/Rlib>中查看。

这里一个对绝大部分读者有用的函数是`install()`，它之前被放在R包`wfun`中。我前几天把它重新进行了迁移和修改。
该模块可以替换`BiocManager::install()`工作，可以从CRAN/BioC/GitHub/Gitee/目录等地方安装包。代码核心其实
就是各种情况的检查，优先使用适合的包和函数进行下载、安装。它的存在就是方便国内使用者，特别是
初学者简便地下载、安装包。

```r
install <- modules::use("https://biosisyphus.github.io/Rlib/install.R")
> install$install("tidyverse/ggplot2")
载入需要的名字空间：remotes
Required package remotes has been installed.
Installing from GitHub mirrors...
试开URL’https://codeload.github.com/tidyverse/ggplot2/zip/HEAD'
downloaded 3.3 MB

载入需要的名字空间：pkgbuild
Required package pkgbuild has been installed.
✓  checking for file ‘/Volumes/Extra/R/Rtmp/RtmpobJHhN/filed328f28489/ggplot2-HEAD/DESCRIPTION’ ...
─  preparing ‘ggplot2’:
✓  checking DESCRIPTION meta-information ...
─  checking for LF line-endings in source and make files and shell scripts (412ms)
─  checking for empty or unneeded directories
─  building ‘ggplot2_3.3.5.9000.tar.gz’
   
将程序包安装入‘/Volumes/Extra/R/R_Library’
(因为‘lib’没有被指定)
Using library: /Volumes/Extra/R/R_Library
Using temp directory: /Volumes/Extra/R/Rtmp/
* installing *source* package ‘ggplot2’ ...
** using staged installation
** R
** data
*** moving datasets to lazyload DB
** inst
** byte-compile and prepare package for lazy loading
Using library: /Volumes/Extra/R/R_Library
Using temp directory: /Volumes/Extra/R/Rtmp/
** help
*** installing help indices
*** copying figures
** building package indices
Using library: /Volumes/Extra/R/R_Library
Using temp directory: /Volumes/Extra/R/Rtmp/
** installing vignettes
** testing if installed package can be loaded from temporary location
Using library: /Volumes/Extra/R/R_Library
Using temp directory: /Volumes/Extra/R/Rtmp/
** testing if installed package can be loaded from final location
Using library: /Volumes/Extra/R/R_Library
Using temp directory: /Volumes/Extra/R/Rtmp/
** testing if installed package keeps a record of temporary installation path
* DONE (ggplot2)
```

考虑到该函数的常用性，如果你觉得这个函数好用，可以使用下面的命令将其保存到本地并进行配置：

```r
install$save()
```

这样你每次打开RStudio，`install`模块总是在存在。

如果代码库中程序存在问题，或者你有好的反馈，欢迎file issue。

