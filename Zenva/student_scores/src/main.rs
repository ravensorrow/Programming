/* 3. Write letter grade function, takes grade as parameter and returns a letter grade */
fn get_letter_grade(score: i32) -> char {
     if score >=90 && score <=100 { 
        'A' 
    } else if score >=80 && score <90 {
        'B'
    } else if score >=70 && score <80 {
        'C'
    } else if score >=60 && score <70 {
        'D'
    } else {
        'F'
    }
}

fn main() {
    /* 1. Create an array of student scores */
    let student_scores = [85, 88, 90, 78, 92, 99, 66, 40, 105];

    /* 2. Use a for loop to iterate through the array and call get_letter_grade function */
    for score in student_scores {
        let letter_grade = get_letter_grade(score);

        /* 4. Print each student's scores along with the corresponding letter grade */
        println!("Student Score : {}, Letter Grade :{}", score, letter_grade);
    }
}
