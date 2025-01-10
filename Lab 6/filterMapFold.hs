filterMapFold :: [Int] -> Int
filterMapFold list =
    foldl (*) 1 (map (*2) (filter (<=10) list))

-- Example usage:
main :: IO ()
main = print (filterMapFold [5, 12, 9, 20, 15])

