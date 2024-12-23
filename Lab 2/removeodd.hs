-- Function to remove all odd numbers from a list
removeOdd :: [Int] -> [Int]
removeOdd = filter even  -- Filters the list, keeping only even numbers

-- Example usage:
main = do
    let nums = [1, 2, 3, 4, 5]
    print (removeOdd nums)  -- Outputs: [2, 4]
