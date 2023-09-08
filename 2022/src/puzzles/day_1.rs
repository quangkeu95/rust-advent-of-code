pub use crate::utils::FileUtils;

pub fn run() {
    // parse input
    let input_file_path = "src/puzzles/day_1/input.txt";

    let file_contents: Vec<String> = FileUtils::parse_text_file(input_file_path).unwrap();

    let mut elfs: Vec<u32> = vec![];
    let mut total_calories: u32 = 0;
    let mut largest_calories: u32 = 0;

    let mut content_iterator = file_contents.iter();

    while let Some(line) = content_iterator.next() {
        let line = line.trim();
        if !line.is_empty() {
            let calories = line.parse::<u32>().unwrap();
            total_calories += calories;
        } else if total_calories > 0 {
            if total_calories > largest_calories {
                largest_calories = total_calories;
            }
            elfs.push(total_calories);
            total_calories = 0;
        }
    }

    dbg!(largest_calories);

    elfs.sort();

    let len_elfs = elfs.len();
    let top_three = elfs[len_elfs - 1] + elfs[len_elfs - 2] + elfs[len_elfs - 3];
    dbg!(top_three);
}
