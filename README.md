# Student Grade App

A simple Rust console application that stores a student’s name, total marks, and number of subjects.  
It calculates the average marks using a custom function and assigns a grade based on the average:

- *A:* 90 and above  
- *B:* 75 to 89  
- *C:* 60 to 74  
- *D:* Below 60  

## How to Run

1. Make sure you have [Rust installed](https://rustup.rs/).

2. Clone the repository:

   ```bash
   git clone https://github.com/your-username/student-grade-app.git
   cd student-grade-app

3. Run the app: 

   ```bash
   cargo run


4. Follow the prompts to enter the student's name, total marks, and number of subjects.



Code Overview

main() handles user input and output.

calculate_average() computes the average marks.

assign_grade() returns the grade character based on the average.
