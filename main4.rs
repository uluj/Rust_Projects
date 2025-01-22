// src/main.rs
struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    // Method to check if an item matches the condition
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

fn custom_filter<T: PartialEq>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone,
{
    let mut filtered = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            filtered.push(item.clone());
        }
    }
    filtered
}

fn main() {
    // Step 6: Create a collection (vector) and a filter condition
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let condition = FilterCondition { value: 3 };

    // Step 7: Call the custom_filter function and store the result
    let filtered_numbers = custom_filter(&numbers, &condition);

    // Step 8: Print the filtered result
    println!("Filtered numbers: {:?}", filtered_numbers);
}
