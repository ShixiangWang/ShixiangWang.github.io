---
title: 使用Circle-Map Realign鉴定环形DNA
author: 王诗翔
date: '2021-08-09'
slug: use-circle-map-for-detecting-eccdna
categories:
  - Blog
tags:
  - Bioinformatics
  - 学习
description: 根据官方教程学习使用软件。
rmd_source: ''
editor_options:
  chunk_output_type: console
---

2021/07/23 19:39

> 原文：[https://github.com/iprada/Circle-Map/wiki/Tutorial:-Identification-of-circular-DNA-using-Circle-Map-Realign](https://github.com/iprada/Circle-Map/wiki/Tutorial:-Identification-of-circular-DNA-using-Circle-Map-Realign)


这是一个教程，一步一步地解释如何从原始数据(fastq文件)到一个可解释的，标签分离的bed文件指示染色体坐标的DNA环。为了制作教程，我们模拟了Illumina读取来自人类基因组未知区域的环形DNA。本教程的目的是使用Circle-Map来提取环形DNA的来源。

## 依赖

- GNU/Linux

- BWA

- samtools

- Circle-Map

```bash
conda install -c bioconda bwa samtools
pip3 install Circle-Map
pip3 install biopython==1.77 # 不安装这个版本目前使用会报错

```


## 第一步：准备和下载数据

### 下载原始数据

直接克隆仓库：

```Bash
git clone https://github.com/iprada/Circle-Map
```



### 下载和准备参考基因组

```bash
wget http://hgdownload.soe.ucsc.edu/goldenPath/hg38/bigZips/hg38.fa.gz
gunzip -d hg38.fa.gz

bwa index hg38.fa
samtools faidx hg38.fa

```



## 第二步：比对到参考基因组

```bash
cd Circle-Map/tutorial/
bwa mem -q ~/data/refs/hg38.fa unknown_circle_reads_1.fastq unknown_circle_reads_2.fastq > unknown_circle.sam

```


这就产生了一个SAM文件，其中包含了关于reads在哪里以及如何与基因组对齐的信息。

- 我们**使用** **-q** **选项（query name sorted）为BWA中的拆分读对比对分配独立的映射质量分数** 。这可以改进Circle-Map概率模型中断点图权值的估计。

- `-t`可以指定多线程。


## 第三步：准备Circle-Map所需的文件

按read名排序并转换为BAM：

```bash
samtools sort -n -o qname_unknown_circle.bam unknown_circle.sam

```


**Circle-Map利用该文件提取环形DNA候选reads，并估计先验分布的参数。** 

我们还需要按最左边的映射坐标对SAM文件进行排序：

```bash
samtools sort -o sorted_unknown_circle.bam unknown_circle.sam

```


Circle-Map使用这个文件来计算基因组覆盖率指标。

- `-@` 选项指定多线程。


现在，我们将使用Circle-Map提取环形DNA重排的reads（部分映射reads和不一致reads对）：

```bash
Circle-Map ReadExtractor -i qname_unknown_circle.bam -o circular_read_candidates.bam

```


这一步报错了，安装github上最新版本也没有解决。

```bash
$ Circle-Map ReadExtractor -i qname_unknown_circle.bam -o circular_read_candidates.bam
[E::idx_find_and_load] Could not retrieve index file for '/public/home/zhaoqi/test/Circle-Map/tutorial/qname_unknown_circle.bam'
Extracting circular structural variants
finished extracting reads. Elapsed time: 0.00025257269541422524 mins
Thanks for using Circle-Map
```


> 已提交 issue，先试着后面的步骤能否走下去。


> **实践证明这里的错误似乎可以忽略（因为参考数据可以得到示例的结果）** 。



按坐标排序：

```BASH
samtools sort -o sort_circular_read_candidates.bam circular_read_candidates.bam

```



构建索引：

```bash
samtools index sort_circular_read_candidates.bam
samtools index sorted_unknown_circle.bam

```



## 第四步：检测环形DNA

所需要的文件：

- **sort_circular_read_candidates.bam** : The circular DNA read candidates. Used for detecting the circular DNA

- **qname_unknown_circle.bam** : The read name sorted BAM file. Used for calculating the realignment prior

- **sorted_unknown_circle.bam** : The coordinate sorted BAM. Used for calculating circular DNA coverage metrics

- **hg38.fa** : Reference sequence. Used for realignment of the partially mapped reads.


执行命令：

```bash
Circle-Map Realign -i sort_circular_read_candidates.bam -qbam qname_unknown_circle.bam -sbam sorted_unknown_circle.bam -fasta ~/data/refs/hg38.fa -o my_unknown_circle.bed

```


- 使用 `-t` 指定多线程。

这里运行时报错没有bedtools，补装软件：

```bash
mamba install -c bioconda bedtools
```



能看到结果：

![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122747.png)

对比参考输出：

![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809122755.png)


除了第一个坐标大了1位，其他结果一样。


