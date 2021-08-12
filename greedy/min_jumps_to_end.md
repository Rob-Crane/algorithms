# Jump Game (2)
Given an array of non-negative integers nums, you are initially positioned at the first index of the array.

Each element in the array represents your maximum jump length at that position.

Your goal is to reach the last index in the minimum number of jumps.

## Approach
Assume you're on an optimal jump, that is, jumping from a position along the optimal jump sequence. Let the jump start at position `i` and end at position `j`. The next jump in the optimal sequence will start from somewhere in `[i+1, j]`.

Which value should be chosen for the starting point of the next jump? As long as the chosen starting point can reach at least as far as any other starting point in `[i+1, j]`, there's no reason to choose another. Another choice can only do worse. For example, let's say starting point `k` in `[i+1, j]` can reach `j+3` which is the furthest reachable from any other point in `[i+1, j]`. If `j+3` is the optimal sequence, we must choose `k`. If `j+3` is not in the optimal sequence and something before it is, a jump from `k` can still reach that position.

Given this observation, the approach is to track the maximum reachable position from each jump as we iterate along the array. When we reach the end of our "current" jump, increment our jump counter and assume the next jump was made to that maximal position. This requires only a single pass of the input and constant additional storage:
```cpp
int minJumps(const vector<int>& nums) {
  if (nums.size() == 1) {
      return 0;
  }

  // Take a jump from 0 position.
  int n_jumps = 1;
  size_t curr_jump = nums[0];
  size_t next_max = 0;

  assert(curr_jump>=1);

  for (size_t i = 1; i < nums.size(); ++i) {
      
      // Can reach end on current jump.
      if (curr_jump >= nums.size()-1) {
          break;
      }
      
      // Maintain the maximum reachable position from positions
      // along the current jump.
      next_max = max(next_max, i + nums[i]);
          
      // Take another jump when end of current jump reached or
      // determined end is reachable from current jump.
      if (i == curr_jump || next_max >= nums.size()-1) {
          ++n_jumps;
          curr_jump = next_max;
      }
  }
  return n_jumps;
}
```
