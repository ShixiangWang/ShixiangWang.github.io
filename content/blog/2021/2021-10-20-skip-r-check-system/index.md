---
title: 跳过R包check系统使用无法显式载入DESCRIPTION的外部包
author: 王诗翔
date: '2021-10-20'
slug: skip-r-check-system
categories:
  - Blog
tags:
  - R
  - 开发
description: 小黑屋
---

```R
.attach_this <- function() {
  if (!"ggpubr" %in% (.packages())) {
    tryCatch(eval(parse(text = "library(ggpubr)")),
             error = function(e) {
               eval(parse(text = 'remotes::install_github("ggpubr")'))
               eval(parse(text = "library(ggpubr)"))
             })
    
  }
}

`%:::%` <- function(pkg, fun, inherits = TRUE) {
  get(fun,
      envir = asNamespace(pkg),
      inherits = inherits
  )
}

.attach_this()
ggboxp <- "ggpubr"%:::%"ggboxplot"


args(ggboxp)

```