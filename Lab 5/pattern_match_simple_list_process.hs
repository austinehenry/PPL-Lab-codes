firstTwoElements :: [a] -> [a]
firstTwoElements xs
    | length xs >= 2 = take 2 xs
    | otherwise      = xs

