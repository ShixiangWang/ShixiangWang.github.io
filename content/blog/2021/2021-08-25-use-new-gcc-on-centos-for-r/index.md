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

如果没有root权限，可以通过conda来安装新版本的gcc,c++等：

```bash
conda install gcc_linux-64
conda install gxx_linux-64
```

> 加上conda-forge通道也可以。

另外可以一键安装常用的编译器：

```bash
conda install -c conda-forge compilers
```

安装完成后记得添加环境变量：

```bash
export CC=/path/to/anaconda/bin/x86_64-conda_cos6-linux-gnu-gcc
export CXX=/path/to/anaconda/bin/x86_64-conda_cos6-linux-gnu-g++
```

如果是R包编译，修改前面提到的`Makevars`文件即可。

如果使用的是miniconda，这个文件的内容可能就是这样的了：

```R
CXX11=~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++11 -fPIC
CXX14=~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC
```

这样就可以愉快地安装包了：

```R
> install.packages("xgboost")
试开URL’https://mirrors.e-ducation.cn/CRAN/src/contrib/xgboost_1.4.1.1.tar.gz'
Content type 'application/x-gzip' length 988320 bytes (965 KB)
==================================================
downloaded 965 KB

* installing *source* package ‘xgboost’ ...
** 成功将‘xgboost’程序包解包并MD5和检查
** using staged installation
checking for x86_64-conda-linux-gnu-gcc... ~/miniconda3/bin/x86_64-conda-linux-gnu-cc
checking whether the C compiler works... yes
checking for C compiler default output file name... a.out
checking for suffix of executables... 
checking whether we are cross compiling... no
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether ~/miniconda3/bin/x86_64-conda-linux-gnu-cc accepts -g... yes
checking for ~/miniconda3/bin/x86_64-conda-linux-gnu-cc option to accept ISO C89... none needed
checking Backtrace lib... 
checking for backtrace in -lexecinfo... no
checking endian... 
configure: creating ./config.status
config.status: creating src/Makevars
** libs
Makevars:17: -DXGBOOST_STRICT_R_MODE=1
Makevars:17: -DDMLC_LOG_BEFORE_THROW=0
Makevars:17: -DDMLC_ENABLE_STD_THREAD=1
Makevars:17: -DDMLC_DISABLE_STDIN=1
Makevars:17: -DDMLC_LOG_CUSTOMIZE=1
Makevars:17: -DXGBOOST_CUSTOMIZE_LOGGER=1
Makevars:17: -DRABIT_CUSTOMIZE_MSG_
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c xgboost_R.cc -o xgboost_R.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c xgboost_custom.cc -o xgboost_custom.o
gcc -std=gnu99 -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include   -fpic  -g -O2  -c xgboost_assert.c -o xgboost_assert.o
gcc -std=gnu99 -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include   -fpic  -g -O2  -c init.c -o init.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c amalgamation/xgboost-all0.cc -o amalgamation/xgboost-all0.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c amalgamation/dmlc-minimum0.cc -o amalgamation/dmlc-minimum0.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c rabit/src/engine.cc -o rabit/src/engine.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c rabit/src/c_api.cc -o rabit/src/c_api.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC  -I"/public/apps/R-4.0/R-4.0.3/include" -DNDEBUG -I./include -I./dmlc-core/include -I./rabit/include -I. -DXGBOOST_STRICT_R_MODE=1 -DDMLC_LOG_BEFORE_THROW=0 -DDMLC_ENABLE_STD_THREAD=1 -DDMLC_DISABLE_STDIN=1 -DDMLC_LOG_CUSTOMIZE=1 -DXGBOOST_CUSTOMIZE_LOGGER=1 -DRABIT_CUSTOMIZE_MSG_  -I/public/apps/R-4.0/zlib/include -I/public/apps/R-4.0/bzip2/include -I/public/apps/R-4.0/xz/include -I/public/apps/R-4.0/curl/include -I/public/apps/R-4.0/pcre/include  -fopenmp -DDMLC_CMAKE_LITTLE_ENDIAN=1 -pthread    -c rabit/src/allreduce_base.cc -o rabit/src/allreduce_base.o
~/miniconda3/bin/x86_64-conda-linux-gnu-g++ -std=c++14 -fPIC -shared -L/public/apps/R-4.0/R-4.0.3/lib -L/public/apps/R-4.0/zlib/lib ....
** R
** data
** demo
** inst
** byte-compile and prepare package for lazy loading
** help
*** installing help indices
** building package indices
** installing vignettes
** testing if installed package can be loaded from temporary location
** checking absolute paths in shared objects and dynamic libraries
** testing if installed package can be loaded from final location
** testing if installed package keeps a record of temporary installation path
* DONE (xgboost)

下载的程序包在
	‘/tmp/RtmpjK0jDw/downloaded_packages’里
```