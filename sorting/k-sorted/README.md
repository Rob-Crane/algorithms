# K-Sorted Array Sort

## Problem

Given an array `A` where each element is `k` positions away from it's sorted position, efficiently sort `A`.

## Solution

Scan through input array and maintain a min-heap of next `k` elements.  Set the current position `i` to the min of heap and push `i+k`th item onto heap.  Solves problem in `O(n log(k))`.
