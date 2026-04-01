use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


    pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
        let mut result = Dataset::new(dataset.columns().clone()); //Create a new dataset and we decided to use a clone in case people want to reutilize the virtual view
        for row in dataset.iter() {
            if check_condition(row, dataset, filter) {
                result.add_row(row.clone());
            }
        }
        return result
    }
    pub fn check_condition(row: &Row, dataset: &Dataset, cond: &Condition) -> bool {
    match cond {
        Condition::Equal(column_name, value) => {
            let idx = dataset.column_index(column_name);
            row.get_value(idx) == value
        }
        Condition::Not(inner) => !check_condition(row, dataset, inner),
        Condition::And(left, right) => {
            check_condition(row, dataset, left) && check_condition(row, dataset, right)
        }
        Condition::Or(left, right) => {
            check_condition(row, dataset, left) || check_condition(row, dataset, right)
        }
    }
}

    pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
        let mut map: HashMap<Value, Dataset> = HashMap::new();
        let col_index = dataset.column_index(group_by_column);
        let columns = dataset.columns().clone();
        for row in dataset.into_iter() {
            let key = row.get_value(col_index).clone();
            if !map.contains_key(&key) {
                map.insert(key.clone(), Dataset::new(columns.clone()));
                }
            match map.get_mut(&key) {
                Some(group_dataset) => {
                group_dataset.add_row(row);
                }
                None => {
                panic!("Not found in map: {:?}", key);
            }
        }
    }
    return map
}

fn sum_column(dataset: &Dataset, column_name: &String) -> i32 {
    let i = dataset.column_index(column_name);
    let mut sum = 0;
    for row in dataset.iter() {
        match row.get_value(i) {
            Value::Integer(val) => sum += val,
            _ => panic!("columns must be integers to sum sorry!"),
        }
    }

    return sum
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    for (group_val, group_dataset) in dataset {
        let agg_val = match aggregation {
            Aggregation::Count(column_name) => {
                Value::Integer(group_dataset.len() as i32)
            }
            Aggregation::Sum(column_name) => {
                Value::Integer(sum_column(&group_dataset, column_name))
            }
            Aggregation::Average(column_name) => {
                let sum = sum_column(&group_dataset, column_name);
                let count = group_dataset.len() as i32;
                let avg = if count == 0 { 0 } else { sum / count };
                Value::Integer(avg)
            }
        };
        result.insert(group_val, agg_val);
    }
    return result
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}