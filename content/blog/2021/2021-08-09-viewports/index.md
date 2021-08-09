---
title: grid 2：视图 viewports
author: 王诗翔
date: '2021-08-09'
slug: viewports
categories:
  - Blog
tags:
  - 可视化
  - 学习
  - grid
keywords: rstats
editor_options:
  chunk_output_type: console
---

> 原文：[https://bookdown.org/rdpeng/RProgDA/the-grid-package.html#viewports](https://bookdown.org/rdpeng/RProgDA/the-grid-package.html#viewports)


## 视图是什么

视图是绘图窗口，我们可以将其移进或移出，以方便使用grid包定制绘图。我们可以导航到其中一个视图，进行一些更改，然后弹出并导航到另一个视图。简而言之，视图提供了一种在绘图（可以想象为一个画板）的不同子空间（不同的层，如果了解ggplot2，采用图层概念理解它）中导航和工作的方法。

下面是一个例子，我们在整个图形的右上角绘制一个棒棒图：

```R
# 默认，grid会初始化第一个视图
grid.draw(rectGrob())
# 创建一个小的视图
# 指定新视图的位置，X为0.5，Y为0.5，视图高宽都为0.5
sample_vp <- viewport(x = 0.5, y = 0.5, 
                      width = 0.5, height = 0.5,
                      just = c("left", "bottom")) # 位置校正参数
# 导航视图：上面只是定义了一个视图对象
# 我们可以想象为一个大画板是一张白纸
# 我们上面准备了一张小的白纸
# 而 push 的目的就是把小的白纸放到大白纸的对应位置
# 接下来的绘图动作就是在小的白纸上进行的
pushViewport(sample_vp)
grid.draw(roundrectGrob())
grid.draw(lollipop)
# 弹出最上层的视图
popViewport()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123352.png)

> 棒棒图的绘制见[「R」grid 图形对象 grobs](https://www.wolai.com/5sw6RUS9fcMYmUUmZfFbd6)一文末尾。


## 视图的`just`参数

值得注意的是这里的`just`参数：它定义了新的视图是如何在旧（大） 的视图上摆放的。这里的`c("left", "bottom")`指定了小的视图的左侧和下侧与其坐标单位`0.5, 0.5`对齐。我们看看修改下该参数的效果：

```R
grid.draw(rectGrob())
sample_vp <- viewport(x = 0.5, y = 0.5, 
                      width = 0.5, height = 0.5,
                      just = c("center", "center"))
pushViewport(sample_vp)
grid.draw(roundrectGrob())
grid.draw(lollipop)
popViewport() 
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123359.png)

## 操作视图

**我们一次只能操作一个视图，当确定视图后，我们就可以在上面绘制图形对象。如果我们需要在不同的视图中绘制，在绘制之前我们需要导航视图到我们需要进行绘制的视图上去。** 

```R
grid.draw(rectGrob())

sample_vp_1 <- viewport(x = 0.75, y = 0.75, 
                      width = 0.25, height = 0.25,
                      just = c("left", "bottom"))
pushViewport(sample_vp_1)
grid.draw(roundrectGrob())
grid.draw(lollipop)
popViewport() # 这里必须弹出，试一试不弹出的结果是什么？

sample_vp_2 <- viewport(x = 0, y = 0, 
                      width = 0.5, height = 0.5,
                      just = c("left", "bottom"))
pushViewport(sample_vp_2)
grid.draw(roundrectGrob())
grid.draw(lollipop)
popViewport()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123407.png)

## 视图嵌套

视图可以层层嵌套堆叠到一起，在实际绘图中，绘制子图、放大部分图形通常可以通过该方式实现。

```R
grid.draw(rectGrob())

sample_vp_1 <- viewport(x = 0.5, y = 0.5, 
                      width = 0.5, height = 0.5,
                      just = c("left", "bottom"))
sample_vp_2 <- viewport(x = 0.1, y = 0.1, 
                      width = 0.4, height = 0.4,
                      just = c("left", "bottom"))
# 先导航到第一个视图
pushViewport(sample_vp_1)
# 绘制
grid.draw(roundrectGrob(gp = gpar(col = "red")))
# 再导航到第二个视图
pushViewport(sample_vp_2)
# 绘制
grid.draw(roundrectGrob())
grid.draw(lollipop)
# 弹出添加的2个视图
popViewport(2)
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123412.png)

注意，上面我们使用了`popViewport(2)`重新导航到主（初始的）绘图视图。

`grid.ls()`可以罗列出所有的绘图对象：

```R
> grid.ls()
GRID.rect.7246
GRID.roundrect.7247
GRID.gTree.7233
  GRID.circle.7231
  GRID.segments.7232
```


## 捕获ggplot的grobs对象

对于ggplot对象，我们也可以捕获全部的grobs对象。操作示例如下：

```R
worldcup %>%
  ggplot(aes(x = Time, y = Passes)) + 
  geom_point()
grid.force() # 必需的
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123420.png)

```R
> grid.ls()
layout
  background.1-9-12-1
  panel.7-5-7-5
    grill.gTree.7261
      panel.background..zeroGrob.7252
      panel.grid.minor.y..polyline.7254
      panel.grid.minor.x..polyline.7256
      panel.grid.major.y..polyline.7258
      panel.grid.major.x..polyline.7260
    NULL
    geom_point.points.7249
    NULL
    panel.border..zeroGrob.7250
  spacer.8-6-8-6
  spacer.8-4-8-4
  spacer.6-6-6-6
  spacer.6-4-6-4
  axis-t.6-5-6-5
  axis-l.7-4-7-4
    NULL
    axis
      axis.1-1-1-1
        GRID.text.7267
      axis.1-2-1-2
  axis-r.7-6-7-6
  axis-b.8-5-8-5
    NULL
    axis
      axis.1-1-1-1
      axis.2-1-2-1
        GRID.text.7264
  xlab-t.5-5-5-5
  xlab-b.9-5-9-5
    GRID.text.7270
  ylab-l.7-3-7-3
    GRID.text.7273
  ylab-r.7-7-7-7
  subtitle.4-5-4-5
  title.3-5-3-5
  caption.10-5-10-5
  tag.2-2-2-2
```


我们可以将ggplot绘制到grid视图中去，下面是一个例子：

> 代码不重要，理解其思想和大致操作逻辑。


```R
balt_counties <- map_data("county", region = "maryland") %>%
  mutate(our_counties = subregion %in% c("baltimore", "baltimore city"))
balt_map <- get_map("Baltimore County", zoom = 10) %>%
  ggmap(extent = "device") + 
  geom_polygon(data = filter(balt_counties, our_counties == TRUE),
               aes(x = long, y = lat, group = group),
               fill = "red", color = "darkred", alpha = 0.2)
maryland_map <- balt_counties %>%
  ggplot(aes(x = long, y = lat, group = group, fill = our_counties)) + 
  geom_polygon(color = "black") + 
  scale_fill_manual(values = c("white", "darkred"), guide = FALSE) + 
  theme_void() + 
  coord_map()

grid.draw(ggplotGrob(balt_map))
md_inset <- viewport(x = 0, y = 0, 
                     just = c("left", "bottom"),
                     width = 0.35, height = 0.35)
pushViewport(md_inset)
grid.draw(rectGrob(gp = gpar(alpha = 0.5, col = "white")))
grid.draw(rectGrob(gp = gpar(fill = NA, col = "black")))
grid.draw(ggplotGrob(maryland_map))
popViewport()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809123428.png)


