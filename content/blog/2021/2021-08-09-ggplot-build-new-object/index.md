---
title: ggplot构建新图形元素
author: 王诗翔
date: '2021-08-09'
slug: ggplot-build-new-object
categories:
  - Blog
tags:
  - R
  - ggplot
  - 学习
description: 学习如何创建一个新的图形元素。
rmd_source: ''
keywords: rstats
editor_options:
  chunk_output_type: console
---

2021/05/24 19:35

> 原文：[https://bookdown.org/rdpeng/RProgDA/building-new-graphical-elements.html](https://bookdown.org/rdpeng/RProgDA/building-new-graphical-elements.html)


由ggplot构造的数据图中的关键元素包括geoms（几何对象）和stats（统计变换）。事实上，ggplot2包具有强大的功能，允许用户制作各种有趣而丰富的数据图形。这些图形可以通过组合调用各种`geom_*`和`stat_*`函数(以及其他类函数)来实现。

为什么要构造新的图形元素？

1. 实现ggplot2目前不存在的特性。

2. 简化复杂的工作流。如果你总是发现自己在用重复的代码绘制类似的图形元素。

创建新的geoms和stats可以简化代码，让用户轻松调整情节的某些元素，而不必每次都费劲地处理整个代码。

## 构造一个geom

ggplot2中的新geoms继承自一个名为Geom的顶级类，并使用两步流程构造：

1. `ggproto()`函数用于构造一个与新的geom对应的新类。这个新类指定了许多属性和函数，这些属性和函数描述了如何在图上绘制数据。

2. `geom_*`函数被构造为标准函数。这个函数返回一个层，该层可以添加到使用`ggplot()`函数创建的plot中。

新的geom类的基本设置如下所示：

```R
GeomNEW <- ggproto("GeomNEW", Geom,
        required_aes = <a character vector of required aesthetics>,
        default_aes = aes(<default values for certain aesthetics>),
        draw_key = <a function used to draw the key in the legend>,
        draw_panel = function(data, panel_scales, coord) {
                ## 返回一个grid grob对象的函数
                ## 是绘图真正工作的地方
        }
)
```


所需的美学映射应该很简单——例如，如果你的新geom生成了一种特殊的散点图，那么你可能需要x和y映射。美学映射的默认值可以包括绘图符号如形状、颜色等内容。

**实现** **draw_panel** **函数是创建一个新的geom的难点。在这里，你必须对grid包有一定的了解，才能访问基于grid系统的ggplot2图的底层元素。但是，你可以通过了解一些grid元素来实现一些可行的绘图内容。** 

`draw_panel`函数有三个参数。`data`元素是一个`data.frame`，其中包含每个指定美学映射的一列，`panel_scales`是一个包含关于当前面板的x和y比例信息的列表，`coord`是一个描述绘图坐标系的对象。`coord`和`panel_scales`对象除了转换数据以便绘制数据之外，没有什么用处。

下面是一个例子：

```R
library(grid)
GeomMyPoint <- ggproto("GeomMyPoint", Geom,
        required_aes = c("x", "y"),
        default_aes = aes(shape = 1),
        draw_key = draw_key_point,
        draw_panel = function(data, panel_scales, coord) {
                ## 先转换数据
                coords <- coord$transform(data, panel_scales)
                
                ## 打印 'coords' 对象
                str(coords)
                
                ## 构造一个 grid grob
                pointsGrob(
                        x = coords$x,
                        y = coords$y,
                        pch = coords$shape
                )
        })
```


除了创建一个新的Geom类之外，你还需要创建实际的函数，该函数将基于你的Geom规范构建一个层。在这里，我们将这个新函数称为`geom_mypoint()`，它是仿造在`geom_point()`函数构建的。

```R
geom_mypoint <- function(mapping = NULL, data = NULL, stat = "identity",
                         position = "identity", na.rm = FALSE, 
                         show.legend = NA, inherit.aes = TRUE, ...) {
        ggplot2::layer(
                geom = GeomMyPoint, mapping = mapping,  # 注意这里的 geom 设定为上面构造的类
                data = data, stat = stat, position = position, 
                show.legend = show.legend, inherit.aes = inherit.aes,
                params = list(na.rm = na.rm, ...)
        )
} 
```


现在我们就可以把这个函数应用起来了：

```R
ggplot(data = worldcup, aes(Time, Shots)) + geom_mypoint()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122057.png)

文字输出：

```R
'data.frame':   595 obs. of  5 variables:
 $ x    : num  0.0694 0.6046 0.3314 0.4752 0.1174 ...
 $ y    : num  0.0455 0.0455 0.0455 0.0791 0.1128 ...
 $ PANEL: Factor w/ 1 level "1": 1 1 1 1 1 1 1 1 1 1 ...
 $ group: int  -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 ...
 $ shape: num  1 1 1 1 1 1 1 1 1 1 ...
```


从`str()`输出中，我们可以看到coords对象包含了x和y映射，以及我们指定为默认的形状映射。注意，x和y都被重新调整为0到1之间。这是标准化的坐标体系。

## 例子：自动化透明geom

ggplot2散点图默认使用实心圆，数据多时便造成重叠问题。一个解决方案是我们构造一个能自动化增加透明度的几何对象，当数据少时，保持较高的不透明度，而数据多时则提高透明度。

先构造geom类：

```R
GeomAutoTransparent <- ggproto("GeomAutoTransparent", Geom,
        required_aes = c("x", "y"),
        default_aes = aes(shape = 19),
        draw_key = draw_key_point,
        draw_panel = function(data, panel_scales, coord) {
                ## 首先转换数据
                coords <- coord$transform(data, panel_scales)
                
                ## 基于数据点设定透明度
                n <- nrow(data)
                if(n > 100 && n <= 200)
                        coords$alpha <- 0.3
                else if(n > 200)
                        coords$alpha <- 0.15
                else
                        coords$alpha <- 1
                ## Construct a grid grob
                grid::pointsGrob(
                        x = coords$x,
                        y = coords$y,
                        pch = coords$shape,
                        gp = grid::gpar(alpha = coords$alpha)
                )
        })
```


接着创建一个`geom_`函数：

```R
geom_transparent <- function(mapping = NULL, data = NULL, stat = "identity",
                         position = "identity", na.rm = FALSE, 
                         show.legend = NA, inherit.aes = TRUE, ...) {
        ggplot2::layer(
                geom = GeomAutoTransparent, mapping = mapping,  
                data = data, stat = stat, position = position, 
                show.legend = show.legend, inherit.aes = inherit.aes,
                params = list(na.rm = na.rm, ...)
        )
}
```


现在我们可以使用它了：

```R
ggplot(data = worldcup, aes(Time, Shots)) + geom_transparent()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122106.png)

