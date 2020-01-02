#include <queue>
#include <set>
#include <utility>

#include <boost/optional.hpp>

namespace alg {
    using OptionalInt = boost::optional<int>;
    using OptionalIntPair = std::pair<OptionalInt, OptionalInt>;
    class MedianTracker {
        public:
            MedianTracker(size_t k) : k_(k), median_(sorted_.end()) {}
            void add_number(int n);
            OptionalIntPair median();
        private:
          size_t k_;
          std::multiset<int> sorted_;
          using SortIt = std::multiset<int>::iterator;
          SortIt median_;
          std::queue<SortIt> queue_;
    };
} // alg
