// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech
}

struct Employee {
    employed: bool,
    employee_type: EmployeeType
}

fn can_access_building(employee: Employee) -> Result<(), String> {

    match employee.employed {
        false => return Err("terminated cannot access".to_owned()),
        _ => ()
    }

    match employee.employee_type {
        EmployeeType::Maintenance => Ok(()),
        EmployeeType::Manager => Ok(()),
        EmployeeType::Marketing => Ok(()),
        _ => return Err("no access allowed".to_owned())
        }
}

fn print_access(employee: Employee) -> Result<(), String> {
    can_access_building(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let employee = Employee{
        employed: true,
        employee_type: EmployeeType::Manager
    };
    let result = print_access(employee);
    match result {
        Err(e) => println!("access denied: {:?}", e),
        _ => () 
    }
}
