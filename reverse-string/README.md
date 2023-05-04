# Reverse String

Welcome to Reverse String on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## 解題邏輯

這題有趣的點是，不僅僅是簡單的將字串反轉而已，如果僅僅是簡單的反轉，我們可以先將字串用`.chars()`轉成Unicode Scalar Value的迭代器（其中每個元素都是`Char`這個資料型態），然後再呼叫反轉，但當遇到[字位](https://zh.wikipedia.org/zh-tw/%E5%AD%97%E4%BD%8D)就會出問題了，舉例來說："é"對應的Code Point是U+00e9:帶尖音符的拉丁文小寫字母e，但"é"卻是編碼成U+0065:拉丁文小寫字母e跟U+0301:結合尖音符，明明肉眼看起來兩者相同，但編碼起來卻截然不同，這時候就會有問題了，你將本來應該一起表示的"é"拆成兩個反轉，反轉過來我們可不知道他們兩個應該放在一起表示才有意義，這會導致預期不同。這時候要使用[unicode_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/)來根據grapheme cluster將文字拆分在進行反轉。

上面說起來很簡單，但深入瞭解卻很複雜，我們需要理解 Code Space, Code Point, Unicode Scalar Value之間的關係。

補充：一個`char`是一個“Unicode Scalar Value”。等同於UTF-32，使用4個bytes來表示。

### Code Space

Unicode Codespace：原始的Unicode標準是一個16位編碼系統，提供了大約65,536個字符的空間。然而，幾年後，標準被進一步擴展，使用**21位存儲**，這就是當前Unicode Codespace的結果，包括了0到0x10FFFF之間1,114,112個不同的值。

### Code Point

Code Point是在表示文本的系統（例如 Unicode）中分配來**表示抽象字符的數字**。在 Unicode 中，每個Code Point都用十六進制值表示，前綴為大寫字母U和加號（例如U+），像是U+1234，其中“1234”是分配的編號。例如：字符“A”分配的Code Point為 U+0041。
在Unicode Codespace中，**每個Character都有對應的Code Point，可能是一個Code Point，也可能由多個Code Point組成**，像是U+1F600（😀）、台灣國旗由兩個Code Point組成🇹: U+1F1F9 🇼: U+1F1FC（🇹🇼）。

### Unicode Scalar Value

**Unicode Scalars是Unicode Code Points的子集**，定義為包括U+0000到U+D7FF和U+E000到U+10FFFF之間的Code Points，介於U+D800和U+DFFF之間，可以發現其中有一些未包含的區域。這些空白區域被分配給Unicode surrogate pair code points，這些Code Points又分為兩組，即high-surrogate（U+D800到U+DBFF）和low-surrogate（U+DC00到U+DFFF）code points。這些僅由 UTF-16 使用，因此它可以表示所有Scalar Value。UTF-8 沒有這個問題；它是一種可變長度編碼方案（Code Point可以是 1-4 個bytes長），因此它可以在不使用Surrogate pairs的情況下對所有Scalars進行編碼。

總而言之，`char`遇到編碼介於U+D800和U+DFFF之間的會出錯，那些是非法的字元。`char`在utf-8裡面都有準確相對應的值（因為utf-8沒有用到surrogate pairs），因此可以安全的被轉換成utf-8，而`str`是utf-8，所以`char`跟`str`之間的轉換是安全的。

簡單來說，落在U+0000到U+D7FF和U+E000到U+10FFFF之間就叫做Unicode Scalar Value，落在U+D800到U+DBFF叫做high-surrogate，落在U+DC00到U+DFFF就叫做low-surrogate，全部合起來就叫做Unicode Code Point。

## Instructions

Reverse a string

For example:
input: "cool"
output: "looc"

Test your function on this string: `uüu` and see what happens. Try to write a function that properly
reverses this string. Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the
last test, and execute the tests with:

```bash
$ cargo test --features grapheme
```

You will need to use external libraries (a `crate` in rust lingo) for the bonus task. A good place to look for those is [crates.io](https://crates.io/), the official repository of crates.

[Check the documentation](https://doc.rust-lang.org/cargo/guide/dependencies.html) for instructions on how to use external crates in your projects.

## Source

### Created by

- @coriolinus

### Contributed to by

- @cbzehner
- @ccouzens
- @cwhakes
- @efx
- @ErikSchierboom
- @hunger
- @lutostag
- @ocstl
- @PaulT89
- @petertseng
- @rofrol
- @rrredface
- @stringparser
- @TheDarkula
- @xakon
- @ZapAnton

### Based on

Introductory challenge to reverse an input string - https://medium.freecodecamp.org/how-to-reverse-a-string-in-javascript-in-3-different-ways-75e4763c68cb