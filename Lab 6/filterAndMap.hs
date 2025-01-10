filterAndMap :: (a -> Bool) -> (a -> b) -> [a] -> [b]
filterAndMap predicate mapFn list =
    map mapFn (filter predicate list)

-- Example usage:
doubleEvens :: [Int] -> [Int]
doubleEvens = filterAndMap even (*2)

main :: IO ()
main = print (doubleEvens [1, 2, 3, 4, 5, 6])

