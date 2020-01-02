#define CATCH_CONFIG_MAIN
#include "catch.hpp"

#include "rolling_median.h"

using namespace alg;

TEST_CASE("empty") {
  MedianTracker tracker(0);
  tracker.add_number(1);
  tracker.add_number(2);

  OptionalIntPair expected = {boost::none, boost::none};
  bool is_none = tracker.median() == expected;
  REQUIRE(is_none);
}


TEST_CASE("single value" ) {
  MedianTracker tracker(1);
  tracker.add_number(1);
  tracker.add_number(2);

  OptionalIntPair expected = {2, boost::none};
  REQUIRE(tracker.median() == expected);
}


TEST_CASE("not full even" ) {
  MedianTracker tracker(5);
  tracker.add_number(1);
  tracker.add_number(2);
  tracker.add_number(3);
  tracker.add_number(4);

  OptionalIntPair expected = {2, 3};
  REQUIRE(tracker.median() == expected);
}


TEST_CASE("not full odd" ) {
  MedianTracker tracker(5);
  tracker.add_number(1);
  tracker.add_number(2);
  tracker.add_number(3);
  OptionalIntPair expected = {2, boost::none};
  REQUIRE(tracker.median() == expected);
}


TEST_CASE("full even") {
  MedianTracker tracker(3);
  tracker.add_number(1);
  tracker.add_number(2);
  tracker.add_number(3);
  tracker.add_number(4);

  OptionalIntPair expected = {3, boost::none};
  REQUIRE(tracker.median() == expected);
}


TEST_CASE("full odd") {
  MedianTracker tracker(3);
  tracker.add_number(1);
  tracker.add_number(2);
  tracker.add_number(3);
  tracker.add_number(4);
  tracker.add_number(5);

  OptionalIntPair expected = {4, boost::none};
  REQUIRE(tracker.median() == expected);
}


TEST_CASE("repeats") {
  MedianTracker tracker(3);
  tracker.add_number(1);
  tracker.add_number(2);
  tracker.add_number(2);
  tracker.add_number(4);
  tracker.add_number(5);
  tracker.add_number(2);

  OptionalIntPair expected = {4, boost::none};
  REQUIRE(tracker.median() == expected);
}
