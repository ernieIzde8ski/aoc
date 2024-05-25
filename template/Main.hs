#!/usr/bin/env cabal
{- cabal:
build-depends:
    base ^>= 4.15.0.0
-}

import Control.Exception
import Data.Char (isSpace)

readInput :: IO String
readInput = do
  result <- try (readFile "input.txt") :: IO (Either SomeException String)
  case result of
    Left _ -> do
      putStrLn "input.txt not available! reading from sample-input.txt"
      input <- readFile "sample-input.txt"
      output <- readFile "sample-output.txt"
      putStrLn $ "expected sample output: " ++ trim output
      return input
    Right text -> return text
  where
    trim = f . f
    f = reverse . dropWhile isSpace

-- part 1
partOne :: String -> Int
partOne _ = -1

-- part 2
partTwo :: String -> Int
partTwo _ = -1

-- main
main = do
  text <- readInput
  let output = show (partOne text) :: String
  putStrLn $ "part one: " ++ output

  let output = show (partTwo text) :: String
  putStrLn $ "part two: " ++ output
