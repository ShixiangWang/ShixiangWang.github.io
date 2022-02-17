---
title: 4步为Jupyter Notebook添加Conda环境
author: 王诗翔
date: '2022-02-17'
slug: 4-jupyter-notebook-conda
categories:
  - Blog
tags:
  - Jupyter
  - conda
description: 译文
---

### 第一步，创建conda环境

```
conda create --name firstEnv
```

你会看到类似下面的内容：

```
(base) PS C:\Users\Administrator> conda create --name firstEnv
Collecting package metadata (current_repodata.json): done
Solving environment: done

## Package Plan ##

  environment location: C:\miniconda\envs\firstEnv



Proceed ([y]/n)? y

Preparing transaction: done
Verifying transaction: done
Executing transaction: done
#
# To activate this environment, use
#
#     $ conda activate firstEnv
#
# To deactivate an active environment, use
#
#     $ conda deactivate
```

### 第二步，激活环境并安装你需要的软件包

```
conda activate firstEnv
conda install -c conda-forge datar
```

这里以datar举例。

```
(firstEnv) PS C:\Users\Administrator> conda install -c conda-forge datar
Collecting package metadata (current_repodata.json): done
Solving environment: done

## Package Plan ##

  environment location: C:\miniconda\envs\firstEnv

  added / updated specs:
    - datar

...
```

### 第三步，安装和配置ipykernel 

```
conda install -c conda-forge ipykernel
```

然后运行：

```
python -m ipykernel install --user --name=firstEnv
```

### 第四步，检查安装

打开你的Jupyter Notebook，查看界面是否已经可以选择firstEnv作为新的环境。

![](https://miro.medium.com/max/1400/1*QpkEdJSnvDDhIWRIxGMr5A.png)


### 收尾

如果不想在Jupyter Notebook中保留该环境，可以移除：

```
(base) PS C:\Users\Administrator> jupyter kernelspec remove firstenv
Kernel specs to remove:
  firstenv              C:\Users\Administrator\AppData\Roaming\jupyter\kernels\firstenv
```

参考：

> https://medium.com/@nrk25693/how-to-add-your-conda-environment-to-your-jupyter-notebook-in-just-4-steps-abeab8b8d084





