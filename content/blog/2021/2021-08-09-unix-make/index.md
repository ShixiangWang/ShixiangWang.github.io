---
title: Unix：Make
author: 王诗翔
date: '2021-08-09'
slug: unix-make
categories:
  - Blog
tags:
  - Unix
  - 自动化
  - 学习
description: 自动化Unix命令构建。
keywords: make
editor_options:
  chunk_output_type: console
---

2021/05/26 22:01

> 原文：[https://seankross.com/the-unix-workbench/working-with-unix.html#make](https://seankross.com/the-unix-workbench/working-with-unix.html#make)


从前没有网页浏览器、文件浏览器、开始菜单或搜索栏。当有人启动电脑时，他们得到的是一个shell提示符，他们做的所有工作都是从这个提示符开始的。那时候，人们仍然喜欢共享软件，但总是存在软件应该如何安装的问题。make程序是解决这个问题的最好的尝试，make的优雅使它至今仍被广泛使用。make的指导设计目标是为了安装一些新软件:

1. 将所有的依赖下载到一个目录。

2. `cd`进入目录。

3. 运行`make`。

这是通过指定一个名为makefile的文件来实现的，**该文件描述了不同文件和程序之间的关系** 。除了安装程序之外，make对于自动创建文档也很有用。让我们构建一个makefile，它创建一个readme.txt文件，该文件自动填充有关当前目录的一些信息。

首先进入目录并创建一个文件：

```Bash
▶ cd ~/Documents/test                                                                                                         
▶ nano makefile
```


填入如下内容：

```纯文本
draft_journal_entry.txt:
  touch draft_journal_entry.txt
```


上面简单的makefile说明了一个规则，它的一般格式如下：

```纯文本
[target]: [dependencies...]
  [commands...]
```


在这个简单的示例中，我们创建了以`draft_journal_entry.txt`为目标的文件，该文件是作为命令的结果创建的。需要注意的是，目标下的任何命令都必须用`Tab`缩进。如果我们不使用`Tab` 来缩进命令，那么`make`将失败。让我们保存并关闭makefile，然后我们可以在控制台中运行以下命令：

```Bash
▶ ls
makefile
```


然后使用下面的操作查看`make`使用方式：

```Bash
▶ make draft_journal_entry.txt
touch draft_journal_entry.txt                                                                                                     
▶ ls
draft_journal_entry.txt makefile
```


在我们为`draft_journal_entry.txt`目标定义的规则下缩进的命令已经执行，所以现在`draft_journal_entry.txt`存在！让我们再次运行相同的`make`命令：

```bash
▶ make draft_journal_entry.txt
make: `draft_journal_entry.txt' is up to date.
```


因为目标文件已经存在，所以没有采取任何操作，相反，我们被告知，`draft_journal_entry.txt`的规则是“最新的”(没有什么需要做的)。

如果我们看一下我们之前草拟的一般规则格式，我们可以看到我们没有为该规则指定任何依赖项。依赖项是目标在构建时所依赖的文件。如果自上次为目标运行`make`以来，依赖项已经更新，则目标不是“最新的”。这意味着下次为该目标运行`make`时将运行该目标的命令。通过这种方式，对依赖项的更改被合并到目标中。为了避免不必要地运行命令，这些命令只在依赖项改变时运行，或者当目标根本不存在时运行。

让我们更新`makefile`，以包含自动生成的`readme.txt`。首先，让我们添加文件：

```bash
▶ echo "1. 2017-06-15-In-Boston" > toc.txt
```


`makefile`修改为：

```纯文本
draft_journal_entry.txt:
  touch draft_journal_entry.txt
  
readme.txt: toc.txt
  echo "This journal contains the following number of entries:" > readme.txt
  wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt
```


运行下面命令：

```bash
▶ make readme.txt
echo "This journal contains the following number of entries:" > readme.txt
wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt
```


看一下`readme.txt`：

```bash
▶ cat readme.txt 
This journal contains the following number of entries:
1
```


再次运行：

```bash
▶ make readme.txt 
make: `readme.txt' is up to date.
```


修改`toc.txt`后再次运行：

```bash
▶ echo "2. 2017-06-16-IQSS-Talk" >> toc.txt
▶ make readme.txt
echo "This journal contains the following number of entries:" > readme.txt
wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt

▶ cat readme.txt                           
This journal contains the following number of entries:
2 
```


为了简化`make`体验，我们可以在`makefile`的顶部创建一个名为`all`的规则，在该规则中我们可以列出由`makefile`构建的所有文件。通过添加`all`目标，我们可以在不带任何参数的情况下运行`make`来构建`makefile`中的所有目标。让我们打开nano并添加以下规则：

```纯文本
all: draft_journal_entry.txt readme.txt

draft_journal_entry.txt:
  touch draft_journal_entry.txt
  
readme.txt: toc.txt
  echo "This journal contains the following number of entries:" > readme.txt
  wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt
```


同时让我们在`makefile`的末尾添加另一个特殊的规则`clean`，它会销毁由我们的`makefile`创建的文件：

```纯文本
all: draft_journal_entry.txt readme.txt

draft_journal_entry.txt:
  touch draft_journal_entry.txt
  
readme.txt: toc.txt
  echo "This journal contains the following number of entries:" > readme.txt
  wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt
  
clean:
  rm draft_journal_entry.txt
  rm readme.txt
```


首先让我们清理下我们的目录：

```bash
▶ make clean
rm draft_journal_entry.txt
rm readme.txt                                                                                                     
▶ ls
makefile toc.txt
```


然后重新构建：

```bash
▶ make
touch draft_journal_entry.txt
echo "This journal contains the following number of entries:" > readme.txt
wc -l toc.txt | egrep -o "[0-9]+" >> readme.txt
                                                                                                     
▶ ls
draft_journal_entry.txt makefile                readme.txt              toc.txt
```




