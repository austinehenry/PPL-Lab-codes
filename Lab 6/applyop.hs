-- Curried function that takes an operation and applies it to a list of two numbers
applyOp :: (Int -> Int -> Int) -> [Int] -> Int
applyOp op [x, y] = op x y

-- Function to prompt user and compute the result
main :: IO ()
main = do
    -- Prompt for operation
    putStrLn "Choose an operation (+ or *):"
    op <- getLine

    -- Prompt for numbers
    putStrLn "Enter the first number:"
    num1 <- readLn
    putStrLn "Enter the second number:"
    num2 <- readLn

    -- Determine the operation and compute the result
    let result = case op of
                    "+" -> applyOp (+) [num1, num2]
                    "*" -> applyOp (*) [num1, num2]
                    _   -> error "Invalid operation"

    -- Output the result
    putStrLn ("The result is: " ++ show result)

