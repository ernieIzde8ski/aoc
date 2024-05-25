#!/usr/bin/env cabal
{- cabal:
build-depends:
    base ^>= 4.15.0.0,
    split ^>= 0.2.5
-}
{-# LANGUAGE NamedFieldPuns #-}

import Control.Exception
import Data.Char (isSpace)
import Data.List.Split
import Text.Read (Lexeme (String))

-- input reading/parsing

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

parseInput = map (mapToBox . mapToInt . splitOn "x") . lines
  where
    mapToBox arr = Box {h = head arr, l = arr !! 1, w = arr !! 2}
    mapToInt = map (read :: String -> Int)

data Box = Box
  { h :: Int,
    l :: Int,
    w :: Int
  }
  deriving (Show)

-- part 1
partOne :: [Box] -> Int
partOne = sum . map (paperArea . surfaceAreas)
  where
    paperArea :: (Int, Int, Int) -> Int
    paperArea (a, b, c) = minimum [a, b, c] + (2 * (a + b + c))
    surfaceAreas (Box {h, l, w}) = (l * w, w * h, h * l)

-- part 2
partTwo :: [Box] -> Int
partTwo = sum . map totalLength
  where
    totalLength box = wrapLength box + ribbonLength box
    wrapLength (Box {h, l, w}) = minimum [h + l, l + w, w + h] * 2
    ribbonLength (Box {h, l, w}) = h * l * w

-- main
main = do
  text <- readInput
  let boxes = parseInput text

  let output = show (partOne boxes) :: String
  putStrLn $ "part one: " ++ output

  let output = show (partTwo boxes) :: String
  putStrLn $ "part two: " ++ output
