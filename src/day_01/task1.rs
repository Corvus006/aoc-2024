use crate::day_01::read_file;

pub fn task1(input: String) -> i32 {
    let lists = match read_file(input) {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return 0;  // Return a default value in case of error
        }
    };

    let mut list1 = lists.list1;
    let mut list2 = lists.list2;

    // Sort the lists
    list1.sort();
    list2.sort();

    // Calculate the differences and their sum
    let mut dif = Vec::new();
    let mut solution: i32 = 0;

    // Calculate the absolute differences between the lists
    for i in 0..list1.len() {
        dif.push((list1[i] - list2[i]).abs());
    }

    // Sum up the differences
    for diff in &dif {
        solution += diff;
    }

    solution  // Return the final sum
}