import bisect
NK = list(map(int, input().split()))
N = NK[0]
K = NK[1]
A = list(map(int, input().split()))
idx = bisect.bisect_left(A, K)
if idx == len(A): 
  idx = -1
print(idx)

