---
title: grid 3：图形坐标系统
author: 王诗翔
date: '2021-08-09'
slug: coords
categories:
  - Blog
tags:
  - 学习
  - 可视化
  - grid
keywords: rstats
editor_options:
  chunk_output_type: console
---

绘图时，我们需要坐标才能准确地对要绘制的对象进行定位。在 grid 包中，有多种绘图坐标系统，选择一个合适的加以利用可以帮助我们更好地绘制图形。

坐标系统不同的单位：

1. *native* 单位：根据数据值。

2. *n* pc单位：将整个（笛卡尔）坐标系缩放为0到1范围。

3. 实际尺寸单位：包括英寸、厘米、毫米。

例如，下面的绘图同时用到了前两者：

```R
ex_vp <- viewport(x = 0.5, y = 0.5, # npc 单位
                  just = c("center", "center"),
                  height = 0.8, width = 0.8,
                  xscale = c(0, 100), yscale = c(0, 10)) # 设定native单位
pushViewport(ex_vp)
grid.draw(rectGrob())
grid.draw(circleGrob(x = unit(20, "native"), y = unit(5, "native"), # 根据native单位绘图
                     r = 0.1, gp = gpar(fill = "lightblue")))
grid.draw(circleGrob(x = unit(85, "native"), y = unit(8, "native"),
                     r = 0.1, gp = gpar(fill = "darkred")))
popViewport()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123634.png)


