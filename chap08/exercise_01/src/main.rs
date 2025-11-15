use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 3, 4, 3, 2, 1];
    let mut count = HashMap::new();

    v.sort();

    println!("Sorted vector: {:?}", v);

    println!("The median is: {}", v[v.len() / 2]);

    for i in &v {
        let counter = count.entry(i).or_insert(0);
        *counter += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;

    for (key, value) in count {
        if value > max_count {
            max_count = value;
            mode = *key;
        }
    }

    println!("The mode is: {}", mode);
}
