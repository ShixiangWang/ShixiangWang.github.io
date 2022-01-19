---
title: R(Studio)中指定外部软件路径
author: 王诗翔
date: '2022-01-19'
slug: r-studio-specify-path
categories:
  - Blog
tags:
  - R
description: solution!
---

在安装R kernel时报错：

```r
> IRkernel::installspec()
Error in IRkernel::installspec() : 
  jupyter-client has to be installed but “jupyter kernelspec --version” exited with code 127.
In addition: Warning message:
In system2("jupyter", c("kernelspec", "--version"), FALSE, FALSE) :
  error in running command
```

这种情况是R识别不了外部的`$PATH`，我们可以通过`~/.Rprofile`进行修改配置。

在RStudio中运行`file.edit("~/.Rprofile")`或者手动打开，添加如下内容：

```r
old_path = Sys.getenv("PATH")
Sys.setenv(PATH = paste(old_path, "~/miniconda3/bin/", sep = ":"))
```

北外镜像近期使用经验整体还是比较稳定的，不妨添加：

```
options(BioC_mirror="https://mirrors.bfsu.edu.cn/bioconductor")
options("repos" = c(CRAN="https://mirrors.bfsu.edu.cn/CRAN/"))
```

`~/.condarc`中可以添加：

```sh
channels:
  - defaults
show_channel_urls: true
default_channels:
  - https://mirrors.bfsu.edu.cn/anaconda/pkgs/main
  - https://mirrors.bfsu.edu.cn/anaconda/pkgs/r
  - https://mirrors.bfsu.edu.cn/anaconda/pkgs/msys2
custom_channels:
  conda-forge: https://mirrors.bfsu.edu.cn/anaconda/cloud
  msys2: https://mirrors.bfsu.edu.cn/anaconda/cloud
  bioconda: https://mirrors.bfsu.edu.cn/anaconda/cloud
  menpo: https://mirrors.bfsu.edu.cn/anaconda/cloud
  pytorch: https://mirrors.bfsu.edu.cn/anaconda/cloud
  pytorch-lts: https://mirrors.bfsu.edu.cn/anaconda/cloud
  simpleitk: https://mirrors.bfsu.edu.cn/anaconda/cloud
```

> https://mirrors.bfsu.edu.cn/help/anaconda/

