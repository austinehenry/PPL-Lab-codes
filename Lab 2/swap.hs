-- Function to swap elements of a tuple
swap :: (a, b) -> (b, a)
swap (x, y) = (y, x)  -- Switches the positions of the tuple elements

-- Example usage:
main = do
    let tuple = (1, "Hello")
    print (swap tuple)  -- Outputs: ("Hello", 1)
