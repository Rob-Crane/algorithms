
import numpy as np

A = [[1,1,1],
     [1,0,0],
     [0,1,0]]

w, v = np.linalg.eig(A)
v_inv = np.linalg.inv(v)

def count_ways(n):

    if n < 1:
        return 0
    if n == 1:
        return 1
    if n == 2:
        return 2
    if n == 3:
        return 4

    q = np.power(w, n-3)
    solution = v.dot(np.identity(3) * q).dot(v_inv).dot([4,2,1])
    return int(solution[0].real)

print(count_ways(100))
