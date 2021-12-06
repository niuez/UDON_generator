# 1本うどんコードトランスパイラ

[鈴鹿高専 Advent Calender 2021](https://qiita.com/advent-calendar/2021/snct) 12日目の企画です.

## 1本うどんコードとは

## 1本うどんコードトランスパイラとは

1本ではない普通のPythonスクリプトを, 1本うどんに麺打ちします.

## 対応している構文

[6. expression](https://docs.python.org/ja/3/reference/expressions.html)

- [`identifier` 識別子](./src/parser/identifier.rs)
- [`literal` リテラル](./src/parser/literal.rs)
  - `stringliteral` 文字列リテラル
    - `"`か`'`で囲まれたもの
    - エスケープには対応
  - `integer` 整数
    - 10進数のみ対応
  - `floatnumber` 浮動小数点数
- [`unary expression` 単項式](./src/parser/unary_expr.rs)
  - `Paren` 丸括弧
  - `List` リスト表示
    - コンマ区切りによる定義にのみ対応
  - `Tuple` タプル
  - `UnaryOpe` 単項算術演算
    - `+`, `-`, `~`,
  - `Subseq`
    - [`call` 呼び出し](./src/parser/call.rs) `primary(args)`
    - [`slicing` スライス表記](./src/parser/index.rs) `primary[index]`
      - 引数が一つしか指定できない 複数指定できると思っていなかった
    - [`member` 属性参照](./src/parser/member.rs) `primary.attr`
- [`expression` 式](./src/parser/expression.rs)
  - `not`
  - `2項演算`
    - `is` `is not` `in` `not in`以外は対応しているはず
    - トランスパイルするだけなので演算順序は考慮してない

[`7. 単純文`](https://docs.python.org/ja/3/reference/simple_stmts.html)
- [`simplestatement`](./src/parser/simplestatement.rs)
  - `expression`
  - `assignment` 代入文
    - 代入する先の形式`target_list`は限られている
    - `identifier`に`[<expression>]`か`.<identifier>`を繋げたもの
  - [`import` import文](./src/parser/import.rs)
    - 例
    - `import numpy`
    - `import numpy as np`
    - `from matplotlib import pyplot`
    - `from matplotlib import pyplot as plt`
    - `relative_module`がよくわからなかったので実装していない
[`8. 複合文`](https://docs.python.org/ja/3/reference/compound_stmts.html)
- 行間を入れると壊れます
- [`statement`](./src/parser/statement.rs)
  - `simple statement`
  - [`if statement`](./src/parser/if_stmt.rs)
  - [`for statement`](./src/parser/for_stmt.rs)
  - [`function definition`](./src/parser/func_def.rs)
    - 関数定義の最後のstatementでのみ`return`が使用可能
  - [`class definition`](./src/parser/class_def.rs)
    - 関数定義のみが使用可能

## トランスパイル

`cargo run <file_path>`でトランスパイルできる


## 例

```py
s = input()
print(s.count('1'))
```

```py
[s := input(), print(s.count("1"))]
```

```py
ab = list(map(int, input().split()))
a = ab[0]
b = ab[1]
if a*b % 2 == 1:
  print("Odd")
else:
  print("Even")
```

```py
[ab := list(map(int, input().split())), a := ab[0], b := ab[1], [print("Odd")] if a * b % 2 == 1 else [print("Even")]]
```
```py
NY = list(map(int, input().split()))
N = NY[0]
Y = NY[1]
a = -1
n = -1
s = -1
for x in range(N+1):
  for y in range(N+1-x):
    z = N - (x + y)
    if 10000*x + 5000*y + 1000*z == Y:
      a = x
      n = y
      s = z
print(a, n, s)
```

```py
[NY := list(map(int, input().split())), N := NY[0], Y := NY[1], a := -1, n := -1, s := -1, [[[[z := N - (x + y), [a := x, n := y, s := z] if 10000 * x + 5000 * y + 1000 * z == Y else []] for y in range(N + 1 - x)]] for x in range(N + 1)], print(a, n, s)]
```

```py
import bisect
NK = list(map(int, input().split()))
N = NK[0]
K = NK[1]
A = list(map(int, input().split()))
idx = bisect.bisect_left(A, K)
if idx == len(A): 
  idx = -1
print(idx)
```

```py
[bisect := __import__("bisect"), NK := list(map(int, input().split())), N := NK[0], K := NK[1], A := list(map(int, input().split())), idx := bisect.bisect_left(A, K), [idx := -1] if idx == len(A) else [], print(idx)]
```


```py
import numpy as np
from matplotlib import pyplot as plt
img = np.ones((100, 100), np.uint8) * 127
plt.imshow(img, cmap="gray")
plt.show()
```

```py
[np := __import__("numpy"), udon_import := __import__("matplotlib", fromlist=["pyplot"]), plt := udon_import.pyplot, img := np.ones((100, 100, ), np.uint8) * 127, plt.imshow(img, cmap="gray"), plt.show()]
```

```py
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
```

```py
[UnionFind := type("UnionFind", (), {"__init__": (lambda self, N : [self.__setattr__("par", []), [[self.par.append(i)] for i in range(N)], None][2]), "find": (lambda self, x : [[self.par.__setitem__(x, self.find(self.par[x]))] if x != self.par[x] else [], self.par[x]][1]), "unite": (lambda self, x, y : [x := self.find(x), y := self.find(y), self.par.__setitem__(y, x), None][3]), "same": (lambda self, x, y : [self.find(self.par[x]) == self.find(self.par[y])][0])}), NQ := list(map(int, input().split())), N := NQ[0], Q := NQ[1], uf := UnionFind(N), [[PAB := list(map(int, input().split())), P := PAB[0], A := PAB[1], B := PAB[2], [uf.unite(A, B)] if P == 0 else [print("YNeos"[slice(not uf.same(A, B), None, 2)])]] for i in range(0, Q)]]
```
