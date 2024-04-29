#!/usr/bin/env cabal
{- cabal:
build-depends: base ^>= 4.15.0.0
-}

-- part one
readInput :: IO String
readInput = do readFile "input.txt"

evalChar :: Char -> Int
evalChar '(' = 1
evalChar ')' = -1
evalChar _ = 0

partOne :: String -> Int
partOne "" = 0
partOne (x : xs) = evalChar x + partOne xs

-- part two
_findFirstNegative :: Int -> Int -> [Char] -> Maybe Int
_findFirstNegative _ _ [] = Nothing
_findFirstNegative sum index char = do
  let newSum = sum + evalChar (char !! index)
  if newSum < 0
    then Just (index + 1)
    else _findFirstNegative newSum (index + 1) char

findFirstNegative :: [Char] -> Maybe Int
findFirstNegative = _findFirstNegative 0 0

partTwo :: String -> Maybe Int
partTwo = findFirstNegative

-- main
main = do
  text <- readInput
  let num = partOne text
  let text = show num :: String
  putStrLn $ "part one: " ++ text

  text <- readInput
  let num = partTwo text
  let text = show num :: String
  putStrLn $ "part two: " ++ text
