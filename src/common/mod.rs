// Common utility functions
pub fn print_status(operation_type : &str, packages : &Vec<&str>) {
    println!("\n{} {} package{}: {:?}\n",
             operation_type,
             packages.len(),
             if packages.len() == 1 { "" } else { "s" },
             packages );
}
