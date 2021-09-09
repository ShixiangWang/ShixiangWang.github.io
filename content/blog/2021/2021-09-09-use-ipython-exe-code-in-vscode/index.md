---
title: 在VSCode中使用ipython执行代码
author: 王诗翔
date: '2021-09-09'
slug: use-ipython-exe-code-in-vscode
categories:
  - Blog
tags:
  - Python
  - VSCode
description: 不同的社区思想确实有点不一样呢
---

参考Stack Overflow[这篇回答](https://stackoverflow.com/questions/52310689/use-ipython-repl-in-vs-code)进行配置，将下面的内容加入`settings.json`.

```json
"python.terminal.launchArgs": [
    "-m",
    "IPython",
    "--no-autoindent",
],
```

Python的数据科学社区都将编辑的重心放到了Jupyter notebook上，忽视了对`.py`文本本身的支持。
这个事情不知道是好是坏，但对于我这个R使用比较严重的人来说使用起来是不舒服的。
Jupyter的记录格式太笨重了，如果只是单独完成分析任务，代码加注释完全够用了。
R里面有RMarkdown格式，相比更加轻松。更重要的是，我在GitHub上2年前提的需求说
VSCode中Python文件不支持按块执行，现在也没有实现。只能找到上面的办法缓解这种不适。

我之前的帖子在：<https://github.com/microsoft/vscode-python/issues/8851>

我在issue中也看到了一些类似的提问和讨论，但皆不了了之。

其实还有一个痛点我没说，在执行完一句代码后，VSCode插件不会自动向下移动光标。

整个源代码编辑交互式分析的体验Python还没有编辑器能比过RStudio。
