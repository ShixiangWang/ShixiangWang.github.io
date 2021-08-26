---
title: 解决由于网络问题导致的stringi安装失败问题
author: 王诗翔
date: '2021-08-26'
slug: install-stringi-when-bad-acess-to-gh
categories:
  - Blog
tags:
  - 配置
  - R
description: 每次重装R，都会碰到的问题
---

这是一个我每次重装R，或者在新的系统上使用R进行包安装，都会遇到的问题。

`stringi`是tidyverse的一个核心包，基本上必装。但由于gayhub经常访问有问题，这个包安装时所需要的依赖文件会下载不了。
解决的办法是手动下载，然后进行安装：

```bash
wget https://github.com/gagolews/stringi/archive/master.zip -O stringi.zip
# 如果上面github的链接无法下载，尝试：
# wget https://download.fastgit.org/gagolews/stringi/archive/master.zip -O stringi.zip
unzip stringi.zip
sed -i '/\/icu..\/data/d' stringi-master/.Rbuildignore
R CMD build stringi-master
R CMD INSTALL stringi*.tar.gz
```

参考：<https://stackoverflow.com/questions/31942322/how-to-install-stringi-from-local-file-absolutely-no-internet-access#>