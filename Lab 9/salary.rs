fn main() {
    let employee = (1, "Alice", 45000);
    let updated_employee = apply_salary_hike(employee);
    println!("Updated Employee: {:?}", updated_employee);
}

fn apply_salary_hike(employee: (u32, &str, u32)) -> (u32, &str, u32) {
    let (id, name, salary) = employee;
    let new_salary = if salary < 50000 { salary + (salary * 10 / 100) } else { salary };
    (id, name, new_salary)
}
