-- Function to double each number in a list and increment it by 1 using composition
doubleAndIncrement :: [Int] -> [Int]
doubleAndIncrement = map (\x -> (2 * x) + 1)  -- Map applies the function (2 * x) + 1 to each element

-- Example usage:
main = do
    let nums = [1, 2, 3, 4]
    print (doubleAndIncrement nums)  -- Outputs: [3, 5, 7, 9]
