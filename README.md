# Word Chain Game

## Problem statement
A program which solves the "word chain game". A word chain is a collection of words created by mutating one letter from the previous one, e.g. cat -> sat -> sit. In the word chain game you are given two words and asked to find the length of the shortest word chain which connects them (if one exists). As input we are given a list of words along with the two to find the chain between. We assume all words in the given list are the same length, and are only made up of ASCII characters.

## Discussion for First attempted solution (with building a static graph)
1. The fact that the words are sorted and of ASCII type, this gives us a hint that we can check the existence of a word for O(1) or O(log2_N) time depending on a collection type. ASCII characters condition reduce the combinatorical search.
2. The parallel processing of graph construction and optimisation can be utilised (such as early exit if the words are equal), but skipped for simplicity.
3. The complexity to build a graph is currently N^2. Can this complexity be reduced? The words sorting and ASCII usage give us a hint that the neighbour nodes can be detected dynamically by combinatorical search of 128*word_len attempts for every node.
4. The algo implementation has a good optimisation feature of replacing strings with integers.

## Discussion for Second attempted solution (with building a dynamic graph)
- The ideas from the first solution have been developed to attempt to build a dynamic graph. The execution speed showed similar performance for the given dataset. The hypothesis is that this solution will outperform the first solution for bigger datasets.

## Remarkable properties of the word chain function
It is interesting to shine the light on some of the remarkable properties of the produced word chain function that can be useful for its applications and deep optimizations of its use.
- AbsMinVal. For every input, we can limit the value of the shortest chain from below by computing the sum of differences between "start" and "end" strings. The prove can be trivially done via mathematical induction or via a contradiction.
- AbsMaxVal. For every input, we can limit the value of the shortest chain from above. Lets define N as the total number of nodes. Lets define R as the minimum number of neighbors among the given nodes. Then N <= (R+1)AbsMaxVal / 3, => AbsMaxVal = ceiling(3N / (R+1)). Lets prove it by grouping each node into the group by the following principle. Group0 is the start. Group1 is the neighbours of the start. Group2 is the nodes that can be reached further from Group1. Etc. Lets observe that for each three sequential groups (GroupK-1, GroupK, GroupK+1) the total number of nodes in them is at least R+1 nodes, => on average (R+1)/3 per node. Therefore, the minimum number of nodes in between the arbitrary nodes is (R+1)AbsMaxVal/3 such that it is slightly above or equal to N. 

## Discussion for next steps
- The provided sorted words array and the ability to use lookup of the values at O(log2_N) can be utilized to build another solution. This is for example useful when there are memory limits.
- The application of the remarkable properties and the knowing typical datasets that this function is to be used at, can gain some insights into unobvious optimisation of the given function or its usage.
