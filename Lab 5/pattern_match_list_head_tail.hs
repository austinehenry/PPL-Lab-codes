firstElement :: Show a => [a] -> String
firstElement (x : _) = "First element is " ++ show x
firstElement [] = "The list is empty."

