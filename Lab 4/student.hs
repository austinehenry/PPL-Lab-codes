averageMarks :: (String, Int, [Int]) -> (String, Double)
averageMarks (name, _, marks) = (name, average marks)
  where
    average xs = if null xs then 0 else fromIntegral (sum xs) / fromIntegral (length xs)

displayStudentAverages :: [(String, Int, [Int])] -> [(String, Double)]
displayStudentAverages [] = []
displayStudentAverages (s:students) = averageMarks s : displayStudentAverages students

students = [("Messi", 101, [90, 85, 88]), ("Ronaldo", 102, [78, 84, 82]), ("Neymar", 103, [95, 92, 90]) ]

main :: IO ()
main = print (displayStudentAverages students)
