use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: Vec<HashMap<String, String>> = Vec::new();
    loop {
        let employee = get_employee();
        let dept = get_dept();

        let mut dept_to_employee = HashMap::new();
        dept_to_employee.insert(String::from(dept), String::from(employee));
        company.push(dept_to_employee);

        println!("Do you want to enter more employees? (y or n)");
        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read the input");
        let ans = String::from(ans.trim());
        if let "y" = &*ans {
            continue;
        } else if let "n" = &*ans {
            break;
        } else {
            println!("Play by the rules.");
            break;
        }
    }
    println!("{:?}", company);
}

fn get_employee() -> String {
    println!("Please enter the name of the employee you want to add:");
    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read the input.");
    let employee = String::from(employee.trim());
    employee
}

fn get_dept() -> String {
    println!("Please enter the department you want to add them to:");
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read the input.");
    let dept = String::from(dept.trim());
    dept
}
