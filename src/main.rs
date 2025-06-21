use std::io;

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(average: f32) -> char {
    if average >= 90.0 {
        'A'
    } else if average >= 75.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks_input = String::new();
    let mut subjects_input = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter total marks obtained:");
    io::stdin().read_line(&mut total_marks_input).expect("Failed to read marks");

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects_input).expect("Failed to read subjects");

    let name = name.trim();
    let total_marks: f32 = total_marks_input.trim().parse().expect("Invalid marks input");
    let subjects: u32 = subjects_input.trim().parse().expect("Invalid subject count");

    let average = calculate_average(total_marks, subjects);
    let grade = assign_grade(average);

    println!("\nStudent Report");
    println!("===============");
    println!("Name    : {}", name);
    println!("Average : {:.2}", average);
    println!("Grade   : {}", grade);
}
