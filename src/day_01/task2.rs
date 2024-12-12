use crate::day_01::read_file;

pub fn task2(input: String) -> usize {
    let lists = match read_file(input) {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return 0;  // Return a default value in case of error
        }
    };
    let  list1 = lists.list1;
    let list2 = lists.list2;

    let mut similarity = Vec::new();
    let mut solution: i32 = 0;

    for i in 0..list1.len() {
        let count = list2.iter().filter(|&&x| x == list1[i]).count(); // Count occurrences of list1[i] in list2
        similarity.push(count as i32 * list1[i]); // Multiply by list1[i] and store
    }

    // Sum all the similarity values
    for i in 0..similarity.len() {
        solution += similarity[i];
    }

    solution as usize
}