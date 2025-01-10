sumStringLengths :: [String] -> Int
sumStringLengths list =
    foldl (\acc len -> acc + len) 0 (map length list)

-- Example usage:
main :: IO ()
main = print (sumStringLengths ["hello", "world", "haskell"])

