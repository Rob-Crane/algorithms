## Gradeschool Multiplication 

C++ implementation of a "gradeschool multiplication" algorithm.  Unlike multiplication operations on C++ numeric types, this isn't bound by the limits of integer representation.  [CPython uses this technique](https://github.com/python/cpython/blob/6f2a8c08573c71b78d2f6e2bfaf31641a0cd092b/Objects/longobject.c#L102) for integers less than 70 digits (and the recursive Karatsuba technique for larger).

## Usage
Only external dependencies is [Catch unit test framework](https://github.com/catchorg/Catch2) if you want to run unit tests and CMake if you want to use the CMake file.  If you have those, it should be as simple as:
```
mkdir build
cd build
cmake ..
make
```

### Improvements
* Support negative integers.
* The "carryover" term maintained while computing the sum of the partial products is represented with an unsigned integer.  For large inputs, this could overflow.
