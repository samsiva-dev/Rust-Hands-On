# Project — Mini Query Result Processor

> **Modules covered:** 1 through 7  
> **Difficulty:** Intermediate  
> **Goal:** Build a CSV query processor that mirrors PostgreSQL's executor pipeline — parsing tuples, filtering rows, projecting columns, and aggregating results — using idiomatic Rust.

---

## The Data

Work with this CSV input — employees at a fictional company:

```
id,name,department,salary,active
1,Alice,Engineering,95000,true
2,Bob,Marketing,72000,true
3,Charlie,Engineering,88000,false
4,Diana,HR,65000,true
5,Eve,Engineering,102000,true
6,Frank,Marketing,69000,false
7,Grace,HR,71000,true
8,Hank,Engineering,91000,true
```

Hardcode this string in your `main.rs` — no file I/O needed.

---

## Data Model

Define a `Record` struct with these fields and types:

| Field        | Type     |
|--------------|----------|
| `id`         | `u32`    |
| `name`       | `String` |
| `department` | `String` |
| `salary`     | `u64`    |
| `active`     | `bool`   |

`Record` must derive `Debug` and `Clone`.

---

## Error Type

Define a `ParseError` enum with at least these variants:

| Variant                  | When to use                          |
|--------------------------|--------------------------------------|
| `MissingField(String)`   | A required column is absent          |
| `InvalidNumber(String)`  | A field that should be numeric isn't |
| `InvalidBool(String)`    | A field that should be bool isn't    |

`ParseError` must implement `std::fmt::Display`.

---

## Functions to Implement

### Parsing

```rust
fn parse_record(line: &str) -> Result<Record, ParseError>
```
Parse a single CSV line into a `Record`. Return appropriate `ParseError` variants on failure. Use `?` for error propagation.

```rust
fn parse_csv(data: &str) -> Vec<Result<Record, ParseError>>
```
Parse all rows, skipping the header line. Return one `Result` per data row.

---

### Querying

```rust
fn filter_active(records: &[Record]) -> Vec<&Record>
```
Return references to all records where `active == true`.

```rust
fn filter_by_department<'a>(records: &'a [Record], dept: &str) -> Vec<&'a Record>
```
Return references to all records in the given department.

```rust
fn average_salary(records: &[&Record]) -> f64
```
Return the average salary. Return `0.0` for an empty slice.

```rust
fn total_salary(records: &[&Record]) -> u64
```
Return the sum of all salaries.

---

### Reporting

```rust
fn print_report(title: &str, records: &[&Record])
```
Print a formatted table with `name`, `department`, and `salary` columns, followed by summary stats.

---

## Expected Output Shape

```
=== Active Engineering Employees ===
Name             Department      Salary
────────────────────────────────────────
Alice            Engineering     95000
Eve              Engineering     102000
Hank             Engineering     91000
────────────────────────────────────────
Count:   3
Total:   288000
Average: 96000.00
```

---

## Constraints

These are non-negotiable — treat them as compiler rules:

- No `unwrap()` in production code paths — use `?` or proper `match`
- Every parse error must be reported, not silently skipped
- Use iterator methods — no manual index loops for filtering or mapping
- `Record` must derive `Debug` and `Clone`
- `ParseError` must implement `Display`

---

## Hints

<details>
<summary>Hint 1 — parsing a single field</summary>

```rust
let fields: Vec<&str> = line.split(',').collect();

let id = fields
    .get(0)
    .ok_or_else(|| ParseError::MissingField("id".to_string()))?
    .trim()
    .parse::<u32>()
    .map_err(|_| ParseError::InvalidNumber("id".to_string()))?;
```

</details>

<details>
<summary>Hint 2 — collecting only Ok records</summary>

```rust
let records: Vec<Record> = parse_csv(data)
    .into_iter()
    .filter_map(|r| match r {
        Ok(record) => Some(record),
        Err(e)     => { eprintln!("Parse error: {}", e); None }
    })
    .collect();
```

</details>

<details>
<summary>Hint 3 — average salary</summary>

```rust
fn average_salary(records: &[&Record]) -> f64 {
    if records.is_empty() { return 0.0; }
    let total: u64 = records.iter().map(|r| r.salary).sum();
    total as f64 / records.len() as f64
}
```

</details>

---

## Module Contribution Map

| Module | Where it appears in this project |
|--------|----------------------------------|
| 1 — Basics | Types, functions, control flow throughout |
| 2 — Ownership | `parse_csv` returns owned `Vec<Record>`, moves through pipeline |
| 3 — Borrowing | All query functions take `&[Record]` or `&[&Record]` |
| 4 — Structs / Enums | `Record` struct, `ParseError` enum, pattern matching in report |
| 5 — Traits | `Display` for `ParseError`, `derive(Debug, Clone)` |
| 6 — Error handling | `ParseError`, `Result`, `?` in `parse_record` |
| 7 — Collections / Iterators | `filter_map`, `map`, `sum`, `collect` throughout |

---

## Suggested Order of Attack

1. Define `Record` and `ParseError` — get the types right first
2. Implement `parse_record` — get one row working with full error handling
3. Implement `parse_csv` — apply it to all rows
4. Implement the four query functions — each is small and focused
5. Implement `print_report` — formatting last
6. Wire everything together in `main`

---

*Post your solution for a full code review across correctness, idiomatic Rust, ownership discipline, error handling, and iterator usage.*
