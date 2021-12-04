class UnionFind:
  def __init__(self, N):
    self.par = []
    for i in range(N):
      self.par.append(i)
  def find(self, x):
    if x != self.par[x]: 
      self.par[x] = self.find(self.par[x])
    return self.par[x]
  def unite(self, x, y):
    x = self.find(x)
    y = self.find(y)
    self.par[y] = x
  def same(self, x, y):
    return self.find(self.par[x]) == self.find(self.par[y])
NQ = list(map(int, input().split()))
N = NQ[0]
Q = NQ[1]
uf = UnionFind(N)
for i in range(0, Q):
  PAB = list(map(int, input().split()))
  P = PAB[0]
  A = PAB[1]
  B = PAB[2]
  if P == 0:
    uf.unite(A, B)
  else:
    print("YNeos"[not uf.same(A, B)::2])

