NY = list(map(int, input().split()))
N = NY[0]
Y = NY[1]
a = 0-1
n = 0-1
s = 0-1
for x in range(N+1):
  for y in range(N+1-x):
    z = N - (x + y)
    if 10000*x + 5000*y + 1000*z == Y:
      a = x
      n = y
      s = z
print(a, n, s)
