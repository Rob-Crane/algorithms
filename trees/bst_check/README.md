# BST Check

## Problem
Given a binary tree check that the following properties are obeyed:
* Every value in a node's left subtree is less than the node's value.
* Every value in a node's right subtree is greater than the node's value.
* Every value is distinct.

## Solution
From the properties, it's not hard to see that a traversal that doing a depth-first traversal to left of current node, visiting current node, then depth-first traversal to right of node will visit nodes in increasing order.  Solution is to traverse tree this way, keeping track of value of last "visited" node, and ensuring each subsequently visited node is strictly greater than the last.
