---
title: Shell：工具工作技巧备忘
author: 王诗翔
date: '2021-08-18'
slug: unix-working-tricks
categories:
  - Blog
tags:
  - 自动化
  - Unix
  - Bash
---

## 远程传输和备份文件（夹）

使用`scp`无法续传，使用`rsync`更好。

```bash
rsync -avLr --progress huaxi:/remote_dir/ ./local_dir
```

