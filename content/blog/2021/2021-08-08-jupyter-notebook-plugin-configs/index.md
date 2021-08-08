---
title: Jupyter Notebook 插件配置
author: 王诗翔
date: '2021-08-08'
slug: jupyter-notebook-plugin-configs
categories:
  - Blog
tags:
  - Jupyter
  - 配置
---

安装命令：

```bash
pip3 install -U jupyter_contrib_nbextensions
jupyter contrib nbextension install --user
pip3 install -U jupyter_nbextensions_configurator
jupyter nbextensions_configurator enable --user

pip3 install -U qgrid # 动态操作 DataFrame
jupyter nbextension enable --py --sys-prefix qgrid 
jupyter nbextension enable --py --sys-prefix widgetsnbextension 
```

如果要修改主题：

```bash
pip3 install -U jupyterthemes
```

网址：https://github.com/dunovank/jupyter-themes

**激活的插件**：

![](https://gitee.com/ShixiangWang/ImageCollection/raw/master/png/20210809000128.png)
