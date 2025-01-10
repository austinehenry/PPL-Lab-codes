-- Curried function to compose two functions
compose :: (b -> c) -> (a -> b) -> (a -> c)
compose f g = \x -> f (g x)

-- Function to multiply by 2
multiplyBy2 :: Num a => a -> a
multiplyBy2 x = x * 2

-- Function to subtract 3
subtract3 :: Num a => a -> a
subtract3 x = x - 3

-- Composed function
composedFunction :: Num a => a -> a
composedFunction = compose multiplyBy2 subtract3

-- Apply the composed function to a list
applyToList :: Num a => [a] -> [a]
applyToList xs = map composedFunction xs

-- Sample Input
main :: IO ()
main = print (applyToList [1, 2, 3, 4])

