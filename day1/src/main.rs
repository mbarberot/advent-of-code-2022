use std::{fs, vec};

fn main() {
    println!("Day 1");
    let file_path = "resources/input.txt";
    let content = fs::read_to_string(file_path).expect("Cannot read file");
    let lines = content.split("\n").collect();
    let most_calories = find_most_calories(&lines);

    println!("Most calories in an elf bag : {}", most_calories);

    let top_three_most_calories = find_top_three_most_calories(&lines);

    println!("Calories in top 3 elf bags : {}", top_three_most_calories);
}

fn find_most_calories(calories: &Vec<&str>) -> i32 {
    let bags = parse_input(calories);

    return bags.iter().map(calculate_sum).max().unwrap();
}

fn find_top_three_most_calories(calories: &Vec<&str>) -> i32 {
    let bags = parse_input(calories);
    let mut bag_calories: Vec<i32> = bags.iter().map(calculate_sum).collect();
    bag_calories.sort();
    bag_calories.reverse();
    let (top_three, _) = bag_calories.split_at(3);
    return top_three.iter().sum();
}

fn calculate_sum(bag: &Vec<i32>) -> i32 {
    bag.iter().fold(0, |accumulator, value| accumulator + value)
}

fn parse_input(calories: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut bags = vec![];
    let mut bag = vec![];

    for content in calories.iter() {
        match content.parse::<i32>() {
            Ok(content_as_int) => bag.push(content_as_int),
            Err(_) => {
                bags.push(bag);
                bag = vec![];
            }
        }
    }

    bags.push(bag);

    return bags;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn givent_example_on_first_part() {
        // arrange
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        // act
        let result = find_most_calories(&input);

        // assert
        assert_eq!(result, 24_000);
    }

    #[test]
    fn given_example_on_second_part() {
        // arrange
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        // act
        let result = find_top_three_most_calories(&input);

        // assert
        assert_eq!(result, 45_000);
    }

    #[test]
    fn one_elf() {
        // arrange
        let input = vec!["1000", "2000", "3000"];

        // act
        let result = find_most_calories(&input);

        // assert
        assert_eq!(result, 6_000);
    }

    #[test]
    fn parse_input_one_bag() {
        // arrange
        let input = vec!["1000", "2000", "3000"];

        // act
        let bags = parse_input(&input);

        // assert
        assert_eq!(bags, vec![vec![1_000, 2_000, 3_000]]);
    }

    #[test]
    fn parse_input_multiple_bags() {
        // arrange
        let input = vec!["1000", "", "2000", "", "3000"];

        // act
        let bags = parse_input(&input);

        // assert
        assert_eq!(bags, vec![vec![1_000], vec![2_000], vec![3_000]]);
    }
}