我们将数据点减少：

```R
ggplot(data = sample_n(worldcup, 50), aes(Time, Shots)) + 
        geom_transparent()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122114.png)

分面图也通过了向量化机制进行工作：

```R
ggplot(data = worldcup, aes(Time, Shots)) + 
        geom_transparent() + 
        facet_wrap(~ Position, ncol = 2) + 
        newtheme
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122120.png)

在这种情况下创建geom的优点是，它对计算进行了抽象，无需每次修改数据，并允许在绘图代码中进行更简单的通信。

## 构建一个统计变换 stat

除了geom，我们还可以在ggplot2中构建一个新的stat，它可以用来**抽象出在绘图中创建/绘制geom时可能需要的任何计算** 。将geom可能需要的任何复杂计算分离出来，可以简化以后编写geom的过程。

创建一个stat看起来有点像创建一个geom，但需要指定不同的函数和类。与创建geom类似，我们需要使用`ggproto()`函数创建一个通常从Stat类继承的新类。然后，我们需要指定一个`stat_*`函数，它将创建ggplot2和相关`geom_*`函数使用的层。

下面是模板：

```R
StatNEW <- ggproto("StatNEW", Stat,
                   compute_group = <a function that does computations>,
                   default_aes = aes(<default values for certain aesthetics>),
                   required_aes = <a character vector of required aesthetics>)
```


