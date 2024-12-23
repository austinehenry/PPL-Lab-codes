-- Function to add pairs of integers in a list of tuples
addPairs :: [(Int, Int)] -> [Int]
addPairs = map (\(x, y) -> x + y)  -- Map applies addition of the two tuple elements

-- Example usage:
main = do
    let pairs = [(1, 2), (3, 4), (5, 6)]
    print (addPairs pairs)  -- Outputs: [3, 7, 11]
