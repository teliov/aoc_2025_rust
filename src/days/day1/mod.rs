fn parse_file_content(file_contents: &String) -> (Vec<&str>, Vec<&str>) {
    let mut list1: Vec<&str> = Vec::new();
    let mut list2: Vec<&str> = Vec::new();

    for line in file_contents.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 {
            continue;
        }
        let mut parts = trimmed_line.split_ascii_whitespace();
        // for loop to iterate over parts, assume expected size is 2
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();

        list1.push(first);
        list2.push(second);
    }

    // sort list1 and list2
    list1.sort_unstable();
    list2.sort_unstable();

    (list1, list2)
}

pub fn solve_part_one(file_contents: &String) -> String {
    let (list1, list2) = parse_file_content(file_contents);

    let mut running_sum = 0;

    let list_length = list1.len();
    for i in 0..list_length {
        let first = list1[i];
        let second = list2[i];

        // cast first and second to integers
        let first_int = first.parse::<i32>().unwrap();
        let second_int = second.parse::<i32>().unwrap();

        // determine the absolute difference between the two
        let diff = (first_int - second_int).abs();
        running_sum += diff;
    }

    running_sum.to_string()
}

pub fn solve_part_two(file_contents: &String) -> String {
    let (list1, list2) = parse_file_content(file_contents);

    let mut running_sum: i32 = 0;
    // iterate over items in list1
    let mut current_item: i32 = -1;
    let mut current_item_score: i32 = 0;

    let mut second_list_idx: usize = 0;
    let second_list_length = list2.len();

    for first in list1.iter() {
        // cast first to integer
        let first_int = first.parse::<i32>().unwrap();
        if current_item == first_int {
            running_sum += current_item_score;
            continue;
        }

        current_item = first_int;

        let mut match_count: i32 = 0;
        while second_list_idx < second_list_length {
            let second = list2[second_list_idx];
            let second_int = second.parse::<i32>().unwrap();

            if current_item == second_int {
                match_count += 1;
                second_list_idx += 1;
            } else if current_item < second_int {
                break;
            } else  {
                second_list_idx += 1;
            }
        }

        current_item_score = match_count * first_int;
        running_sum += current_item_score;
    }

    running_sum.to_string()
}