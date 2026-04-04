extern crate tarpc;
use analytics_lib::query::{Query, Condition, Aggregation};
use analytics_lib::dataset::Value;
use std::time::Instant;
use std::io::BufRead;

use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    //split into main sections
    let filter_start = input.find("FILTER").unwrap();
    let group_by_start = input.find("GROUP BY").unwrap();

    let filter_str = input[filter_start + "FILTER ".len()..group_by_start].trim();
    
    //splits input string into parts using whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // fliter A1 section, gorup by grade and count name 
    let filter_column = parts[0]
        .trim_matches('(')
        .trim_matches(')')
        .to_string();

    //trims quotes from string and filter
    let filter_value = parts[2]
        .trim_matches('"')
        .trim_matches(')')
        .to_string();
    

    // column group by
    let group_by = parts[parts.len() - 3].to_string();

    // aggregation
    let aggregation_type = parts[parts.len() - 2];
    let aggregation_column = parts[parts.len() - 1].to_string();

    let filter = if filter_str.starts_with('(') {
    // remove parentheses
    let inner = &filter_str[1..filter_str.len() - 1];

    // split on OR
    let or_parts: Vec<&str> = inner.split("OR").map(|s| s.trim()).collect();

    let mut conditions = Vec::new();

    for part in or_parts {
        let parts: Vec<&str> = part.split_whitespace().collect();

        let filter_column = parts[0].to_string();
        let filter_value = parts[2].trim_matches('"').to_string();

        conditions.push(Condition::Equal(
            filter_column,
            Value::String(filter_value),
        ));
    }

    // combine into OR chain
    let mut cond = conditions[0].clone();
    for c in conditions.into_iter().skip(1) {
        cond = Condition::Or(Box::new(cond), Box::new(c));
    }

    cond
} else {
    // simple case (your original logic)
    let parts: Vec<&str> = filter_str.split_whitespace().collect();

    let filter_column = parts[0].to_string();
    let filter_value = parts[2].trim_matches('"').to_string();

    Condition::Equal(
        filter_column,
        Value::String(filter_value),
    )
};
    //determines aggregation type and creates aggregation object
    let aggregation = match aggregation_type {
        "COUNT" => Aggregation::Count(aggregation_column),
        "SUM" => Aggregation::Sum(aggregation_column),
        "AVERAGE" => Aggregation::Average(aggregation_column),
        _ => panic!("Invalid aggregation"),
    };
    //returns query object using elements below
    Query::new(filter, group_by, aggregation)
}



// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}