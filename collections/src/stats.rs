use std::collections::HashMap;

pub struct Stats {
  pub mean: f32,
  pub median: i32,
  pub mode: i32,
}

pub fn get_stats(numbers: &Vec<i32>) -> Option<Stats> {
  if numbers.len() == 0 {
    return None;
  }

  let mut total: i32 = 0;

  let mut frequencies = HashMap::<i32, u32>::new();
  let mut max_frequency: u32 = 0;

  let mut sorted_numbers = numbers.clone();
  sorted_numbers.sort_unstable();
  let mut mode = 0;

  for number in numbers {
    total += *number;
    let frequency = frequencies.entry(*number).or_insert(0);
    *frequency += 1;
    if *frequency > max_frequency {
      max_frequency = *frequency;
      mode = *number;
    }  }

  let stats = Stats {
    mean: (total as f32) / (numbers.len() as f32),
    median: sorted_numbers[sorted_numbers.len() / 2],
    mode: mode,
  };
  return Some(stats);
}
