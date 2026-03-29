use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


//Returns a new Dataset containing only rows that satisfy the given condition   
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    //Checks if a single row meets the given condition
    fn row_satisfies_condition(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
        match condition {
            Condition::Equal(col_name, value) => {
                let col_index = dataset.columns()
                .iter()
                .position(|(name, _)| name == col_name)
                .unwrap();

                let row_value = &row.get_value(col_index);

                row_value == &value  //Checks if a column equals a specified value and keeps all columns
            }
            _ => false, 
        }
    }
    //Create a new empty dataset with the same columns to hold filtered rows
    let mut new_dataset = Dataset::new(dataset.columns().clone());

    for row in dataset.iter() {
        if row_satisfies_condition(row, filter, dataset) {
            new_dataset.add_row(row.clone());
        }
    }
    new_dataset
}




pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut groups: HashMap<Value, Dataset> = HashMap::new();
    //finds index of the column used for grouping
    let column_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    //goes through each row
    for row in dataset.rows {
        let key = row.get_value(column_index).clone();
        //checks if group already exists
        if groups.contains_key(&key) {
            let data_for_group = groups.get_mut(&key).unwrap(); //gets group and adds row
            data_for_group.add_row(row);
        } else {
            //creates new group dataset for else group
            let mut new_data = Dataset::new(columns.clone());
            new_data.add_row(row);
            //inserts into the hashmap
            groups.insert(key, new_data);
        }

    }
    return groups;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
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