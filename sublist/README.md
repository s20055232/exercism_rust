# Sublist

Welcome to Sublist on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## 解題邏輯

運用基本的Rust枚舉觀念就可以解答，可以像我最一開始的做法一樣使用`if..else`，也可以使用`match`。

我自己寫得答案跟這個[社群答案](https://exercism.org/tracks/rust/exercises/sublist/solutions/alireza4050)水平差太多了，其中對於作者`match`的認識跟使用方法非常有意思，推薦各位去看他的解答。

## Instructions

Given two lists determine if the first list is contained within the second
list, if the second list is contained within the first list, if both lists are
contained within each other or if none of these are true.

Specifically, a list A is a sublist of list B if by dropping 0 or more elements
from the front of B and 0 or more elements from the back of B you get a list
that's completely equal to A.

Examples:

* A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
* A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
* A = [3, 4], B = [1, 2, 3, 4, 5], A is a sublist of B
* A = [1, 2, 3], B = [1, 2, 3], A is equal to B
* A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
* A = [1, 2, 4], B = [1, 2, 3, 4, 5], A is not a superlist of, sublist of or equal to B

## Source

### Created by

* @EduardoBautista

### Contributed to by

* @ashleygwilliams
* @coriolinus
* @cwhakes
* @eddyp
* @EduardoBautista
* @efx
* @ErikSchierboom
* @IanWhitney
* @kytrinyx
* @lutostag
* @mkantor
* @nfiles
* @petertseng
* @rofrol
* @stringparser
* @xakon
* @ZapAnton