**stat的最终目标是以某种方式呈现数据，使其适合绘图** 。为此，`compute_group()`函数必须返回一个`data.frame`，以便ggplot2（通常需要`data.frame`）中的绘图机制知道该做什么。

如果你的stat输出可以作为一个标准的/已经存在的geom的输入，那么你就不需要编写一个自定义的geom。你的stat只需要以现有geom能够识别的方式格式化它的输出。例如，如果你希望以一种特殊的方式呈现数据，但最终将其绘制为多边形，则可以利用现有的`geom_polygon()`函数。

## 例子：标准置信区间

在数据分析或统计建模过程中，一个常见的任务是绘制一组参数估计数以及这些点的95%置信区间。给定一个估计值和一个标准误差，基本统计理论认为，我们可以用估计值加上/减去1.96倍的标准误差来近似这个参数的95%置信区间。我们可以构建一个简单的统计，它采用一个估计和标准误差，并构建`geom_segment()`所需的数据，以便绘制大约95%的置信区间。

让我们用R自带的空气质量数据集来计算臭氧的月平均水平以及这些月平均水平的标准误差。

```R
library(datasets)
library(dplyr)
data("airquality")
monthly <- dplyr::group_by(airquality, Month) %>%
        dplyr::summarize(ozone = mean(Ozone, na.rm = TRUE),
                  stderr = sd(Ozone, na.rm = TRUE) / sqrt(sum(!is.na(Ozone))))
monthly
# A tibble: 5 x 3
  Month ozone stderr
  <int> <dbl>  <dbl>
1     5  23.6   4.36
2     6  29.4   6.07
3     7  59.1   6.20
4     8  60.0   7.78
5     9  31.4   4.48
```


下面是一个简单图形：

```R
ggplot(monthly, aes(x = Month, y = ozone)) + 
        geom_point() + 
        ylab("Ozone (ppb)")
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122127.png)

但是上面的图并没有显示我们所期望的月均值的变化。我们可以创建一个stat来完成这项工作，并将信息提供给`geom_segment()`。首先，我们需要回忆一下，`geom_segment()`需要美学映射x、xend、y和yend，它们指定每个线段的开始和结束点。因此，我们的stat也应该指定这些信息。在`ggproto()`调用中定义的`compute_group()`函数应该提供这个功能。

```R
StatConfint <- ggproto("StatConfint", Stat,
                       compute_group = function(data, scales) {
                               ## Compute the line segment endpoints
                               x <- data$x
                               xend <- data$x
                               y <- data$y - 1.96 * data$stderr
                               yend <- data$y + 1.96 * data$stderr
                               
                               ## Return a new data frame
                               data.frame(x = x, xend = xend,
                                          y = y, yend = yend)
                       },
                       required_aes = c("x", "y", "stderr")
)
```


接下来我们定义一个单独的`stat_*`函数来构造可叠加的层：

```R
stat_confint <- function(mapping = NULL, data = NULL, geom = "segment",
                           position = "identity", na.rm = FALSE, 
                           show.legend = NA, inherit.aes = TRUE, ...) {
        ggplot2::layer(
                stat = StatConfInt, 
                data = data, 
                mapping = mapping, 
                geom = geom, 
                position = position, 
                show.legend = show.legend, 
                inherit.aes = inherit.aes,
                params = list(na.rm = na.rm, ...)
        )
}
```


这样我们就可以使用它了：

```R
ggplot(data = monthly, aes(x = Month, y = ozone, stderr = stderr)) + 
        geom_point() + 
        ylab("Ozone (ppb)") + 
        geom_segment(stat = "confint")
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122134.png)

## 结合geom和stat

结合geoms和stats，你可以创建新的图形元素，并使用你定义的特殊计算。此外，如果你需要一些自定义绘图，而现有的`geom`不能立即处理，那么你可以考虑编写一个单独的geom来处理由你的统计计算出的数据。在本节中，我们将展示如何结合统计和geom来创建自定义绘图。

我们将使用的示例是创建一个“瘦盒图”，它看起来像这样。

