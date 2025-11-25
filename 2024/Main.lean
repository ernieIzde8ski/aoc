import AdventOfCode
import Cli

def mainCmdRunner(p : Cli.Parsed): IO UInt32 := do
  let verbose := p.hasFlag "verbose"
  let debug := p.hasFlag "debug"
  let day := p.flag! "day" |>.as! Nat

  IO.println s!"Verbose: {verbose}"
  IO.println s!"Debug: {debug}"
  IO.println s!"Day: {day}"

  return 0

def mainCmd : Cli.Cmd := `[Cli|
  exampleCmd VIA mainCmdRunner; ["0.0.1"]
  "Advent of Code code runner."

  FLAGS:
    verbose;    "Enables verbose output."
    d, day: Nat; "Selects a particular day."
    debug;        "Uses debug input."

  EXTENSIONS:
    Cli.defaultValues! #[("day", "0")]
]

def main := mainCmd.validate
