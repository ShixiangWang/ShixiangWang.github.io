---
title: yum升级git版本
author: 王诗翔
date: '2021-11-29'
slug: yum-upgrade-git
categories:
  - Blog
tags:
  - CentOS
  - Linux
  - git
description: Yum yum
rmd_source: ''
keywords: git
---

安装新的RPM仓库：

```sh
sudo yum -y install https://packages.endpoint.com/rhel/7/os/x86_64/endpoint-repo-1.7-1.x86_64.rpm
```

更新Git：

```sh
sudo yum upgrade git
```

> 参考：<https://www.seozen.top/centos-update-upgrade-git.html>
