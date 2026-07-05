struct Emp {
    id: u32,
    name: String,
    salary: f32,
}

fn main() {
    //CREATE MUTABLE EMPLOYEE
    let mut emp1: Emp = Emp {
        id: 10,
        name: String::from("Jethalal"),
        salary: 10000.00,
    };

    //PRINT EMPLOYEE
    print_emp(&emp1);

    //INCREASE SALARY BY 10%
    emp1.salary = emp1.salary + emp1.salary * 0.10;

    print_emp(&emp1);
}

fn print_emp(emp: &Emp) {
    println!("Employee Id = {}", emp.id);
    println!("Employee Name = {}", emp.name);
    println!("Employee Salary = {}", emp.salary);
}
