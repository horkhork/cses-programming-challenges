https://cses.fi/problemset/task/1090

There are n children who want to go to a Ferris wheel, and your task is to find a gondola for each child.

Each gondola may have one or two children in it, and in addition, the total weight in a gondola may not exceed x. You know the weight of every child.

What is the minimum number of gondolas needed for the children?

Input

The first input line contains two integers n and x: the number of children and the maximum allowed weight.

The next line contains n integers p1,p2,…,pn: the weight of each child.

Output

Print one integer: the minimum number of gondolas.

Constraints
1≤n≤2⋅105
1≤x≤109
1≤pi≤x
Example

Input:
4 10
7 2 3 9

Output:
3


Got a working implementation but it's slow for the large test inputs. Using
flamegraph to see if I can find out where the slowness is

```bash
cargo install flamegraph

```

Getting perf working on Mac was a quick dead end, switch to Ubuntu box
```bash
sudo apt update\n
sudo apt install linux-tools-common\n
uname -r\n
sudo apt-get install linux-tools-4.15.0-140
sudo apt install linux-tools-generic linux-tools-4.15.0-140-generic
perf
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph ./target/release/ferris-wheel < index.html
```

Need to follow up getting perf to work, instead, I optimized the nested loop by
using two list indexes, one from the front and one from the end.
