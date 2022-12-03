use std::fs;
use std::str::Split;

fn main() {
    let per_elf = split_per_elf(r"C:\Users\jahug\git\advent-of-code\aoc1i\input.txt");
    let mut sum_per_elf = Vec::new();
    for elf in per_elf {
        sum_per_elf.push(sum_elf(&elf));
    }
    let arr_max_and_index = find_max(&sum_per_elf);
    sum_per_elf = remove_max_elf(sum_per_elf, arr_max_and_index[1]);
    println!("highest value {}", arr_max_and_index[0]);
    println!("highest value index {}", arr_max_and_index[1]);
    let arr_second_and_index = find_max(&sum_per_elf);
    sum_per_elf.remove(arr_second_and_index[1].try_into().unwrap());
    println!("highest value {}", arr_second_and_index[0]);
    println!("highest value index {}", arr_second_and_index[1]);
    let arr_third_and_index = find_max(&sum_per_elf);
    sum_per_elf.remove(arr_third_and_index[1].try_into().unwrap());
    println!("highest value {}", arr_third_and_index[0]);
    println!("highest value index {}", arr_third_and_index[1]);
    let total = arr_third_and_index[0] + arr_second_and_index[0] + arr_max_and_index[0];
    println!("total {}", total);
}

fn split_per_elf(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let elves: Split<&str> = contents.split("\r\n\r\n");
    let mut per_elf = Vec::new();
    for elf in elves {
        per_elf.push(elf.to_string());
    }
    return per_elf;
}


fn sum_elf(elf: &str) -> i32 {
    let meals: Split<&str> = elf.split("\r\n");
    let mut sum = 0;
    for meal in meals {
        let meal_calories: i32 = meal.parse().unwrap_or(0);
        sum += meal_calories 
    }
    return sum;
}

fn find_max(sum_per_elf: &Vec<i32>) -> [i32;2] {
    let mut highest_index = 0;
    let mut i = 0;
    let mut highest = 0;
    for elf in sum_per_elf{
        if elf > &highest {
            highest = *elf;
            highest_index = i;
        }
        i += 1;
    }
    return [highest, highest_index];
}
fn remove_max_elf(mut sum_per_elf: Vec<i32>, index: i32) -> Vec<i32> {
    sum_per_elf.remove(index.try_into().unwrap());
    return sum_per_elf;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_elves() {
        let result = split_per_elf(r"C:\Users\jahug\git\advent-of-code\aoc1i\sample.txt").len();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_sum_elf() {
        let elves = split_per_elf(r"C:\Users\jahug\git\advent-of-code\aoc1i\sample.txt");
        let elves_slice: &[String] = &elves[..];
        let elves_slice_ref: [&str; 6] = [elves_slice[0].as_str(), elves_slice[1].as_str(), elves_slice[2].as_str(), elves_slice[3].as_str(), elves_slice[4].as_str(), elves_slice[5].as_str()];
        let mut result: [i32;6] = [0,1,2,3,4,5];
        for i in 0..=5 {
            result[i] = sum_elf(elves_slice_ref[i]);
        }
        assert_eq!(result, [53308, 26716, 33867, 67712, 56252, 46484]);
    }
}
