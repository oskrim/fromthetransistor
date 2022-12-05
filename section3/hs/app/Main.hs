module Main where

import Parser
import System.Environment

process :: String -> IO ()
process input = do
  let res = compile input
  print res

main :: IO ()
main = do
  args <- getArgs
  input <- readFile $ head args
  process input
