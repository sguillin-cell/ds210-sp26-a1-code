extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::{Query, Condition, Aggregation};
use analytics_lib::dataset::Value;
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let sections: Vec<&str> = input.split("GROUP BY").collect(); // All queries follow the same form so split at the GROUP
    let filter_part = sections[0].trim().strip_prefix("FILTER").unwrap().trim().trim_matches(|c| c == '(' || c == ')'); 
    let group_agg_part = sections[1].trim(); // Removes extra spaces
    let parts: Vec<&str> = group_agg_part.split_whitespace().collect(); // Splits the right part of queyr
    let group_by = parts[0].to_string(); //Convert to a string since the query type needs a string
    let aggregation = match parts[1] {
        "COUNT" => Aggregation::Count(parts[2].to_string()), //If string is count use count func
        "SUM" => Aggregation::Sum(parts[2].to_string()), //If string is sum use sum func
        "AVERAGE" => Aggregation::Average(parts[2].to_string()), //else use average function
        _ => panic!("Unknown aggregation"),
    };

    let filter = parse_condition(filter_part);

    Query::new(filter, group_by, aggregation)
}

fn parse_condition(cond_str: &str) -> Condition {
    let cond_str = cond_str.trim().trim_matches(|c| c == '(' || c == ')');

        // NOT
    if cond_str.starts_with('!') {
            let inner = cond_str.trim_start_matches('!').trim();
            return Condition::Not(Box::new(parse_condition(inner)));
        }

        // OR
        if cond_str.contains("OR") {
            let parts: Vec<&str> = cond_str.split("OR").collect();
            let mut condition = parse_condition(parts[0]);
            for part in parts.iter().skip(1) {
                condition = Condition::Or(Box::new(condition), Box::new(parse_condition(part)));
            }
            return condition;
        }

        // AND
        if cond_str.contains("AND") {
            let parts: Vec<&str> = cond_str.split("AND").collect();
            let mut condition = parse_condition(parts[0]);
            for part in parts.iter().skip(1) {
                condition = Condition::And(Box::new(condition), Box::new(parse_condition(part)));
            }
            return condition;
        }

        // EQUAL
        let parts: Vec<&str> = cond_str.split("==").collect();
        let field = parts[0].trim().to_string();
        let value = parts[1].trim().replace("\"", "");
        Condition::Equal(field, Value::String(value))
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