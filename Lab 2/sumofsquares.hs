-- Function to calculate the sum of squares of elements in a list
sumOfSquares :: [Int] -> Int
sumOfSquares = sum . map (\x -> x * x)  -- Map squares each element, then sum adds them up

-- Example usage:
main = do
    let nums = [1, 2, 3]
    print (sumOfSquares nums)  -- Outputs: 14
