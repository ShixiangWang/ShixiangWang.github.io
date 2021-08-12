---
title: RNAseq原理与分析课程笔记
author: 王诗翔
date: '2021-08-12'
slug: rnaseq-cource-notes
categories:
  - Blog
  - 课程
tags:
  - RNA-seq
  - 笔记
description: 之前购买的腾讯课堂课程做做扫盲笔记。
keywords: RNA-seq
editor_options:
  chunk_output_type: console
---

很久之前在腾讯课堂购买了孟浩巍的RNAseq课程，讲解的挺细致完备的。这里主要是很少做这样的分析，所以一方面扫盲，一方面记录下其中重要的点（自己不知道的）或者值得拓展了解的知识点。

## 参考基因组与基因分布

![image-20210812121053382](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812121053.png)

拼装失败的地方大都是rRNA位置区域，与后面的内容对应。

![image-20210812121124166](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812121124.png)

## rRNA

这里有几个点：

1. 一个DNA分子是比一个氨基酸大的
2. rRNA是集中分布的
3. rRNA在全部RNA中的占比很高，达80%左右。所以mRNA测序要么富集poly-A，要么去掉rRNA。

![image-20210812121151184](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812121151.png)



![image-20210812121213271](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812121213.png)

![image-20210812121233241](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812121233.png)


## 测序的概念

![image-20210812121253398](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812122355.png)

双端测序涉及到两端的转换。补个截图增强理解。

![image-20210812121308991](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812122407.png)





