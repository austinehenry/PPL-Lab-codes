-- Function to calculate the square of an integer
square :: Int -> Int  -- Type declaration: takes an Int and returns an Int
square x = x * x      -- Multiplies the number by itself

-- Example usage:
main = do
    let num = 5  -- Declare a number
    print (square num)  -- Outputs: 25
