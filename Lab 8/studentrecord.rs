// Define the Student struct
#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u32,
    grade: String,
}

// Function to display student records (immutable borrow)
fn display_students(students: &[Student]) {
    for student in students {
        println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
    }
}

// Function to modify a student's grade (mutable borrow)
fn update_grade(student: &mut Student, new_grade: String) {
    student.grade = new_grade;
}

fn main() {
    // Create a Vec to store multiple students
    let mut students = Vec::new();

    // Add students to the record system
    students.push(Student {
        name: String::from("Alice"),
        age: 20,
        grade: String::from("B"),
    });
    students.push(Student {
        name: String::from("Bob"),
        age: 22,
        grade: String::from("A"),
    });

    // Display students using immutable borrow
    println!("Student Records:");
    display_students(&students);

    // Update Bob's grade using mutable borrow
    update_grade(&mut students[1], String::from("A+"));

    // Display updated records
    println!("\nUpdated Student Records:");
    display_students(&students);
}
