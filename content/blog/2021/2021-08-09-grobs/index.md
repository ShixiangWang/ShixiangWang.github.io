---
title: grid 1：图形对象grobs
author: 王诗翔
date: '2021-08-09'
slug: grobs
categories:
  - Blog
tags:
  - 可视化
  - 学习
  - grid
description: grid的核心数据对象。
rmd_source: ''
keywords: rstats
editor_options:
  chunk_output_type: console
---

学习材料：[https://bookdown.org/rdpeng/RProgDA/the-grid-package.html#grobs](https://bookdown.org/rdpeng/RProgDA/the-grid-package.html#grobs)

`grobs` 是 grid 绘图系统中图形对象的表示，即 `graphics + objects = grobs`

`grobs` 的创建通常使用以 `Grob` 作为后缀的函数们，包括`circleGrob`, `linesGrob`, `polygonGrob`, `rasterGrob`, `rectGrob`, `segmentsGrob`, `legendGrob`, `xaxisGrob`, and `yaxisGrob` 等等。 `gridExtra` 包定义了更多的图形对象。

下面是一个绘圆的示例：

```R
library(grid)
my_circle <- circleGrob(x = 0.5, y = 0.5, r = 0.5,
                        gp = gpar(col = "gray", lty = 3))
```


每个图形对象有自带的参数，比如圆有它的中心，半径。而不同的对象有相同的一些参数设定，包括颜色、线型、大小等等，这通过 `gpar` 函数进行设定。

再创建图形对象后，使用 `grid.draw()` 将图形绘制出来。

```R
grid.draw(my_circle)
```


![](https://cdn.nlark.com/yuque/0/2021/png/471931/1621587529167-de0bc3e6-5ed6-40fd-a3ab-09b45deb3baf.png)

多个图形对象可以叠加，而且叠加后可以修改原来的图形对象。

```R
my_circle <- circleGrob(name = "my_circle",
                        x = 0.5, y = 0.5, r = 0.5,
                        gp = gpar(col = "gray", lty = 3))
grid.draw(my_circle)

my_rect <- rectGrob(x = 0.5, y = 0.5, width = 0.8, height = 0.3)
grid.draw(my_rect)
```


![](https://cdn.nlark.com/yuque/0/2021/png/471931/1621587745380-6f6f27da-20c8-4cf3-9a2c-d129b36c1a97.png)

修改：

```R
grid.edit("my_circle", gp = gpar(col = "red", lty = 1))
```


![](https://cdn.nlark.com/yuque/0/2021/png/471931/1621590475555-31a04caa-af58-4424-b44a-eeeac28be70a.png)

ggplot2 是基于 grid 系统构建的，所以它们可以比较好的进行整合。

```R
wc_plot <- ggplot(worldcup, aes(x = Time, y = Passes)) + 
  geom_point()

grid.draw(wc_plot)
grid.draw(my_circle)
```


![](https://cdn.nlark.com/yuque/0/2021/png/471931/1621590706907-05bd4920-4635-4b5c-b5b5-8c845c3de10c.png)

我们可以使用 grid 提供的函数来修改 ggplot2 图形，首先我们需要先找到 ggplot 图像中的对象，然后使用 `grid.edit()` 进行修改。

下面是一个例子：

```R
wc_plot
grid.force() # 将 ggplot2 转变为 grid 对象列表
grid.ls()
```


输出：

```R
layout
  background.1-7-10-1
  panel.6-4-6-4
    grill.gTree.1413
      panel.background..rect.1404
      panel.grid.minor.y..polyline.1406
      panel.grid.minor.x..polyline.1408
      panel.grid.major.y..polyline.1410
      panel.grid.major.x..polyline.1412
    NULL
    geom_point.points.1400
    NULL
    panel.border..zeroGrob.1401
  spacer.7-5-7-5
  spacer.7-3-7-3
  spacer.5-5-5-5
  spacer.5-3-5-3
  axis-t.5-4-5-4
  axis-l.6-3-6-3
    axis.line.y..zeroGrob.1432
    axis
      axis.1-1-1-1
        GRID.text.1429
      axis.1-2-1-2
  axis-r.6-5-6-5
  axis-b.7-4-7-4
    axis.line.x..zeroGrob.1425
    axis
      axis.1-1-1-1
      axis.2-1-2-1
        GRID.text.1422
  xlab-t.4-4-4-4
  xlab-b.8-4-8-4
    GRID.text.1416
  ylab-l.6-2-6-2
    GRID.text.1419
  ylab-r.6-6-6-6
  subtitle.3-4-3-4
  title.2-4-2-4
  caption.9-4-9-4
```


然后我们可以针对性修改：

```R
grid.edit("geom_point.points.1400", gp = gpar(col = "red"))
grid.edit("GRID.text.1419", gp = gpar(fontface = "bold"))
```


ggplot2 包中的  `ggplotGrob()` 函数可以将 ggplot 对象转化为 grob。

gTree grob 可以包含多个子 grob（即组装多个子对象），可以用于像箱线图这种复杂的图形中。下面是例子：

```R
candy <- circleGrob(r = 0.1, x = 0.5, y = 0.6)
stick <- segmentsGrob(x0 = 0.5, x1 = 0.5, y0 = 0, y1 = 0.5)
lollipop <- gTree(children = gList(candy, stick))
grid.draw(lollipop)
```


![](https://cdn.nlark.com/yuque/0/2021/png/471931/1621591087235-ca33ddb9-18c7-41ca-bb8b-94f242755135.png)

列出子对象：

```R
grid.ls(lollipop)
GRID.gTree.7233
  GRID.circle.7231
  GRID.segments.7232
```




