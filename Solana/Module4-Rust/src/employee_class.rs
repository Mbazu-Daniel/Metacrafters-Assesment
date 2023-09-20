
    // Define the Employee struct
    #[derive(Debug)]
    struct Employee {
        name: String,
        salary: f64,
        id: u32,
        employee_type: EmployeeType,
    }

    // Define the EmployeeType enum
    #[derive(Debug)]
    enum EmployeeType {
        ProductManager,
        BlockchainDeveloper,
        DataAnalyst,
        FullStackDeveloper,
    }

    // Implement a method to add and subtract salaries
    impl Employee {
        fn add_salaries(&mut self, amount: f64) {
            self.salary += amount;
        }
        fn sub_salaries(&mut self, amount: f64) {
            self.salary -= amount;
        }
    }

pub fn employee_struct_class(){   
    // Create instances of the Employee struct for both 
    // enployee types
    let mut employee1 = Employee {
        name: String::from("Daniel Mbazu"),
        salary: 12000.0,
        id: 1,
        employee_type: EmployeeType::ProductManager,
    };

    let mut employee2 = Employee {
        name: String::from("The Rock"),
        salary: 15000.0,
        id: 2,
        employee_type: EmployeeType::BlockchainDeveloper,
    };
    let mut employee3 = Employee {
        name: String::from("Michael Jordan"),
        salary: 9000.0,
        id: 3,
        employee_type: EmployeeType::DataAnalyst,
    };
    let mut employee4 = Employee {
        name: String::from("Rust Anchor"),
        salary: 23000.0,
        id: 4,
        employee_type: EmployeeType::FullStackDeveloper,
    };

    // Add salary to the employee 1 salary
    employee1.add_salaries(2000.0);

    // Subtract salary from the employee 2
    employee2.sub_salaries(1800.0);

    // Add salary to the employee 1 salary
    employee1.add_salaries(1700.0);

    // Subtract salary from the employee 2
    employee2.sub_salaries(5000.0);

    // Print the updated employee information
    
    println!("=============================================");  
    println!("Employee 1:");
    println!("Name: {}", employee1.name);
    println!("Salary: ${:.2}", employee1.salary);
    println!("ID: {}", employee1.id);
    println!("Employee Type: {:?}", employee1.employee_type);
    println!("=============================================");    
    println!("Employee 2:");
    println!("Name: {}", employee2.name);
    println!("Salary: ${:.2}", employee2.salary);
    println!("ID: {}", employee2.id);
    println!("Employee Type: {:?}", employee2.employee_type);    
    println!("=============================================");  
    println!("=============================================");  
    println!("Employee 1:");
    println!("Name: {}", employee3.name);
    println!("Salary: ${:.2}", employee3.salary);
    println!("ID: {}", employee3.id);
    println!("Employee Type: {:?}", employee3.employee_type);
    println!("=============================================");    
    println!("Employee 2:");
    println!("Name: {}", employee4.name);
    println!("Salary: ${:.2}", employee4.salary);
    println!("ID: {}", employee4.id);
    println!("Employee Type: {:?}", employee4.employee_type);    
    println!("=============================================");  
}