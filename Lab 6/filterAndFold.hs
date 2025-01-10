filterAndFold :: (a -> Bool) -> (a -> b -> b) -> b -> [a] -> b
filterAndFold predicate foldFn initialValue list =
    foldr foldFn initialValue (filter predicate list)

-- Example usage:
sumOfOdds :: [Int] -> Int
sumOfOdds = filterAndFold odd (+) 0

main :: IO ()
main = print (sumOfOdds [1, 2, 3, 4, 5, 6])

