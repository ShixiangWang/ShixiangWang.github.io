---
title: CentOS/Redhat R包使用最新的gcc编译
author: 王诗翔
date: '2021-08-25'
slug: use-new-gcc-on-centos-for-r
categories:
  - Blog
tags:
  - 配置
  - Linux
  - Unix
description: R包在Linux下编译gcc版本太低怎么办？
---

R包在Linux下编译不通过，原因是gcc版本太低怎么办？

一些有C++代码的R包可能会用到一些新的C++特性，需要C++11或者C++14。这个问题通常在CentOS/红帽系统上出现，因为系统稳定的要求，这个系列的系统它的C++版本很低。
但请读者前往注意了别自己编译新版本的gcc，然后替换掉系统的。这种操作我试过几次，系统基本上就崩掉了。

正确的解决方式是安装独立的gcc，通过环境变量引用和使用它。

在Root用户下操作：

```bash
yum install centos-release-scl
yum install devtoolset-9
```

然后在你使用R的用户下操作：

```bash
# If you use your non-root account to install packages, 
# change /root to /home/your_id in the following command
mkdir -p /root/.R
vi /root/.R/Makevars 
```

将下面的内容写入打开的文件，然后保存：

```bash
CXX11=/opt/rh/devtoolset-9/root/usr/bin/g++ -std=c++11 -fPIC
CXX14=/opt/rh/devtoolset-9/root/usr/bin/g++ -std=c++14 -fPIC
```

