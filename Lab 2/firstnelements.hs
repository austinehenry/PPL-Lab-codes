-- Function to get the first n elements of a list
firstNElements :: Int -> [a] -> [a]
firstNElements n = take n  -- `take n` gets the first n elements of the list

-- Example usage:
main = do
    let nums = [1, 2, 3, 4, 5]
    let n = 3
    print (firstNElements n nums)  -- Outputs: [1, 2, 3]
