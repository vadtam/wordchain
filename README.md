# Word Chain Game

## Intro
A program which solves the "word chain game". A word chain is a collection of words created by mutating one letter from the previous one, e.g. cat -> sat -> sit. In the word chain game you are given two words and asked to find the length of the shortest word chain which connects them (if one exists). As input we are given a list of words along with the two to find the chain between. We assume all words in the given list are the same length, and are only made up of ASCII characters.

## Discussion
1. The fact that the words are sorted and of ASCII type, this gives us a hint that we can check the existence of a word for O(1) or O(log2_N) time depending on a collection type. ASCII characters condition reduce the combinatorical search.
2. The parallel processing of graph construction and optimisation can be utilised (such as early exit if the words are equal), but skipped for simplicity.
3. The complexity to build a graph is currently N^2. Can this complexity be reduced? The words sorting and ASCII usage give us a hint that the neighbour nodes can be detected dynamically by combinatorical search of 256*word_len attempts for every node.