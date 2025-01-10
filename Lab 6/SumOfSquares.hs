-- Define the curried function applyOp
applyOp :: (Num a) => (a -> a -> a) -> [a] -> a
applyOp _ [] = 0
applyOp op (x:xs) = op x (applyOp op xs)

-- Function to filter even numbers, square them, and sum them
sumOfSquaresOfEvens :: [Int] -> Int
sumOfSquaresOfEvens xs = applyOp (+) (map (^2) (filter even xs))

-- Sample Input
result = sumOfSquaresOfEvens [1, 2, 3, 4, 5, 6]
