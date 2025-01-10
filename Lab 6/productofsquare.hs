productOfSquaredOdds :: [Int] -> Int
productOfSquaredOdds list =
    foldl (*) 1 (map (^2) (filter odd list))

-- Example usage:
main :: IO ()
main = print (productOfSquaredOdds [1, 2, 3, 4, 5, 6])

