#include<iostream>
#include <vector>

#include "rolling_median.h"


int main() {

    int d = 5;
    std::vector<int> input = {2, 3, 4, 2, 3, 6, 8, 4, 5};
    std::vector<int> medians;
    alg::MedianTracker tracker(d);
    for (int number : input) {
        tracker.add_number(number);
        alg::OptionalIntPair med = tracker.median();
        std::cout<<*med.first;
        if (med.second) {
            std::cout<< ", " << *med.second;
        }
        std::cout<<std::endl;
    }
    return 0;
}
