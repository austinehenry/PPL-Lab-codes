-- Function to calculate the factorial of a number using recursion
factorial :: Int -> Int
factorial 0 = 1               -- Base case: factorial of 0 is 1
factorial n = n * factorial (n - 1)  -- Recursive step

-- Example usage:
main = do
    let n = 5
    print (factorial n)  -- Outputs: 120
