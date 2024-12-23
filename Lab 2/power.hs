-- Function to calculate power using recursion
power :: Int -> Int -> Int
power base 0 = 1               -- Base case: any number to the power of 0 is 1
power base exp = base * power base (exp - 1)  -- Recursive step

-- Example usage:
main = do
    let base = 2
    let exp = 3
    print (power base exp)  -- Outputs: 8
