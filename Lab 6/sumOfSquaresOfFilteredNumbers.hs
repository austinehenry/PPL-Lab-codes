-- Function to filter numbers greater than 10, square the remaining numbers, and sum them
sumOfSquaresOfFilteredNumbers :: [Int] -> Int
sumOfSquaresOfFilteredNumbers numbers = sum (map (^2) (filter (<= 10) numbers))

-- Sample Input
main :: IO ()
main = print (sumOfSquaresOfFilteredNumbers [5, 12, 9, 20, 15])
