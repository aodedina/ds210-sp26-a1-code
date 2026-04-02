extern crate tarpc;
use analytics_lib::query::{Query, Condition, Aggregation};
use analytics_lib::dataset::Value;
use std::time::Instant;
use std::io::BufRead;

use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    //splits input string into parts using whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();

    // fliter A1 section, gorup by grade and count name 
    let filter_column = parts[1].to_string();

    //trims quotes from string and filter
    let filter_value = parts[3]
        .trim_matches('"')
        .to_string();

    //column group by
    let group_by = parts[6].to_string();
    //type of aggregation like count or sum and then column aggregation
    let aggregation_type = parts[7];
    let aggregation_column = parts[8].to_string();

    let filter = Condition::Equal(
        filter_column,
        Value::String(filter_value),
    );
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