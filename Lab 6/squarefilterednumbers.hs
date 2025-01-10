composeFilterMap :: (a -> Bool) -> (a -> b) -> [a] -> [b]
composeFilterMap filterFn mapFn =
    map mapFn . filter filterFn

-- Example usage:
squareFilteredNumbers :: [Int] -> [Int]
squareFilteredNumbers = composeFilterMap (<=5) (^2)

main :: IO ()
main = print (squareFilteredNumbers [3, 7, 2, 8, 4, 6])

