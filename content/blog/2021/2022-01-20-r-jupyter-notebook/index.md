---
title: R使用Jupyter Notebook那些事
author: 王诗翔
date: '2022-01-20'
slug: r-jupyter-notebook
categories:
  - Blog
tags:
  - R
  - Jupyter
description: 人用的少就问题多～
---

<!-- Links -->

在操作的一台服务器的R没有X11支持，Jupyter一运行代码就报错Kernel挂掉。而RStudio Server可以直接点击切换图形后端为Cairo。
Jupyter 怎么搞呢？

首先尝试了Stack Overflow的一个办法，在 `~/.Rprofile` 中加入代码：

```r
setHook(packageEvent("grDevices", "onLoad"),
        function(...) grDevices::X11.options(type = "cairo"))
options(device = "x11")
```

最后还是在IRkernel的官方仓库问题区看到解决的办法：

> https://github.com/IRkernel/IRkernel/issues/388

加入下面的语句就可以了。

```r
## Set default 'type' for png() calls - useful when X11 device is not available!
## NOTE: Needs 'cairo' capability
options(bitmapType='cairo')
```

完成后重启下Jupyter，然后等待一会，再试一试。

另外，由于Jupyter无法像R一样拖动绘图窗口实时修改图形大小，如果要改动的话需要
提前用下面的语句设置：

```r
options(repr.plot.width = 4, repr.plot.height = 3)
```

我们可以写一个简化函数：

```r
setplot = function(w=5, h=4, d = c("svg", "png")) {
  options(repr.plot.width = w, repr.plot.height = h)
  options(jupyter.plot_mimetypes = paste0("image/", switch(
    match.arg(d),
    png = "png",
    svg = "svg+xml"
  )))
}
setplot()
```

> Jupyter 对 SVG 支持的更好，默认用这个看起来更舒服。

