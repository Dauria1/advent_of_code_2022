use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to open file");
    let data: Vec<Vec<i32>> = parse_calories(&input);
    get_top_elf(&data);
    get_top_three_elves(&data);
}

fn get_top_elf(data: &Vec<Vec<i32>>) -> (usize, i32) {
    let (index, sum) = get_elf_with_highest_calories(&data);
    println!("Elf {} has the highest calories with {}", index + 1, sum);
    return (index, sum);
}

fn get_top_three_elves(data: &Vec<Vec<i32>>) -> (Vec<(usize, i32)>, i32) {
    let elves = get_top_three_elves_with_highest_calories(&data);
    let three_elves_calories: Vec<i32> = elves.iter().map(|(_, calories)| *calories).collect();
    let total_calories = three_elves_calories.iter().sum::<i32>();
    println!(
        "Elves {:?} are top 3: Their total calorie sum is: {}",
        elves, total_calories
    );
    return (elves, total_calories);
}

fn parse_calories(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .split("\n\n")
        .map(|elves_calories| {
            elves_calories
                .lines()
                .map(|calories| calories.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_elves_calories_and_index(data: &[Vec<i32>]) -> Vec<(usize, i32)> {
    data.iter()
        .enumerate()
        .map(|(i, x)| (i, x.iter().sum()))
        .collect()
}

fn get_elf_with_highest_calories(data: &[Vec<i32>]) -> (usize, i32) {
    let elves_calories = get_elves_calories_and_index(data);
    elves_calories
        .iter()
        .max_by_key(|&(_, sum)| sum)
        .unwrap()
        .clone()
}

fn get_top_three_elves_with_highest_calories(data: &[Vec<i32>]) -> Vec<(usize, i32)> {
    let mut elves_with_calories = get_elves_calories_and_index(data);
    elves_with_calories.sort_by_key(|&(i, sum)| (sum, i));
    elves_with_calories.into_iter().rev().take(3).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elves_calories_and_index() {
        let data = vec![vec![10, 20], vec![30], vec![40, 50, 60]];
        let expected_output = vec![(0, 30), (1, 30), (2, 150)];
        assert_eq!(get_elves_calories_and_index(&data), expected_output);
    }

    #[test]
    fn test_get_top_three_elves_with_highest_calories() {
        let data = vec![
            vec![50, 65],
            vec![70],
            vec![80, 90, 100],
            vec![110],
            vec![120, 130],
        ];
        let expected_output = vec![(2, 270), (4, 250), (0, 115)];
        assert_eq!(
            get_top_three_elves_with_highest_calories(&data),
            expected_output
        );
    }

    #[test]
    fn test_get_top_elf() {
        let data = vec![vec![50, 60], vec![70], vec![80, 90, 100]];
        assert_eq!(get_top_elf(&data), (2, 270));
    }

    #[test]
    fn test_get_top_three_elves() {
        let data = vec![
            vec![50, 65],
            vec![70],
            vec![80, 90, 100],
            vec![110],
            vec![120, 130],
        ];
        let expected_output = vec![(2, 270), (4, 250), (0, 115)];
        let expected_total_calories = 635;
        let (elves, total_calories) = get_top_three_elves(&data);
        assert_eq!(elves, expected_output);
        assert_eq!(total_calories, expected_total_calories);
    }
}
