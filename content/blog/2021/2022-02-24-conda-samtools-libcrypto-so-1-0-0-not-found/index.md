---
title: conda安装samtools包使用报错：libcrypto.so.1.0.0 not found
author: 王诗翔
date: '2022-02-24'
slug: conda-samtools-libcrypto-so-1-0-0-not-found
categories:
  - Blog
tags:
  - conda
description: 又是conda出问题，斗智斗勇。
---

通过conda的bioconda Channel安装samtools后，使用时发现报错。

```sh
(biosoft) wsx 11:55:46 ~
$ conda install -c bioconda samtools
Collecting package metadata (current_repodata.json): done
Solving environment: done

## Package Plan ##

  environment location: /home/wsx/miniconda3/envs/biosoft

  added / updated specs:
    - samtools


The following packages will be downloaded:

    package                    |            build
    ---------------------------|-----------------
    bzip2-1.0.8                |       h7b6447c_0          78 KB  defaults
    c-ares-1.18.1              |       h7f8727e_0         114 KB  defaults
    curl-7.80.0                |       h7f8727e_0          95 KB  defaults
    krb5-1.19.2                |       hac12032_0         1.2 MB  defaults
    libcurl-7.80.0             |       h0b77cf5_0         339 KB  defaults
    libedit-3.1.20210910       |       h7f8727e_0         166 KB  defaults
    libev-4.33                 |       h7f8727e_1         111 KB  defaults
    libgcc-7.2.0               |       h69d50b8_2         269 KB  defaults
    libnghttp2-1.46.0          |       hce63b2e_0         680 KB  defaults
    libssh2-1.9.0              |       h1ba5d50_1         269 KB  defaults
    samtools-1.7               |                1         1.0 MB  bioconda
    ------------------------------------------------------------
                                           Total:         4.3 MB

The following NEW packages will be INSTALLED:

  bzip2              anaconda/pkgs/main/linux-64::bzip2-1.0.8-h7b6447c_0
  c-ares             anaconda/pkgs/main/linux-64::c-ares-1.18.1-h7f8727e_0
  curl               anaconda/pkgs/main/linux-64::curl-7.80.0-h7f8727e_0
  krb5               anaconda/pkgs/main/linux-64::krb5-1.19.2-hac12032_0
  libcurl            anaconda/pkgs/main/linux-64::libcurl-7.80.0-h0b77cf5_0
  libedit            anaconda/pkgs/main/linux-64::libedit-3.1.20210910-h7f8727e_0
  libev              anaconda/pkgs/main/linux-64::libev-4.33-h7f8727e_1
  libgcc             anaconda/pkgs/main/linux-64::libgcc-7.2.0-h69d50b8_2
  libnghttp2         anaconda/pkgs/main/linux-64::libnghttp2-1.46.0-hce63b2e_0
  libssh2            anaconda/pkgs/main/linux-64::libssh2-1.9.0-h1ba5d50_1
  samtools           bioconda/linux-64::samtools-1.7-1


Proceed ([y]/n)? y


Downloading and Extracting Packages
libev-4.33           | 111 KB    | ############################################################################################################### | 100% 
libgcc-7.2.0         | 269 KB    | ############################################################################################################### | 100% 
libedit-3.1.20210910 | 166 KB    | ############################################################################################################### | 100% 
libnghttp2-1.46.0    | 680 KB    | ############################################################################################################### | 100% 
libcurl-7.80.0       | 339 KB    | ############################################################################################################### | 100% 
curl-7.80.0          | 95 KB     | ############################################################################################################### | 100% 
samtools-1.7         | 1.0 MB    | ############################################################################################################### | 100% 
bzip2-1.0.8          | 78 KB     | ############################################################################################################### | 100% 
libssh2-1.9.0        | 269 KB    | ############################################################################################################### | 100% 
krb5-1.19.2          | 1.2 MB    | ############################################################################################################### | 100% 
c-ares-1.18.1        | 114 KB    | ############################################################################################################### | 100% 
Preparing transaction: done
Verifying transaction: done
Executing transaction: done
(biosoft) wsx 11:56:13 ~
$ samtools
samtools: error while loading shared libraries: libcrypto.so.1.0.0: cannot open shared object file: No such file or directory
```

又是xx.so找不到的conda常见问题，但每一种解决办法都不同。

参考 https://github.com/merenlab/anvio/issues/1479 和 https://github.com/bioconda/bioconda-recipes/issues/13958。

使用下面命令可以解决：

```sh
conda install -c bioconda samtools=1.9 --force-reinstall
```