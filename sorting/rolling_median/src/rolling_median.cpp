#include "rolling_median.h"

#include <cassert>
#include <iterator>

namespace alg {

    void MedianTracker::add_number(int n) {
        SortIt insert_it = sorted_.insert(n);
        queue_.push(insert_it);

        // Update median after adding new value.
        if (median_ == sorted_.end()) { // initialize
            median_ = insert_it;
        } else {
            if (queue_.size() % 2 == 0 && n < *median_) {
                --median_;
            } else if (queue_.size() % 2 == 1 && n >= *median_) {
                ++median_;
            }
        }

        if (queue_.size() > k_) {
            // Remove value and update median
            SortIt remove_it = queue_.front();
            if (queue_.size() == 1) { // degenerate case k = 0
                median_ = sorted_.end();
            }
            else if (queue_.size() % 2 == 0 && *remove_it <= *median_) {
                ++median_;
            } else if (queue_.size() % 2 == 1 && 
                       (remove_it == median_ ||
                        *remove_it > *median_)) {
                --median_;
            }
            queue_.pop();
            sorted_.erase(remove_it);
        }
        assert(queue_.size() == sorted_.size());
    }

    OptionalIntPair MedianTracker::median() {
        OptionalIntPair ret;
        if (median_ != sorted_.end()) {
            ret.first = *median_;
            SortIt next = std::next(median_);
            if (queue_.size() % 2 == 0 && next != sorted_.end()) {
                ret.second = *next;
            }
        }
        return ret;
    }
} // alg
