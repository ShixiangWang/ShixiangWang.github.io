---
title: 关于区间突变概率计算理解的纠正
author: 王诗翔
date: '2021-08-12'
slug: adjustment-for-mutaiton-prob-in-a-region
categories:
  - Blog
tags:
  - R
  - Bioinformatics
description: 学习有时候就是在跟自己反复较量。
---

> 资料来源：<https://www.huber.embl.de/msmb/Chap-Generative.html>，我把资料中的情景更改了下。

假设50个样本有100个碱基，单个位点有百分之一的突变率，汇总50个样本，我们期望在任何给定位置，50个样本观测到突变次数的总和服从具有参数为0.5的泊松分布。

一个随机的图如下：

![image-20210812200203540](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812200203.png)

现在让我们假设实际观测的图如下：

![image-20210812200244466](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812200244.png)

这种情况的概率是多少呢？

让我们先看一下至少出现7次的概率：
$$
\begin{equation*}
P(X\geq 7)= \sum_{k=7}^\infty P(X=k).
\end{equation*}
$$
它可以转变为1减去出现少于7次的概率。

在R里面可以计算：

![image-20210812200423825](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812200533.png)

我们假设我们最后要计算得到的概率为$\epsilon$：
$$
\begin{equation*}
\epsilon=P(X\geq 7)=1-P(X\leq 6)\simeq10^{-6}.
\end{equation*}
$$

**停！上面是错误的计算！**

上面我们看了100个位置，寻找最大值并发现它是7，这种情况下出现7的概率比单个位置出现7的概率要大！

这里我们使用极端值分析，先对每个位置出现的次数排序，然后重新命名。

![image-20210812200945523](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812200945.png)

那么最大值出现至少7次的概率可以采用互补计算：
$$
\begin{equation*}
\begin{aligned}
 P(x_{(100)}\geq 7)
&=&1-P(x_{(100)} \leq 6)\\\\
&=&1-P(x_{(1)}\leq 6 )\times P(x_{(2)}\leq 6 )\times \cdots \times P(x_{(100)} \leq 6 )\\\\
&=&1-P(x_1\leq 6 )\times P(x_2\leq 6 )\times \cdots \times P(x_{100}\leq 6 )\\\\
&=&1-\prod_{i=1}^{100} P(x_i \leq 6 ).\end{aligned}
\end{equation*}
$$
由于100个事件是独立的，所以转换为：
$$
\begin{equation*}
\prod_{i=1}^{100} P(x_i \leq 6)=
\left(P(x_i \leq 6)\right)^{100}=
\left(1-\epsilon\right)^{100}.
\end{equation*}
$$
为了计算这个值，我们使用二项公式然后进行逼近计算：

![image-20210812201240461](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812201240.png)

那么最后的概率是1减去这个值，所以结果是$10^{-4}$。

我们还可以进一步通过蒙特卡洛模拟计算该值：

![image-20210812201455190](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210812201455.png)

这个结果与我们前面的计算基本一致。

