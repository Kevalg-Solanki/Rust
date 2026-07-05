struct Student {
    name: String,
    age: u8,
    marks: u8,
}
fn main() {
    let std1:Student = Student {
        name: String::from("Babu"),
        age: 12,
        marks: 43,
    };
    let std2:Student = Student {
        name: String::from("Jethalal"),
        age: 13,
        marks: 99,
    };
    let std3:Student = Student {
        name: String::from("Babita"),
        age: 12,
        marks: 80,
    };

    print_student(&std1);
    print_student(&std2);
    print_student(&std3);
}

fn print_student(student: &Student) {
    println!("Student Name = {}", student.name);
    println!("Student Age = {}", student.age);
    println!("Student Marks = {}", student.marks);
}
