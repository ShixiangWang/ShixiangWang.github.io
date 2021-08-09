---
title: Unix：Bash编程
author: 王诗翔
date: '2021-08-09'
slug: unix-bash-programming
categories:
  - Blog
tags:
  - Unix
  - Bash
  - Linux
  - 学习
description: 一文熟悉Bash基础。
editor_options:
  chunk_output_type: console
---

> 原文：[https://seankross.com/the-unix-workbench/bash-programming.html](https://seankross.com/the-unix-workbench/bash-programming.html)


## 数学

创建`math.sh`：

```bash
#!/usr/bin/env bash
# File: math.sh

expr 5 + 2
expr 5 - 2
expr 5 \* 2 # 转义
expr 5 / 2
```


保存然后运行：

```Bash
$ bash math.sh
7
3
10
2 
```


⚠️注意，bash使用整除法。求余使用`%`符号。

如果我们想要进行更为复杂的数学计算，使用`bc`命令。

创建文件`bigmath.sh`：

```bash
#!/usr/bin/env bash
# File: bigmath.sh

echo "22 / 7" | bc -l
echo "4.2 * 9.15" | bc -l
echo "(6.5 / 0.5) + (6 * 2.2)" | bc -l
```


结果：

```bash
$ bash bigmath.sh
3.14285714285714285714
38.430
26.2

```


注意，如果不指定`-l` 选项，返回的就是整数结果：

```Bash
$ echo "22 / 7" | bc
3 
```


## 变量

变量用于存储数据，赋值使用等号：

```bash
chapter_number=5
```


注意等号两侧不能有空格，这与一般的编程语言不同。也就是说，下面的非法的：

```bash
chapter_number = 5
```


使用`let`命令修改变量：

```bash
let chapter_number=$chapter_number+1
echo $chapter_number
# 6 
```


存储字符串也是可以的：

```bash
the_empire_state="New York"
echo $the_empire_state
# New York 
```


有时，你可能希望像在命令行上那样运行命令，并将该命令的结果存储在一个变量中。我们可以通过将命令用美元符号和圆括号（`$()`）括起来来实现这一点。这种语法叫作**命令替换** 。例如：

```bash
math_lines=$(cat math.sh | wc -l)
echo $math_lines
# 7 
```


带有美元符号的变量名也可以在其他字符串中使用，以便将变量的值插入到字符串中：

```bash
echo "I went to school in $the_empire_state."
# I went to school in New York. 
```


在编写Bash脚本时，脚本会免费提供一些变量。让我们用以下代码创建一个名为`vars.sh`的新文件：

```bash
#!/usr/bin/env bash
# File: vars.sh

echo "Script arguments: $@"
echo "First arg: $1. Second arg: $2."
echo "Number of arguments: $#"
```


上面几个变量可以获取全部参数、指定参数（`$2`获取第2个参数，以此类推）以及参数的数目。

```bash
$ bash vars.sh red
Script arguments: red
First arg: red. Second arg: .
Number of arguments: 1 
```


## 用户输入

如果你正在为自己或其他人编写Bash程序，那么获取用户输入的一种方式就是指定用户提供给程序的参数，正如我们在前一节中讨论的那样。你还可以通过使用`read`命令暂时停止程序的执行，要求用户在命令行上输入一个字符串。让我们写一个小脚本，你可以看到`read`命令是如何工作的：

```bash
#!/usr/bin/env bash
# File: letsread.sh

echo "Type in a string and then press Enter:"
read response
echo "You entered: $response"
```


让我们运行脚本，然后输入一个字符串：

```bash
$ bash letsread.sh
Type in a string and then press Enter:
Hello!
You entered: Hello! 
```


## 逻辑控制

### 条件执行

如何编程语言都有逻辑值，表示是`true`与否`false`。

在具体看怎么使用判断语句前，我们先看下退出状态码。

运行命令：

```bash
$ this_command_does_not_exist
Error in running command bash 
```


当每一条bash命令运行后，会隐式地将命令运行的状态结果存储在一个变量`$?`中：

```bash
$ echo $?
127 
```


这个整数就是状态码。

这个特殊的退出状态向shell发出指示，它应该向控制台打印错误消息。成功运行的程序的退出状态是什么？让我们一起来看看：

```Bash
$ echo I will succeed.
I will succeed.
$ echo $?
0 
```


成功的状态码是`0`。我们再看看逻辑值：

```bash
$ true
$ echo $?
0
$ false
$ echo $?
1 
```


如您所见，`true`的退出状态为0，而`false`的退出状态为1。由于这些程序不做其他事情，你可以将`true`定义为始终具有退出状态0的程序，将`false`定义为始终具有退出状态1的程序。

在讨论逻辑操作符AND操作符（`&&`）和OR操作符（`||`）时，了解这些程序的退出状态很重要。AND和OR运算符可用于命令行程序的条件执行。当一个程序的执行依赖于另一个程序的退出状态时，就会发生条件执行。例如，在AND操作符的情况下，只有当`&&`左边的程序的退出状态为0时，`&&`右边的程序才会被执行。让我们来看看一些小的例子：

```Bash
$ true && echo "Program 1 was executed."
Program 1 was executed.
$ false && echo "Program 2 was executed."
 
```


在由AND操作符连接在一起的一系列程序中，如果左侧程序退出状态是非零，右侧则不执行。

OR运算符（`||`）遵循一组类似的原理。只有当左边的命令失败，退出状态不是0时，`||`右边的命令才会被执行。让我们来看看它是如何工作的：

```bash
$ true || echo "Program 1 was executed."

$false || echo "Program 2 was executed."
Program 2 was executed. 
```


你可以组合多个OR操作符，这样只有第一个退出状态为0的程序才会被执行。

### 条件表达式

让Bash脚本能够做出决策是非常有用的。条件执行允许你根据某些程序的成功或失败来控制执行这些程序的情况，但你也可以构造条件表达式，即等价于`true`或`false`的逻辑语句。条件表达式可以比较两个值，也可以对一个值提出问题。条件表达式总是在双括号（`[[]]`）之间，它们要么使用逻辑标志，要么使用逻辑操作符。例如，可以使用一些逻辑标志来比较两个整数。如果我们想知道一个整数是否大于另一个，我们可以使用`-gt`大于标志。在命令行中输入这个简单的条件表达式：

```bash
$ [[ 4 -gt 3 ]]
$ echo $?
0 
```


这些二进制逻辑表达式比较两个值，但也有只查看一个值的一元逻辑表达式。例如，你可以使用`-e`逻辑标志来测试文件是否存在。让我们来看看这个选项的作用：

```bash
$ [[ -e math.sh ]] && echo t || echo f
t 
```


全部的逻辑标志如下：

|Logical Flag|Meaning|Usage|
|---|---|---|
|-gt|**G** reater **T** han|`[[ $planets -gt 8 ]]`|
|-ge|**G** reater Than or **E** qual To|`[[ $votes -ge 270 ]]`|
|-eq|**Eq** ual|`[[ $fingers -eq 10 ]]`|
|-ne|**N** ot **E** qual|`[[ $pages -ne 0 ]]`|
|-le|**L** ess Than or **E** qual To|`[[ $candles -le 9 ]]`|
|-lt|**L** ess **T** han|`[[ $wives -lt 2 ]]`|
|-e|A File **E** xists|`[[ -e $taxes_2016 ]]`|
|-d|A **D** irectory Exists|`[[ -d $photos ]]`|
|-z|Length of String is **Z** ero|`[[ -z $name ]]`|
|-n|Length of String is **N** on-Zero|`[[ -n $name ]]`|



除了逻辑标志之外，还有逻辑操作符。最有用的逻辑操作符之一是正则表达式匹配操作符`=~`。正则表达式匹配操作符将字符串与正则表达式进行比较，如果字符串与正则表达式匹配，则表达式等价于true，否则等价于false。让我们用几种不同的方法来测试这个操作符：

```bash
$ [[ rhythms =~ [aeiou] ]] && echo t || echo f
f
$ my_name=sean
$ [[ $my_name =~ ^s.+n$ ]] && echo t || echo f
t 
```


还有一个非运算符`!`，它反转任何条件表达式的值。NOT操作符将真表达式转换为假表达式，反之亦然。让我们看几个使用NOT操作符的例子：

```bash
$ [[ 7 -gt 2 ]] && echo t || echo f
t
$ [[ ! 7 -gt 2 ]] && echo t || echo f
f
$ [[ 6 -ne 3 ]] && echo t || echo f
t
$ [[ ! 6 -ne 3 ]] && echo t || echo f
f 
```


全部逻辑操作符：

|Logical Operator|Meaning|Usage|
|---|---|---|
|=~|Matches Regular Expression|`[[ $consonants =~ [aeiou] ]]`|
|=|String Equal To|`[[ $password = "pegasus" ]]`|
|!=|String Not Equal To|`[[ $fruit != "banana" ]]`|
|!|Not|`[[ ! "apple" =~ ^b ]]`|



### If和Else

条件表达式的功能非常强大，因为我们可以使用它们来控制正在编写的Bash程序的执行方式。Bash编程中的基本构造之一是IF语句。在IF语句中编写的代码只在某个条件为真时执行，否则代码将被跳过。让我们写一个带有IF语句的小程序：

```bash
#!/usr/bin/env bash
# File: simpleif.sh

echo "Start program"

if [[ $1 -eq 4 ]]
then
  echo "You entered $1"
fi

echo "End program"
```


我们来看看运行结果，首先不带参数：

```bash
$ bash simpleif.sh
Start program
End program

```


带一个比4大的参数：

```bash
$ bash simpleif.sh 77
Start program
End program
```


带4：

```bash
$ bash simpleif.sh 4
Start program
You entered 4
End program
```


如果要处理额外的情况，我们需要加一个ELSE语句块：

```bash
#!/usr/bin/env bash
# File: simpleifelse.sh

echo "Start program"

if [[ $1 -eq 4 ]]
then
  echo "Thanks for entering $1"
else
  echo "You entered: $1, not what I was looking for."
fi

echo "End program"
```


如果是有多个if-else结构，使用ELIF语句块：

```bash
#!/usr/bin/env bash
# File: simpleelif.sh

if [[ $1 -eq 4 ]]
then
  echo "$1 is my favorite number"
elif [[ $1 -gt 3 ]]
then
  echo "$1 is a great number"
else
  echo "You entered: $1, not what I was looking for."
fi
```


**注意，IF/ELIF语句后都存在** **then** **才对。** Bash语句的写法可能与其他的语言有所不同，但使用并无差别。

## 数组

Bash中的数组是值的**有序列表** 。可以通过将列表赋值给变量名从头创建列表。列表是用圆括号`()`创建的，用**空格** 分隔列表中的每个元素。让我们列出埃及的灾殃：

```bash
plagues=(blood frogs lice flies sickness boils hail locusts darkness death)

```


要检索数组，需要使用**参数展开** ，其中包括美元符号和花括号`${}`。数组中元素的位置从**0** 开始编号。要获取该数组的第一个元素，请使用`${plagues[0]}`：

```bash
$ echo ${plagues[0]}
blood 
```


要得到瘟疫的所有元素，在方括号中使用星号（`*`）：

```bash
$ echo ${plagues[*]}
blood frogs lice flies sickness boils hail locusts darkness death 
```


你也可以通过在方括号中指定索引来改变数组中的单个元素：

```bash
$ echo ${plagues[*]}
blood frogs lice flies sickness boils hail locusts darkness death
$ plagues[4]=disease
$ echo ${plagues[*]}
blood frogs lice flies disease boils hail locusts darkness death 
```


如果只获取数组的一部分，你必须指定你想从数组开始的索引，然后是你想从数组中检索的元素数，用冒号分隔：

```bash
$ echo ${plagues[*]:5:3}
boils hail locusts 
```


你可以使用井号（`#`）找到数组的长度：

```bash
$ echo ${#plagues[*]}
10 
```


你可以使用加等于操作符（`+=`）将数组添加到数组的末尾：

```bash
$ dwarfs=(grumpy sleepy sneezy doc)
$ echo ${dwarfs[*]}
grumpy sleepy sneezy doc
$ dwarfs+=(bashful dopey happy)
$ echo ${dwarfs[*]}
grumpy sleepy sneezy doc bashful dopey happy

```


## 大括号

Bash有一个非常方便的工具，可以从称为**大括号展开** 的序列中创建字符串。大括号展开使用大括号和两个点（`{..}`）创建一个字母或数字的序列。例如，要创建一个包含0到9之间所有数字的字符串，你可以这样做：

```bash
$ echo {0..9}
0 1 2 3 4 5 6 7 8 9 
```


字母也可以：

```bash
$ echo {a..e}
a b c d e
$ echo {W..Z}
W X Y Z 
```


这种序列生成可以组合以及连用：

```bash
$ echo a{0..4}
a0 a1 a2 a3 a4
$ echo b{0..4}c
b0c b1c b2c b3c b4c
$ echo {1..3}{A..C}
1A 1B 1C 2A 2B 2C 3A 3B 3C 
```


如果你想使用变量来定义序列，你需要使用`eval`命令来创建序列：

```bash
$ start=4
$ end=9
$ echo {$start..$end}
{4..9}
$ eval echo {$start..$end}
4 5 6 7 8 9 
```


而且可以使用`,`实现序列的增补：

```bash
$ echo {{1..3},{a..c}}
1 2 3 a b c 
```


## 循环♻️

循环是Bash语言中最重要的编程结构之一。到目前为止，我们编写的所有程序都是从脚本的第一行执行到最后一行，但是循环允许基于逻辑条件或遵循序列重复代码行。

### For

我们要讨论的第一种循环是FOR循环。FOR循环遍历你指定的序列的每个元素。让我们看一个小的FOR循环示例：

```bash
#!/usr/bin/env bash
# File: forloop.sh

echo "Before Loop"

for i in {1..3}
do
    echo "i is equal to $i"
done

echo "After Loop"
```


运行该脚本：

```bash
$ bash forloop.sh
Before Loop
i is equal to 1
i is equal to 2
i is equal to 3
After Loop 
```


一旦我们了解了上述原则，我们可以尝试阅读一些其他类型的序列生成策略，看看这个例子：

```bash
#!/usr/bin/env bash
# File: manyloops.sh

echo "Explicit list:"

for picture in img001.jpg img002.jpg img451.jpg
do
    echo "picture is equal to $picture"
done

echo ""
echo "Array:"

stooges=(curly larry moe)

for stooge in ${stooges[*]}
do
    echo "Current stooge: $stooge"
done

echo ""
echo "Command substitution:"

for code in $(ls)
do
    echo "$code is a bash script"
done
```


运行：

```bash
$ bash manyloops.sh
Explicit list:
picture is equal to img001.jpg
picture is equal to img002.jpg
picture is equal to img451.jpg

Array:
Current stooge: curly
Current stooge: larry
Current stooge: moe

Command substitution:
bigmath.sh is a bash script
condexif.sh is a bash script
forloop.sh is a bash script
letsread.sh is a bash script
manyloops.sh is a bash script
math.sh is a bash script
nested.sh is a bash script
simpleelif.sh is a bash script
simpleif.sh is a bash script
simpleifelse.sh is a bash script
vars.sh is a bash script 
```


### While

现在我们已经有了几个FOR循环，让我们继续看WHILE循环。WHILE循环确实是编程结构中的“里斯花生酱杯”，它结合了部分FOR循环和IF语句。让我们看一个WHILE循环的例子，这样你就可以明白我的意思了：

```bash
#!/usr/bin/env bash
# File: whileloop.sh

count=3

while [[ $count -gt 0 ]]
do
  echo "count is equal to $count"
  let count=$count-1
done
```


上面注意`do`和`done`是一对，读者自己编写时莫要忘记了。

看看运行结果：

```bash
$ bash whileloop.sh
count is equal to 3
count is equal to 2
count is equal to 1
```


### 嵌套

像IF语句一样，FOR和WHILE语句可以嵌套：

```bash
#!/usr/bin/env bash
# File: nestedloops.sh

for number in {1..3}
do
  for letter in a b
  do
    echo "number is $number, letter is $letter"
  done
done
```


运行：

```bash
$ bash nestedloops.sh
number is 1, letter is a
number is 1, letter is b
number is 2, letter is a
number is 2, letter is b
number is 3, letter is a
number is 3, letter is b 
```


## 函数

### 写函数

函数是有名称的一小段代码。编写函数允许我们在程序中多次重用相同的代码。函数的语法如下：

```bash
function [name of function] {
  # code here
}
```


是不是很简单？让我们来写一个`hello.sh`：

```bash
#!/usr/bin/env bash
# File: hello.sh

function hello {
  echo "Hello"
}

hello # 这里是调用函数执行
hello
hello
```


函数的整个结构，包括`function`关键字、函数名和写在方括号内的函数代码，作为函数定义。函数定义将函数内的代码分配给函数名（在本例中是`hello`）。函数定义后，就可以像其他任何命令一样使用它。使用三次`hello`命令应该相当于使用三次`echo "Hello"`。让我们运行这个脚本来找出答案：

```bash
$ bash hello.sh
Hello
Hello
Hello 
```


函数与整个bash脚本共享许多行为，包括它们如何处理参数。通常的bash脚本参数如`$1`、`$2`和`$@`都在函数中工作，这允许你指定函数参数。让我们创建一个稍微修改过的hello.sh版本，我们将其命名为`ntmy.sh`：

```bash
#!/usr/bin/env bash
# File: ntmy.sh

function ntmy {
  echo "Nice to meet you $1"
}
```


在上面的文件中，请注意，在定义了ntmy函数之后，我们没有使用它。这是因为我们将开始使用我们定义为命令行程序的函数。到目前为止，本章我们一直在使用`bash[脚本名]`的语法来执行脚本的内容。现在我们将开始使用`source`命令，**它允许我们将bash脚本中的函数定义作为命令行命令使用** 。让我们对这个文件使用`source`，这样我们就可以使用`ntmy`命令：

```bash
$ source ntmy.sh
$ ntmy Jeff
Nice to meet you Jeff
$ ntmy Philip
Nice to meet you Philip
$ ntmy Jenny
Nice to meet you Jenny 
```


就像这样，你已经创建了自己的命令！但一旦关闭当前shell，你将失去对ntmy命令的访问权，但在下一节中，我们将讨论如何设置你自己的命令，以便始终能够访问它们。

我们写一个更复杂的函数。假设我们想要从命令行中对一个数字序列进行相加，但是我们无法知道序列中有多少数字。我们需要什么东西来写这个函数？**首先，我们需要一种方法来捕获可变长度的参数列表，其次，我们需要一种方法来遍历该列表以便将每个元素相加，我们还需要一种方法来存储序列的累积和** 。这三个要求可以通过使用`$@`变量、一个FOR循环和可以存储和的变量来满足。在编写程序之前，将一个更大的目标分解成一系列独立的模块是很重要的，这样我们可以更容易地确定需要哪些特性和工具。让我们在一个名为`addseq.sh`的文件中编写这个程序。

```bash
#!/usr/bin/env bash
# File: addseq.sh

function addseq {
  sum=0

  for element in $@
  do
    let sum=sum+$element
  done

  echo $sum
}
```


运行：

```bash
$ source addseq.sh
$ addseq 12 90 3
105
$ addseq 0 1 1 2 3 5 8 13
33
$ addseq
0
$ addseq 4 6 6 6 4
26 
```


### 从函数中获取值

函数主要用于两个目的：**计算值和副作用** 。在前一节的`addseq`命令中，我们为该命令提供了一个数字序列，然后该命令为我们提供了该序列的和，这是我们感兴趣的值。在本例中，我们可以看到`addseq`基于一些输入值计算了一个值。许多其他命令，例如`pwd`，返回一个值而不影响我们计算机上的文件状态。然而，像`mv`或`cp`这样的功能可以移动和复制我们计算机上的文件。**每当一个函数在我们的计算机上创建或更改文件时，就会产生一个副作用** 。如果成功，这些命令不会打印任何值。

我们经常会编写函数来计算某个值，理解如何将函数的结果存储在变量中以便以后使用是很重要的。让我们源代码`addseq.sh`并再次运行它：

```bash
$ source addseq.sh
$ addseq 3 0 0 7
10
```


如果我们回头看`addseq.sh`的代码，可以看到我们在函数`sum`中创建了一个变量。当您在函数中创建变量时，这些变量成为全局可访问的，**这意味着即使在程序结束后，该变量仍然在您的shell中保留其值** 。我们可以通过返回`sum`的值来很容易地验证这一点：

```bash
$ echo $sum
10 
```


这是我们可以用来检索函数计算的值的一个策略示例。不幸的是，这种方法存在问题，因为它改变了我们可能在shell中使用的变量的值。例如，如果我们在一个名为`sum`的变量中存储一些其他重要的值，我们会通过运行`addseq`意外地销毁该值。为了避免这个问题，**在函数中赋值变量时使用local关键字是很重要的** 。`local`关键字确保函数外部的变量不会被函数覆盖。让我们创建一个名为`addseq2`的`addseq`新版本，它在赋值变量时使用`local`。

```bash
#!/usr/bin/env bash
# File: addseq2.sh

function addseq2 {
  local sum=0

  for element in $@
  do
    let sum=sum+$element
  done

  echo $sum
}
```


## 编写程序

### Unix哲学

也许在我们开始讨论Unix工具时，你已经注意到了一些设计模式，现在我们将明确地讨论它们。Unix工具是按照一组指导方针设计的，Ken Thompson的思想最好地总结了这些指导方针，**即每个Unix程序应该做好一件事** 。当编写函数和程序时，遵循这个规则可以实现以下几个目标：

- 限制一个程序只做一件事会减少程序的长度，如果程序包含错误或需要修改，那么越短的程序就越容易修复。

- 编写简短的程序还可以帮助代码的用户理解在他们需要阅读您的代码时代码中发生了什么。与阅读小说相比，阅读诗歌会诱发不同的认知负荷。

- 那些不阅读程序源代码的人（大多数用户不会——他们不应该这样做）将能够更容易地理解程序的输入、输出和副作用。

- 使用小程序来编写一个新程序会增加新程序也很小的可能性。可组合性是将小程序串在一起创建新程序的概念。

Unix中可组合性的概念最好通过使用用于创建程序管道的管道操作符（`|`）来说明。当你考虑你的程序将有什么输入以及你的程序将输出到控制台时，你应该考虑你的程序是否可能在管道中使用，你应该相应地组织你的程序。

### 让程序可执行

首先让我们看下我们的工作目录：

```bash
$ ls -l | head -n 3
-rw-rw-r-- 1 sean sean 138 Jun 26 12:51 addseq.sh
-rw-rw-r-- 1 sean sean 146 Jun 26 14:45 addseq2.sh
-rw-rw-r-- 1 sean sean 140 Jan 29 10:06 bigmath.sh 
```


该表的左列包含一系列单独的字符和破折号。第一个连字符（`-`）表示这个列表中的每个条目都是文件。如果其中任何一个是目录，那么就会用`d`代替连字符。除去第一个连字符，我们有以下字符串：`rw-rw-r-—`。该字符串反映了为该文件设置的权限。我们可以授予三种权限：读取文件`r`、写入或编辑文件`w`或作为程序执行文件`x`的能力。这三种权限可以被授予三种不同的访问级别，它们对应于权限字符串中的三组`rwx`中的每一组：文件的所有者、文件所属的组以及除所有者和组成员之外的所有人。由于你创建了该文件，因此你是该文件的所有者，你可以使用chmod命令为你所拥有的文件设置权限。

`chmod`命令有两个参数。第一个参数是一个字符串，它指定了我们将如何改变一个文件的权限，第二个参数是文件的路径。第一个论证必须以一种非常具体的方式组成。首先，我们可以指定要为哪组用户更改权限：

|Character|Meaning|
|---|---|
|`u`|文件所有者|
|`g`|文件所属群组|
|`o`|其他人|
|`a`|所有人|



然后我们需要指定是添加、删除还是设置权限：

|Character|Meaning|
|---|---|
|`+`|添加权限|
|`-`|移除权限|
|`=`|设定权限|



最后，我们指定要更改的权限：

|Character|Meaning|
|---|---|
|`r`|读文件|
|`w`|写或编辑文件|
|`x`|执行文件|



让我们举个小例子：

```bash
$ echo 'echo "a small program"' > short # 创建一个脚本文件
$ ls -l short 
-rw-r--r--  1 sean  staff  23 Jun 28 09:47 short
$ # 添加可执行权限
$ chmod u+x short
$ ls -l short
-rwxr--r--  1 sean  staff  23 Jun 28 09:47 short 
```


现在我们就可以直接使用下面的方式执行它了：

```bash
$ ./short
a small program 
```


看起来很有效！不过，我们应该在这个程序中添加一个小细节。即使我们已经让文件可执行了，如果我们把程序给别人，他们可能使用的是不知道如何执行程序的shell。我们需要通过在程序的开头添加称为`shebang`的特殊文本行来指示程序应该如何运行。`shebang`总是以`#!`后面是将执行文件中的代码的程序的路径。表示我们想使用Bash的shebang是`#!/usr/bin/env bash`，我们已经把它添加到脚本的开头有一段时间了！让我们重写这个程序以包含Bash shebang，然后运行这个程序。

```bash
$ echo '#!/usr/bin/env bash' > short
$ echo 'echo "a small program"' >> short
```


现在我们的Bash脚本已经准备好了！

### 环境变量

我们距离能够将脚本和函数作为shell命令使用只有一步之遥，但是首先我们需要了解环境变量。环境变量是Bash创建的变量，用于存储关于当前计算环境的数据。环境变量名全部使用大写字母。让我们看看这些变量的值。`HOME`变量包含到主目录的路径，`PWD`变量包含到当前目录的路径。

```bash
$ echo $HOME
/Users/sean
$ echo $PWD
/Users/sean/Code 
```


如果我们希望某个函数始终作为命令可用，那么我们需要更改`PATH`变量。让我们先看一下这个变量。

```bash
$ echo $PATH
/usr/local/bin:/usr/bin:/bin:/usr/local/git/bin 
```


`PATH`变量包含计算机上以冒号分隔的一系列路径。当shell启动时，它会在这些路径中搜索可执行文件，然后让这些可执行命令在我们的shell中可用。使我们的脚本可用的一种方法是向`PATH`添加一个目录。可执行目录中的Bash脚本可以作为命令使用。每次启动shell时，我们都需要修改`PATH`，因此可以修改`~/.bash_profile`（Linux一般是`~/.bashrc`）使可执行脚本的目录始终位于`PATH`中。要修改环境变量，我们需要使用`export`关键字。

首先，让我们在Code目录中创建一个名为Commands的新目录，在那里我们可以保存可执行脚本。然后我们将在`~/.bash_profile`中添加一行，以便将Commands添加到`PATH`。

```bash
$ mkdir Commands
$ nano ~/.bash_profile
```


内容为：

```bash
alias docs='cd ~/Documents'
alias edbp='nano ~/.bash_profile'

export PATH=~/Code/Commands:$PATH
```


> 不建议读者进行上述操作，如果要进行，请首先备份`~/.bash_profile`文件。


这样我们就可以在任意目录执行`short`命令了：

```bash
$ source ~/.bash_profile
$ short
a small program 
```


除了使个别脚本可执行之外，我们还可以在`~/.bash_profile`添加一个`source`命令。这样我们就可以在命令行上使用Bash函数。让我们重新编辑该文件：

```bash
alias docs='cd ~/Documents'
alias edbp='nano ~/.bash_profile'

export PATH=~/Code/Commands:$PATH
source ~/Code/addseq2.sh
```


保存文件，通过`source`更新配置文件，然后就可以将脚本中的函数当作命令使用了：

```bash
$ source ~/.bash_profile
$ addseq2 9 8 7
24 
```




