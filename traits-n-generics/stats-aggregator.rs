/*
    Generic Stat Aggregator
    Build a small statistics aggregation system that exercises generics, 
    trait definitions, trait bounds, default implementations, and trait objects.

    Part 1: Define Traits and Structs
    Part 2: Implement Traits for Structs
    Part 3: Create a Generic Aggregator and functions
    Part 4: top_n function to get top N items by value
    Part 5: Trait Objects and Dynamic Dispatch
    Part 6: Display Trait for better reporting
    Part 7: Return Summarize trait object from a function

    Concepts practiced: traits, generics, trait bounds, default methods, trait objects, dynamic dispatch
*/

trait Summarize {
    fn summary(&self) -> String;

    // Default implementation for label method
    fn label(&self) -> &str {
        "item"
    }
}

trait Numeric {
    fn value(&self) -> f64;
}

struct TemperatureReading {
    celsius: f64,
    city: String,
}

struct StockPrice {
    ticker: String,
    price: f64,
}

struct HeartRate {
    bpm: u32,
    subject: String,
}

impl Summarize for TemperatureReading {
    fn summary(&self) -> String {
        format!("{}: {}°C", self.label(), self.celsius)
    }

    fn label(&self) -> &str {
        &self.city
    }
}

impl Summarize for StockPrice {
    fn summary(&self) -> String {
        format!("{}: ${}", self.label(), self.price)
    }

    fn label(&self) -> &str {
        &self.ticker
    }
}

impl Summarize for HeartRate {
    fn summary(&self) -> String {
        format!("{}: {} bpm", self.label(), self.bpm)
    }

    fn label(&self) -> &str {
        &self.subject
    }
}

impl Numeric for TemperatureReading {
    fn value(&self) -> f64 {
        self.celsius
    }
}

impl Numeric for StockPrice {
    fn value(&self) -> f64 {
        self.price
    }
}

impl Numeric for HeartRate {
    fn value(&self) -> f64 {
        self.bpm as f64
    }
}

impl std::fmt::Display for TemperatureReading {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TemperatureReading({}, {}°C)", self.city, self.celsius)
    }
}

impl std::fmt::Display for StockPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StockPrice({}, ${:.2})", self.ticker, self.price)
    }
}

impl std::fmt::Display for HeartRate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HeartRate({}, {} bpm)", self.subject, self.bpm)
    }
}

struct Aggregator<T> {
    items: Vec<T>,
}

impl<T: Summarize + Numeric> Aggregator<T> {
    fn new() -> Self {
        Aggregator { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn average(&self) -> f64 {
        let sum: f64 = self.items.iter().map(|item| item.value()).sum();
        if sum == 0.0 {
            return 0.0; // Avoid division by zero
        }
        sum / self.items.len() as f64
    }

    fn report(&self) -> String
    where
        T: Summarize + Numeric + std::fmt::Display,
    {
        // format calls Display implementation for each item
        self.items.iter().map(|item| item.summary()).collect::<Vec<String>>().join("\n")
    }

    fn top_n(&self, n: usize) -> Vec<&T> {
        let mut sorted_items = self.items.iter().collect::<Vec<&T>>();
        sorted_items.sort_by(|a, b| b.value().partial_cmp(&a.value()).unwrap());
        sorted_items.into_iter().take(n).collect()
    }
}

// Function that returns a trait object implementing Summarize
// Why Box? Because we can't return a trait directly, we need to return a pointer to it. 
// Box is a smart pointer that allocates on the heap, allowing us to return a trait object.
fn make_item(hot: bool) -> Box<dyn Summarize> {
    if hot {
        Box::new(TemperatureReading { celsius: 42.0, city: String::from("Phoenix") })
    } else {
        Box::new(StockPrice { ticker: String::from("AAPL"), price: 150.0 })
    }
}

fn print_all(items: &[&dyn Summarize]) {
    for item in items {
        println!("{}", item.summary());
    }
}

fn main() {
    let mut temp_aggregator = Aggregator::new();
    temp_aggregator.add(TemperatureReading { celsius: 22.5, city: String::from("New York") });
    temp_aggregator.add(TemperatureReading { celsius: 18.0, city: String::from("London") });

    println!("Temperature Report:\n{}", temp_aggregator.report());
    println!("Average Temperature: {:.2}°C", temp_aggregator.average());
    println!("Top Temperature: {}", temp_aggregator.top_n(1)[0].summary());
    println!("-----------------------------");

    let mut stock_aggregator = Aggregator::new();
    stock_aggregator.add(StockPrice { ticker: String::from("AAPL"), price: 150.0 });
    stock_aggregator.add(StockPrice { ticker: String::from("GOOGL"), price: 2800.0 });

    println!("Stock Report:\n{}", stock_aggregator.report());
    println!("Average Stock Price: {:.2}", stock_aggregator.average());
    println!("Top Stock Price: {}", stock_aggregator.top_n(1)[0].summary());
    println!("-----------------------------");

    let mut heart_rate_aggregator = Aggregator::new();
    heart_rate_aggregator.add(HeartRate { bpm: 72, subject: String::from("Alice") });
    heart_rate_aggregator.add(HeartRate { bpm: 68, subject: String::from("Bob") });

    println!("Heart Rate Report:\n{}", heart_rate_aggregator.report());
    println!("Average Heart Rate: {:.2} bpm", heart_rate_aggregator.average());
    println!("Top Heart Rate: {}", heart_rate_aggregator.top_n(1)[0].summary());
    println!("-----------------------------");

    let temperature_binding = TemperatureReading { celsius: 25.0, city: String::from("Tokyo") };
    let stock_binding = StockPrice { ticker: String::from("MSFT"), price: 300.0 };
    let heart_rate_binding = HeartRate { bpm: 80, subject: String::from("Charlie") };
    let all_items: Vec<&dyn Summarize> = vec![
        &temperature_binding,
        &stock_binding,
        &heart_rate_binding,
    ];
    println!("All Items:");
    print_all(&all_items);
}