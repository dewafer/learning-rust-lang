use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g., 'Add Sally to Engineering' or 'List all' or 'List Engineering'):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.starts_with("Add ") {
            let parts: Vec<&str> = input.splitn(4, ' ').collect();
            if parts.len() == 4 && parts[2].eq_ignore_ascii_case("to") {
                let name = parts[1].to_string();
                let department = parts[3].to_string();

                company.entry(department.clone()).or_insert_with(Vec::new).push(name);
                println!("Added to {} department.", department);
            } else {
                println!("Invalid command format. Use 'Add <Name> to <Department>'.");
            }
        } else if input.eq_ignore_ascii_case("List all") {
            let mut all_departments: Vec<_> = company.iter().collect();
            all_departments.sort_by_key(|&(dept, _)| dept);

            for (department, employees) in all_departments {
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                println!("{}: {:?}", department, sorted_employees);
            }
        } else if input.starts_with("List ") {
            let department = input[5..].trim();
            if let Some(employees) = company.get(department) {
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                println!("{}: {:?}", department, sorted_employees);
            } else {
                println!("No such department found.");
            }
        } else {
            println!("Invalid command. Use 'Add <Name> to <Department>', 'List all', or 'List <Department>'.");
        }
    }
}
