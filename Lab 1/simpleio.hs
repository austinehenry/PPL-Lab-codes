module Main where  -- Declare the Main module

-- Main function for user interaction
main :: IO ()
main = do
    putStrLn "What is your name?"  -- Prompt the user
    name <- getLine                -- Read the user's input
    putStrLn ("Hello, " ++ name)   -- Print the greeting
