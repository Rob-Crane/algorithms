# Longest Substring Without Repeating Characters
Given a string s, find the length of the longest substring without repeating characters.

## Approach
Greedy approach is appropriate because solution substring is easily dividable from the remaining input.

Strategy is to iterate from left to right, saving the last occurence of each character and tracking the beginning index of the last time a repeated element was encountered.
