import AdventOfCode
import AdventOfCode.d01

import Cli

def handleDate(p1: String → Int)(p2: String → Int): IO Unit := do
  println! "Enter text input:"
  let text ← (← IO.getStdin).readToEnd
  println! "Got it, hold on…"
  let a1 := p1 text
  let a2 := p2 text
  println! s!"Part 1: {a1}"
  println! s!"Part 2: {a2}"

def mainCmdRunner(p : Cli.Parsed): IO UInt32 := do
  let day := p.flag! "day" |>.as! Nat

  match day with
  | 0 => IO.eprintln "Get fucked nerd"
  | 1 => handleDate AOC.d01.part1 AOC.d01.part2
  | _ => IO.eprintln "Invalid date."

  return 0

def mainCmd : Cli.Cmd := `[Cli|
  adventofcode VIA mainCmdRunner; ["0.0.1"]
  "Advent of Code 2024."

  FLAGS:
    d, day: Nat; "Selects a particular day."

  EXTENSIONS:
    Cli.defaultValues! #[("day", "0")]
]

def main := mainCmd.validate
