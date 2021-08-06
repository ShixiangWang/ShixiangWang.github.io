---
title: Slurm使用技巧
author: 王诗翔
date: '2021-08-06'
slug: slurm-tricks
categories:
  - Blog
tags:
  - HPC
  - slurm
description: 介绍下自己在使用slurm进行工作的一些经验和技巧。
rmd_source: ''
keywords: slurm
editor_options:
  chunk_output_type: console
---

Slurm的官方文档在：<https://slurm.schedmd.com/>。

这篇文章记录下我在工作中使用经常用到，但是一般的初级文档不会涉及的内容、操作。

## 系统节点资源概览

一般我们输入`sinfo`，只能看到非常精简的节点信息：

```bash
$ sinfo
PARTITION AVAIL  TIMELIMIT  NODES  STATE NODELIST
cn           up   infinite      1    mix cn03
cn           up   infinite      2  alloc cn[01-02]
cn           up   infinite     15   idle cn[04-18]
fat          up   infinite      1   idle fat01
gpu*         up   infinite      1   idle gpu01
```

我们可以添加几个选项来获得一个更为完备的信息：

```bash
$ sinfo -Nel
Fri Aug  6 15:54:17 2021
NODELIST   NODES PARTITION       STATE CPUS    S:C:T MEMORY TMP_DISK WEIGHT AVAIL_FE REASON              
cn01           1        cn   allocated   80   80:1:1 771000        0      1   (null) none                
cn02           1        cn   allocated   80   80:1:1 771000        0      1   (null) none                
cn03           1        cn       mixed   80   80:1:1 771000        0      1   (null) none                
cn04           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn05           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn06           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn07           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn08           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn09           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn10           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn11           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn12           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn13           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn14           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn15           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn16           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn17           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
cn18           1        cn        idle   80   80:1:1 771000        0      1   (null) none                
fat01          1       fat        idle   80   80:1:1 296649        0      1   (null) none                
gpu01          1      gpu*        idle   40   40:1:1 772000        0      1   (null) none   
```

全部的选项介绍在文档[*sinfo*](https://slurm.schedmd.com/sinfo.html)中。

## Job Array

之前使用PBS任务系统工作过，没有看到可以快速提交和处理相似任务脚本的命令，我手动写过脚本去批量生成任务然后提交。这种工作其实
很常见，例如要处理好几百个样本，每个样本都需要跑某一（多）个软件程序。因此每个样本处理可能除了文件名不同，
其需要的计算资源是一致的。

[*Batch Submission of Serial Jobs for Parallel Execution*](https://portal.supercomputing.wales/index.php/index/slurm/interactive-use-job-arrays/batch-submission-of-serial-jobs-for-parallel-execution/)这篇文章介绍了3种处理策略，我最喜欢最后一种，因为官方工具原生支持了批处理任务的提交。

下面是来自[*Job Arrays*](https://portal.supercomputing.wales/index.php/index/slurm/interactive-use-job-arrays/job-arrays/)一文的例子，我们可以直接拿来作模板使用。

`array.sh`：

```bash
#!/bin/bash

#SBATCH -J arraytest
#SBATCH --array=0-4
#SBATCH -o output-%A_%a-%J.o
#SBATCH -n 1

echo SLURM_JOB_ID $SLURM_JOB_ID
echo SLURM_ARRAY_JOB_ID $SLURM_ARRAY_JOB_ID
echo SLURM_ARRAY_TASK_ID $SLURM_ARRAY_TASK_ID
```

这个脚本的核心在`--array`选项，它定义了一个数组，这个数组的每个元素会传入到`$SLURM_ARRAY_TASK_ID`这个变量中，利用这个变量我们就能处理不同样本的任务。该选项也可以在`sbatch`提交任务时指定，非常灵活。
在头文件中定义的资源是每个任务通用的，比如上面指定使用一个CPU，那么有100个任务的话，每个任务1个，就是100个。

日志文件会是下面的格式：

```
output-231_0-232.o
output-231_1-233.o
output-231_2-234.o
output-231_3-235.o
output-231_4-231.o
```

输出文件内容会是下面的信息格式。

`output-231_1-233.o`：

```
SLURM_JOB_ID 233
SLURM_ARRAY_JOB_ID 231
SLURM_ARRAY_TASK_ID 1
```

相关环境变量汇总如下：

| BASH Environment Variable | SBATCH Field Code | Description                                |
| :------------------------ | :---------------- | :----------------------------------------- |
| $SLURM_JOB_ID             | %J                | Job identifier                             |
| $SLURM_ARRAY_JOB_ID       | %A                | Array parent job identifier                |
| $SLURM_ARRAY_TASK_ID      | %a                | Array job iteration index                  |
| $SLURM_ARRAY_TASK_COUNT   |                   | Number of indexes (tasks) in the job array |
| $SLURM_ARRAY_TASK_MAX     |                   | Maximum array index                        |
| $SLURM_ARRAY_TASK_MIN     |                   | Minimum array index                        |

更多的控制需要看[*Job Array Support*](https://slurm.schedmd.com/job_array.html)这篇文档。