
// Record Struct representing a Single Row
// This struct represents a single record in the query result. 
// It contains fields for id, name, department, salary, and active status.
#[derive(Debug, Clone)]
struct Record {
    id: u32,
    name: String,
    department: String,
    salary: u64,
    active: bool,
}

// Enum for possible parsing errors
#[derive(Debug)]
enum ParseError {
    MissingFields(usize), // not enough fields in the line
    MissingField(String), // a required field is missing
    InvalidNumber(String), // a field that should be a number is not
    InvalidBool(String), // a field that should be a boolean is not
}

// Implementing std::fmt::Display for ParseError 
// to provide user-friendly error messages
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingFields(count) => write!(f, "Expected 5 fields but found {}", count),
            ParseError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ParseError::InvalidNumber(field) => write!(f, "Invalid number in field: {}", field),
            ParseError::InvalidBool(field) => write!(f, "Invalid boolean in field: {}", field),
        }
    }
}

// parse_record: Function to parse a single CSV line into a Record.
fn parse_record(line: &str) -> Result<Record, ParseError> {
    // Split the line by commas and trim whitespace
    let fields: Vec<&str> = line.split(',')
        .map(|s| s.trim())
        .collect();
    
    // Check if we have the correct number of fields
    if fields.len() != 5 {
        return Err(ParseError::MissingFields(fields.len()));
    }

    // Parse each field and handle potential errors
    let id = fields[0].parse::<u32>()
        .map_err(|_| ParseError::InvalidNumber("id".to_string()))?;
    let name = fields[1].to_string();
    let department = fields[2].to_string();
    let salary = fields[3].parse::<u64>()
        .map_err(|_| ParseError::InvalidNumber("salary".to_string()))?;
    let active = fields[4].parse::<bool>()
        .map_err(|_| ParseError::InvalidBool("active".to_string()))?;

    Ok(Record {
        id,
        name,
        department,
        salary,
        active,
    })
}

// parse_csv: Function to parse all rows, skipping the header. 
// Returns one Result per data row
fn parse_csv(data: &str) -> Vec<Result<Record, ParseError>> {
    data.lines()
        .skip(1) // Skip the header line
        .map(|line| parse_record(line))
        .collect()
}

// Filter active employees
fn filter_active<'a>(records: &[&'a Record]) -> Vec<&'a Record> {
    // Filter the records to include only active employees i.e
    // record.active is true and collect the results into a new vector
    records.iter()
        .copied()
        .filter(|record| record.active)
        .collect()
}

// Filter employees by department
fn filter_by_department<'a>(records: &'a [Record], department: &str) -> Vec<&'a Record> {
    // Filter the records to include only those in the specified department
    // and collect the results into a new vector where the lifetime 
    // of the references is tied to the input records slice
    records.iter()
        .filter(|record| record.department == department)
        .collect()
}

// Calculate average salary
fn average_salary(records: &[&Record]) -> f64 {
    // Calculate the total salary by iterating over the records,
    let total_salary: u64 = records.iter()
        .map(|record| record.salary)
        .sum();
    
    // Count the number of records to avoid division by zero
    let count = records.len() as f64;

    if count == 0.0 {
        0.0 // Return 0 if there are no records to avoid division by zero
    } else {
        total_salary as f64 / count // Return the average salary  
    }
}

// Calculate total salary 
fn total_salary(records: &[&Record]) -> u64 {
    // Calculate the total salary by iterating over 
    // the records and summing their salaries
    records.iter()
        .map(|record| record.salary)
        .sum()
}

// Formatted output of records
fn print_report(department: &str, records: &[Record]) {
    // Print the title of the report
    println!("=== Active {} Employees ===", department);

    let by_department: Vec<&Record> = filter_by_department(records, department);
    let active_by_department: Vec<&Record> = filter_active(&by_department);

    // Header for the report
    println!("{:<30} {:<20} {:<10}", "Name", "Department", "Salary");
    println!("{:-<60}", ""); // Separator line

    // Print each record in a formatted manner
    for record in &active_by_department {
        println!(
            "{:<30} {:<20} {:<10}",
            record.name, record.department, record.salary
        );
    }

    println!("{:-<60}", ""); // Separator line

    // Print summary statistics
    println!("Count: {}", active_by_department.len());
    println!("Total: {}", total_salary(&active_by_department));
    println!("Average: {:.2}", average_salary(&active_by_department));
}

fn main() {
    // Sample CSV data
    let csv_data = "id,name,department,salary,active
1,John Doe,Engineering,75000,true
2,Jane Smith,Marketing,65000,false
3,Bob Johnson,Engineering,80000,true
4,Alice Williams,HR,60000,true
5,Charlie Brown,Marketing,55000,false
6,Emily Davis,Engineering,72000,true
7,Michael Wilson,HR,58000,false
8,Sarah Miller,Marketing,62000,true
9,David Lee,Engineering,78000,true
10,Laura Garcia,HR,61000,true";

    // Parse the CSV data into records
    let parsed_records = parse_csv(csv_data);

    // Collect valid records and print errors for invalid ones
    let mut valid_records = Vec::new();
    for result in parsed_records {
        match result {
            Ok(record) => valid_records.push(record),
            Err(e) => eprintln!("Error parsing record: {}", e),
        }
    }

    // Print the report for the Engineering department
    print_report("Engineering", &valid_records);

    // Print the report for the Marketing department
    print_report("Marketing", &valid_records);

    // Print the report for the HR department
    print_report("HR", &valid_records);

}