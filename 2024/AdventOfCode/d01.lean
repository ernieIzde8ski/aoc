namespace AOC.d01

def parseLists(str: String) :=
  str.splitOn "\n"
    |> .filter (not ∘ String.isEmpty)
    |> .map parseLine
    |> List.unzip
  where
    parseLine s : Nat × Nat :=
      s.splitOn "   "
      |> List.map String.toNat!
      |> λx↦(x[0]!, x[1]!)


def part1(str: String): Int :=
  parseLists str
    |> λ(lefts, rights) ↦ (lefts.mergeSort, rights.mergeSort)
    |> λ(lefts, rights) ↦ List.zip lefts rights
    |> .map distance
    |> .map (Int.ofNat ∘ Int.natAbs)
    |> List.sum
  where
    distance : Nat × Nat → Int
    | (a, b) => (Int.ofNat a) - (Int.ofNat b)

#guard part1 "3   4
4   3
2   5
1   3
3   9
3   3" == 11

def part2(str: String): Int :=
  panic! "not implemented"

end AOC.d01
