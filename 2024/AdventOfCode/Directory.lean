-- https://specifications.freedesktop.org/basedir/latest/

namespace Directory

----------


abbrev Path := System.FilePath
def Path.mk := System.mkFilePath
-- cool, strings coerce to paths
-- don't need `.ofString` then

----------

private def home_dir: IO (Option Path) := do
  match (← IO.getEnv "HOME") with
  | some val => return some val
  | none => do
  match (← IO.getEnv "USER") with
  | some user => return some ("/home" / user)
  | none => pure none

private def directory
    (envKey: String)
    (fallback: String)
    (ns: Option String)
    : IO (Option Path) := do

    let optBase: IO (Option Path) := do
      match (← IO.getEnv envKey) with
      | some str => return some str
      | none => do
      match (← home_dir) with
      | some path => return some (path / fallback)
      | none => pure none

    match (← optBase, ns) with
    | (some base, some name) => return some (base / name)
    | (optBase, _) => return optBase

----------

def config_home(ns: Option String := none) :=
  directory "XDG_CONFIG_HOME" ".config" ns

def data_home(ns: Option String := none) :=
  directory "XDG_DATA_HOME" ".local/share" ns
