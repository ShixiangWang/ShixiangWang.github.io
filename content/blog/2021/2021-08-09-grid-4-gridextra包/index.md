---
title: grid 4：gridExtra包
author: 王诗翔
date: '2021-08-09'
slug: grid-4-gridextra包
categories:
  - Blog
tags:
  - 可视化
  - 学习
  - grid
rmd_source: ''
keywords: rstats
editor_options:
  chunk_output_type: console
---

gridExtra包提供了grid系统的有用拓展，包括对grobs对象的操作方法和其他一些grobs对象。

`grid.arrange()`可以绘制多个对象：

```R
library(gridExtra)
grid.arrange(lollipop, circleGrob(),
             rectGrob(), lollipop, 
             ncol = 2) 
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123914.png)

由于ggplot2也是基于grid系统的，所以我们可以使用该函数对ggplot对象组合排序：

```R
time_vs_shots <- ggplot(worldcup, aes(x = Time, y = Shots)) + 
  geom_point()
player_positions <- ggplot(worldcup, aes(x = Position)) + 
  geom_bar()

grid.arrange(time_vs_shots, player_positions, ncol = 2)
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123921.png)

使用`layout_matrix`选项可以进行更自定义控制：

```R
grid.arrange(time_vs_shots, player_positions,
             layout_matrix = matrix(c(1, 2, 2), ncol = 3))
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123931.png)

包括图形填充：

```R
grid.arrange(time_vs_shots, player_positions,
             layout_matrix = matrix(c(1, NA, NA, NA, 2, 2), 
                                    byrow = TRUE, ncol = 3))
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123938.png)

使用`tableGrob()`函数，我们可以在图中嵌入表格。下面是一个略微复杂的例子：

```R
worldcup_table <- worldcup %>%
  filter(Team %in% c("Germany", "Spain", "Netherlands", "Uruguay")) %>%
  group_by(Team) %>%
  dplyr::summarize(`Average time` = round(mean(Time), 1),
                   `Average shots` = round(mean(Shots), 1)) %>%
  tableGrob()
`summarise()` ungrouping output (override with `.groups` argument)

grid.draw(ggplotGrob(time_vs_shots)) # 先绘制了一个ggplot
wc_table_vp <- viewport(x = 0.22, y = 0.85, 
                        just = c("left", "top"),
                        height = 0.1, width = 0.2) # 添加一个新的视图绘制表格
pushViewport(wc_table_vp)
# 绘制表格
grid.draw(worldcup_table)
popViewport()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123945.png)

**更多关于grid的知识：** 

- grid附加文档：[https://stat.ethz.ch/R-manual/R-devel/library/grid/doc/index.html](https://stat.ethz.ch/R-manual/R-devel/library/grid/doc/index.html)

- gridExtra一系列附加文档：[https://cran.r-project.org/web/packages/gridExtra/index.html](https://cran.r-project.org/web/packages/gridExtra/index.html)


