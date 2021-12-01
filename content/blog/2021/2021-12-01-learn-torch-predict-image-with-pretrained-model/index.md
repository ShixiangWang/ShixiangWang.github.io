---
title: torch入门：使用预训练模型预测图像分类
author: 王诗翔
date: '2021-12-01'
slug: learn-torch-predict-image-with-pretrained-model
categories:
  - Blog
tags:
  - torch
  - Python
  - 深度学习
description: Py!torch!
---

代码来源图书 [deep-learning-with-pytorch](https://www.manning.com/books/deep-learning-with-pytorch)。

```python
from torchvision import models
import torch

dir(models)

resnet = models.resnet101(pretrained=True)
from torchvision import transforms
preprocess = transforms.Compose([
        transforms.Resize(256),
        transforms.CenterCrop(224),
        transforms.ToTensor(),
        transforms.Normalize(
            mean=[0.485, 0.456, 0.406],
            std=[0.229, 0.224, 0.225]
        )])

from PIL import Image
img = Image.open("../data/p1ch2/bobby.jpg")
img_t = preprocess(img)
batch_t = torch.unsqueeze(img_t, 0)
resnet.eval()
out = resnet(batch_t)
out

with open('../data/p1ch2/imagenet_classes.txt') as f:
    labels = [line.strip() for line in f.readlines()]

_, index = torch.max(out, 1)

percentage = torch.nn.functional.softmax(out, dim=1)[0] * 100 # 转换为概率
labels[index[0]], percentage[index[0]].item()

_, indices = torch.sort(out, descending=True)
[(labels[idx], percentage[idx].item()) for idx in indices[0][:5]]
```

国内无法下载预训练模型的办法：

- 从镜像 <https://coggle.club/note/dl/pretrained-models> 进行下载。

```
!cd /home/wsx/.cache/torch/hub/checkpoints && wget -c http://mirror.coggle.club/models/torchvision/resnet101-5d3b4d8f.pth
```

- 加载模型文件

```python
pthfile = '/home/wsx/.cache/torch/hub/checkpoints/resnet101-5d3b4d8f.pth'
resnet = models.resnet101(pretrained=False)

resnet.load_state_dict(torch.load(pthfile))
```

