mod departments;
mod pig_latin;
mod stats;

fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 4, 5, 5, 5];
    let stats = stats::get_stats(&numbers).unwrap();

    println!(
        "Mean: {}, median: {}, mode: {}",
        stats.mean, stats.median, stats.mode
    );

    println!("{}", pig_latin::pig_latin("hello world, こんにちは世界"));

    departments::start();
}