```R
## This code is not runnable yet!
library(ggplot2)
library(datasets)
data(airquality)
mutate(airquality, Month = factor(Month)) %>%
        ggplot(aes(Month, Ozone)) + 
        geom_skinnybox()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122140.png)

这个boxplot与传统的boxplot（例如`geom_boxplot()`）在以下方面有所不同：

1. “须”延伸到数据的最小值和最大值。

2. 中位数用点表示，而不是直线。

3. 这里没有一个方框表示第25和第75百分位数之间的区域。

首先，我们可以创建一个stat，从数据中计算相关的汇总统计：最小值、第一个四分位数、中位数、第三个四分位数和最大值。

```R
StatSkinnybox <- ggproto("StatSkinnybox", Stat, 
                         compute_group = function(data, scales) {
                                 probs <- c(0, 0.25, 0.5, 0.75, 1)
                                 qq <- quantile(data$y, probs, na.rm = TRUE) 
                                 out <- qq %>% as.list %>% data.frame
                                 names(out) <- c("ymin", "lower", "middle", 
                                                 "upper", "ymax")
                                 out$x <- data$x[1]
                                 out
                         },
                         required_aes = c("x", "y")
                         )

stat_skinnybox <- function(mapping = NULL, data = NULL, geom = "skinnybox",
                           position = "identity", show.legend = NA, 
                           outliers = TRUE, inherit.aes = TRUE, ...) {
        ggplot2::layer(
                stat = StatSkinnybox, 
                data = data, 
                mapping = mapping, 
                geom = geom, 
                position = position, 
                show.legend = show.legend, 
                inherit.aes = inherit.aes,
                params = list(outliers = outliers, ...)
        )        
}
```


有了可用来处理数据的统计，我们就可以继续编写geom了。这组函数负责在绘图区域中绘制适当的图形。首先，我们可以使用`ggproto()`函数创建GeomSkinnybox类。在这里，关键函数是`draw_panel()`函数，由于代码的长度，我们单独实现它。注意，在`draw_panel_function()`函数中，我们需要手动调整boxplot的“lower”、“upper”和“middle”部分，否则它们将不会出现在绘图中（它们将在错误的单元中）。

```R
library(scales)
draw_panel_function <- function(data, panel_scales, coord) {
        coords <- coord$transform(data, panel_scales) %>%
                mutate(lower = rescale(lower, from = panel_scales$y.range),
                       upper = rescale(upper, from = panel_scales$y.range),
                       middle = rescale(middle, from = panel_scales$y.range))
        med <- pointsGrob(x = coords$x,
                          y = coords$middle,
                          pch = coords$shape)
        lower <- segmentsGrob(x0 = coords$x,
                              x1 = coords$x,
                              y0 = coords$ymin,
                              y1 = coords$lower,
                              gp = gpar(lwd = coords$size))
        upper <- segmentsGrob(x0 = coords$x,
                              x1 = coords$x,
                              y0 = coords$upper,
                              y1 = coords$ymax,
                              gp = gpar(lwd = coords$size))
        gTree(children = gList(med, lower, upper))
}

GeomSkinnybox <- ggproto("GeomSkinnybox", Geom,
                         required_aes = c("x", "ymin", "lower", "middle", 
                                          "upper", "ymax"),
                         default_aes = aes(shape = 19, lwd = 2),
                         draw_key = draw_key_point,
                         draw_panel = draw_panel_function
                         )
```


最后，我们实现`geom_skinnybox()`函数，它从`stat_skinnybox()`函数和GeomSkinnybox类中绘制。

```R
geom_skinnybox <- function(mapping = NULL, data = NULL, stat = "skinnybox", 
                           position = "identity", show.legend = NA, 
                           na.rm = FALSE, inherit.aes = TRUE, ...) {
        layer(
                data = data, 
                mapping = mapping,
                stat = stat,
                geom = GeomSkinnybox,
                position = position,
                show.legend = show.legend,
                inherit.aes = inherit.aes,
                params = list(na.rm = na.rm, ...)
        )
}
```


这样我们就可以使用它了：

```R
mutate(airquality, Month = factor(Month)) %>%
        ggplot(aes(Month, Ozone)) + 
        geom_skinnybox()
```


![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122148.png)


