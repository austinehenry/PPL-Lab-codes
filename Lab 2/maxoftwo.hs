-- Function to find the maximum of two integers
maxOfTwo :: Int -> Int -> Int  -- Type declaration: takes two Ints and returns an Int
maxOfTwo x y
    | x > y     = x  -- If x is greater, return x
    | otherwise = y  -- Otherwise, return y

-- Example usage:
main = do
    let a = 10
    let b = 20
    print (maxOfTwo a b)  -- Outputs: 20
