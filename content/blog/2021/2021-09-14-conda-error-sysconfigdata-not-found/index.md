---
title: 解决conda报错：Module _sysconfigdata_x86_64_conda_linux_gnu not found
author: 王诗翔
date: '2021-09-14'
slug: conda-error-sysconfigdata-not-found
categories:
  - Blog
tags:
  - conda
  - Python
description: 很奇怪的错误
---

最新可能是受conda update conda的影响，发现使用conda涉及Python的操作时一直出现问题，
报错：

```bash
ModuleNotFoundError: No module named '_sysconfigdata_x86_64_conda_linux_gnu'
```

例如：

```bash
$ pip list
Traceback (most recent call last):
  File "~/miniconda3/bin/pip", line 7, in <module>
    from pip._internal.cli.main import main
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/cli/main.py", line 9, in <module>
    from pip._internal.cli.autocompletion import autocomplete
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/cli/autocompletion.py", line 10, in <module>
    from pip._internal.cli.main_parser import create_main_parser
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/cli/main_parser.py", line 8, in <module>
    from pip._internal.cli import cmdoptions
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/cli/cmdoptions.py", line 23, in <module>
    from pip._internal.cli.parser import ConfigOptionParser
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/cli/parser.py", line 12, in <module>
    from pip._internal.configuration import Configuration, ConfigurationError
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/configuration.py", line 27, in <module>
    from pip._internal.utils.misc import ensure_dir, enum
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/utils/misc.py", line 42, in <module>
    from pip._internal.locations import get_major_minor_version, site_packages, user_site
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/locations/__init__.py", line 13, in <module>
    from . import _distutils, _sysconfig
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/locations/_distutils.py", line 19, in <module>
    from .base import get_major_minor_version
  File "~/miniconda3/lib/python3.9/site-packages/pip/_internal/locations/base.py", line 15, in <module>
    site_packages: typing.Optional[str] = sysconfig.get_path("purelib")
  File "~/miniconda3/lib/python3.9/sysconfig.py", line 519, in get_path
    return get_paths(scheme, vars, expand)[name]
  File "~/miniconda3/lib/python3.9/sysconfig.py", line 509, in get_paths
    return _expand_vars(scheme, vars)
  File "~/miniconda3/lib/python3.9/sysconfig.py", line 170, in _expand_vars
    _extend_dict(vars, get_config_vars())
  File "~/miniconda3/lib/python3.9/sysconfig.py", line 559, in get_config_vars
    _init_posix(_CONFIG_VARS)
  File "~/miniconda3/lib/python3.9/sysconfig.py", line 428, in _init_posix
    _temp = __import__(name, globals(), locals(), ['build_time_vars'], 0)
ModuleNotFoundError: No module named '_sysconfigdata_x86_64_conda_linux_gnu'
```

搜索了一圈发现用以下的办法可以解决。

首先查一下哪些路径可能涉及到该模块：

```bash
$ find ~ -name _sysconfigdata_x86_64*
```

如果系统中存在多个不同的Python版本，可能有多个结果，我们需要查找哪个有`_sysconfigdata_x86_64_conda_cos6_linux_gnu.py`但是没有
对应的`_sysconfigdata_x86_64_conda_linux_gnu.py`。

一种更简便地策略是先查看自己使用的是哪个Python，路径来自哪里。比如说我们这里是conda报错，
那肯定对应着相应的环境，如果是base环境，我们可以直接进入对应的Python库目录下：

```bash
cd ~/miniconda3/lib/python3.9/
```

然后进行下面的拷贝操作：

```bash
cp _sysconfigdata_x86_64_conda_cos6_linux_gnu.py _sysconfigdata_x86_64_conda_linux_gnu.py
```

报错就没有了。

回顾以下不难猜测这两个模块可能就是冗余备份，但一些程序只认其中一个，所以缺失了不行。

另外值得思考的是：

- 为什么该文件出现了丢失？
- 为什么不统一使用指定的模块？

>参考：<https://stackoverflow.com/questions/68261254/conda-error-sysconfigdata-x86-64-conda-linux-gnu>