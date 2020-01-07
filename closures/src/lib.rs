mod cacher;

use std::thread;
use std::time::Duration;
use cacher::Cacher;

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut simulated_expensive_calculation = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation.value(intensity)
            );
        }
    }
}
