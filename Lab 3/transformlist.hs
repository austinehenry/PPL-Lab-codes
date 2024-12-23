-- Define the square function first
square :: Int -> Int
square x = x * x

-- Define the add10 function
add10 :: Int -> Int
add10 x = x + 10

-- Define the transformList function using function composition
transformList :: [Int] -> [Int]
transformList xs = map (add10 . square) xs

