-- Define the type for a student: (Name, Roll Number, Marks List)
type Student = (String, Int, [Int])

-- Function to calculate the average marks for a student
averageMarks :: Student -> Float
averageMarks (_, _, marks) = 
    let totalMarks = sum marks
        numSubjects = length marks
    in if numSubjects > 0 then fromIntegral totalMarks / fromIntegral numSubjects else 0.0

-- Function to display all student names and their average marks
displayStudentAverages :: [Student] -> [(String, Float)]
displayStudentAverages students = [(name, averageMarks student) | (name, _, _) <- students]

-- Test data (list of students)
students :: [Student]
students = [("Alice", 101, [85, 90, 78]), 
            ("Bob", 102, [92, 88, 81, 76]), 
            ("Charlie", 103, [78, 83, 85])]

-- Main function to test the code
main :: IO ()
main = do
    let averages = displayStudentAverages students
    print averages

