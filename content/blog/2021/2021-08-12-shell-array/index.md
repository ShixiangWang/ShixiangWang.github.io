---
title: Shell：使用数组
author: 王诗翔
date: '2021-08-12'
slug: shell-array
categories:
  - Blog
tags:
  - Bash
description: 生成和操作shell数组。
keywords: bash
editor_options:
  chunk_output_type: console
---

之前使用Shell编程很少使用到数组，最近尝试使用后发现它在某些情况下非常有用。
这里简单介绍如何生成和使用数组。

## 生成数组

我们只要将一组空格分隔的序列用括号括起来，就生成了一个数组。

```bash
array=(a b c d e f g)
```

## 使用数组

### 输出数组

使用`${array[*]}`或`${array[@]}`输出全部元素：

```bash
bash-3.2$ echo ${array[*]}
a b c d e f g
bash-3.2$ echo ${array[@]}
a b c d e f g
```

在`array`前加`#`输出元素个数：

```bash
bash-3.2$ echo ${#array[*]}
7
```

获取元素将`*`改成索引，记得是从0开始：

```bash
bash-3.2$ echo ${array[1]}
b
bash-3.2$ echo ${array[2]}
c
```

注意在非引用的情况下不需要美元符号和花括号，例如修改元素：

```bash
bash-3.2$ array[2]=ff
bash-3.2$ echo ${array[2]}
ff
```

迭代数组：

```bash
#for i in "${!array[@]}"; do 
#  printf "%s\t%s\n" "$i" "${array[$i]}"
#done

bash-3.2$ for i in "${!array[@]}"; do 
>   printf "%s\t%s\n" "$i" "${array[$i]}"
> done
0	 a
1	 b
2	 ff
3	 d
4	 e
5	 f
6	 g
```

注意我们常用的序列生成命令`seq`并不会生成一个数组，哪怕你可以迭代它：

```bash
bash-3.2$ echo `seq 1 10`
1 2 3 4 5 6 7 8 9 10
bash-3.2$ a2=`seq 1 10`
bash-3.2$ echo ${a2}
1 2 3 4 5 6 7 8 9 10
bash-3.2$ echo $a2
1 2 3 4 5 6 7 8 9 10
bash-3.2$ echo ${a2[1]}

bash-3.2$ echo ${a2[2]}

bash-3.2$ echo ${a2[0]}
1 2 3 4 5 6 7 8 9 10
bash-3.2$ for i in $a2; do echo $i; done
1
2
3
4
5
6
7
8
9
10
```

下一节我们再看如何转换。

## 数据处理中利用数组

如果你有一定的数据分析经验，会比较容易发现上面的知识并不能带来什么用处。
在数据处理中使用数组，我们需要掌握一点技巧。

### 以可编程的方式引用数组元素

在实际处理时，我们一般不可能会手动地指定元素在所在数组中的索引。所以，我们需要一种办法做到。

其实也很简单，将索引直接用Shell变量替换即可：

```bash
bash-3.2$ idx=2
bash-3.2$ echo ${array[$idx]}
ff
```

使用时一定要注意不同符号的位置关系。

### 序列转换为数组

我们先看看怎么将`a2`转换为数组。

将序列转换为数组，还是使用`()`。

```bash
bash-3.2$ echo $a2
1 2 3 4 5 6 7 8 9 10
bash-3.2$ a3=($a2)
bash-3.2$ echo $a3
1
bash-3.2$ echo ${#a2[*]}
1
bash-3.2$ echo ${#a3[*]}
10
bash-3.2$ echo ${a3[*]}
1 2 3 4 5 6 7 8 9 10
```

> 注意美元符两侧不要加引号。

这样我们就能够愉快地将命令生成的序列数组化，然后单个获取和操作其中的元素了：

```bash
bash-3.2$ touch file{1..10}
bash-3.2$ files=(`ls file*`)
bash-3.2$ echo ${files[3]}
file3
```