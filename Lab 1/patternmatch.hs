-- Function to check if a list is empty
isEmpty :: [a] -> Bool
isEmpty [] = True   -- If the list is empty, return True
isEmpty _  = False  -- Otherwise, return False

-- Main function to test isEmpty
main :: IO ()
main = do
    print (isEmpty [1, 2, 3])  -- Test with a non-empty list
    print (isEmpty [])         -- Test with an empty list
