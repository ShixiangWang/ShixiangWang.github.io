---
title: "LeetCode：0001-两数之和"
author: 王诗翔
date: '2021-08-05'
slug: leetcode-0001-two-sum
categories:
  - LeetCode
tags:
  - R
  - golang
  - algo
keywords: leetcode
editor_options:
  chunk_output_type: console
---

**难度：简单。** 

参考：

- [https://leetcode-cn.com/problems/two-sum](https://leetcode-cn.com/problems/two-sum)

- [https://books.halfrost.com/leetcode/ChapterFour/0001~0099/0001.Two-Sum/](https://books.halfrost.com/leetcode/ChapterFour/0001~0099/0001.Two-Sum/)

代码仓库：[https://github.com/ShixiangWang/LeetCode](https://github.com/ShixiangWang/LeetCode)

## 问题

给定一个整数数组`nums`和一个整数目标值`target`，请你在该数组中找出 和为目标值`target` 的那两个整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。

示例：

```
输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
```



## 题解

顺序扫描数组，对每一个元素，在`map`中找能组合给定值的另一半数字，如果找到了，直接返回`2`个数字的下标即可。如果找不到，就把这个数字存入`map`中，等待扫到“另一半”数字的时候，再取出来返回结果。

这种解法将数据扫描一遍必然得到结果，所以时间复杂度是`O(n)`。


### Go

```go
package main

import "fmt"

func twoSum(nums []int, target int) []int {
  m := make(map[int]int)
  for i := 0; i < len(nums); i++ {
    another := target - nums[i]
    if _, ok := m[another]; ok {
      return []int{m[another], i}
    }
    m[nums[i]] = i
  }
  return nil
}

func main() {
  fmt.Println(twoSum([]int{2, 7, 11, 15}, 9))
  fmt.Println(twoSum([]int{3, 2, 4}, 6))
}

```


运行结果：

```Bash
[Running] go run "/Users/wsx/Documents/GitHub/LeetCode/LeetCode/leetcode/0001-two-sum/main.go"
[0 1]
[1 2]
```


### R

依据题解和Go的实现思路，不难写出R的对应程序。不过考虑到语言之间的差异，值得注意的有几点：

1. R本身不自带纯粹的字典实现。我们使用`c()`构造命名向量，但需要注意该向量是支持整数索引的，所以当我们使用输入序列作为键时，都必须使用相应的字符串形式。

2. R是以1作为索引起始，为了保持结果一致，最后返回值统一减去1。

```R
twoSum <- function(nums, target) {
  # stopifnot(length(target) == 1, is.numeric(nums), is.numeric(target))
  m <- c()
  for (i in seq_along(nums)) {
    another <- target - nums[i]
    if (length(m) > 0 && !is.na(m[as.character(another)])) {
      return(c(as.integer(m[as.character(another)]), i) - 1L) # R 是1开始索引的编程语言，这里做一个-1处理
    }
    m[as.character(nums[i])] = i
  }
  return(NULL)
}

cat(twoSum(c(2, 7, 11, 15), 9))
cat("\n")
cat(twoSum(c(3, 2, 4), 6))

```


运行结果：

```Bash
[Running] Rscript "/Users/wsx/Documents/GitHub/LeetCode/LeetCode/leetcode/0001-two-sum/main.R"
Using library: /Users/wsx/Library/R
0 1
1 2
```
