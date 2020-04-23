class VertexCalculation:
  """
  Stores the value of A[i] including caching
  whether w-i was included.
  """
  @staticmethod
  def from_last(w, vp, vpp):
    if vp.mwis > vpp.mwis + w:
      mwis = vp.mwis
      include_w = False
    else:
      mwis = vpp.mwis + w
      include_w = True
    return VertexCalculation(mwis, include_w)

  def __init__(self, mwis, include_w):
    self.mwis = mwis
    self.include_w = include_w


A = []
num_v = None
with open('input.txt') as f:
  for l in f:
    num = int(l)
    if not num_v:
      num_v = num
      continue
    if not A:
      A.append(VertexCalculation(0, None))
      A.append(VertexCalculation(num, True))
    else:
      vc = VertexCalculation.from_last(num, A[-1], A[-2])
      A.append(vc)

print(A[-1].mwis)
path_includes = [False for _ in A]
i = len(A)-1
while i >= 1:
  if A[i].include_w:
    path_includes[i] = True
    i -= 2
  else:
    i -= 1

queries = [1, 2, 3, 4, 17, 117, 517, 997]
res = []
for q in queries:
  if q > len(path_includes)-1:
    res.append(False)
  else:
    res.append(path_includes[q])
print(res)
