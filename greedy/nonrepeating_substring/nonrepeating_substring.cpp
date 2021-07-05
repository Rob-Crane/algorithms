constexpr size_t kNumChars = '~' - ' ' + 1;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        std::array<int, kNumChars> last_seen;
        for (int& x : last_seen) {
            x = -1; // -1 signals unset.
        }
        int best = 0;
        int begin = -1;
        for (int i = 0; i < s.size(); ++i) {
            size_t c_idx = s[i] - ' ';
            assert(c_idx < last_seen.size());
            if (last_seen[c_idx] != -1 && last_seen[c_idx] > begin) {
                begin = last_seen[c_idx];
            }
            last_seen[c_idx] = i;
            int l = i - begin;
            if (l > best) {
                best = l;
            }
        }
        return best;
    }
};