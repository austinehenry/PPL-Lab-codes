-- Define the function to compute the sum of a list
sumList :: [Int] -> Int
sumList [] = 0                -- Base case: the sum of an empty list is 0
sumList (x:xs) = x + sumList xs  -- Recursive case: add the head to the sum of the tail

-- Main function to test the sumList function
main :: IO ()
main = print (sumList [1, 2, 3, 4, 5])
