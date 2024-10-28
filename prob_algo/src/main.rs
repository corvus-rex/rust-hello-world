use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use plotters::prelude::*;
use rand::Rng;

fn main() {
    // Define different sizes for each complexity type
    let sizes_n = vec![10_u128.pow(3), 10_u128.pow(4), 10_u128.pow(5), 10_u128.pow(6), 10_u128.pow(7), 10_u128.pow(8)];
    let sizes_logn = vec![10_u128.pow(3), 10_u128.pow(4), 10_u128.pow(5), 10_u128.pow(6), 10_u128.pow(7), 10_u128.pow(8), 10_u128.pow(9)];
    let sizes_nlogn = vec![10_u128.pow(3), 5 * 10_u128.pow(3), 10_u128.pow(4), 5 * 10_u128.pow(4), 10_u128.pow(5)];

    let complexities = vec![
        ("n", sizes_n),
        ("logn", sizes_logn),
        ("nlogn", sizes_nlogn)
    ];

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for (complexity, sizes) in complexities {
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let mut times = vec![(0, 0.0); sizes.len()];
            for (index, &n) in sizes.iter().enumerate() {
                let array = generate_random_array(n);
                let (time_taken, _) = hire(&array, complexity);
                times[index] = (n, time_taken.as_secs_f64());
                println!("Complexity: {}, Size: {} -> Completed", complexity, n);
            }
            let mut results = results.lock().unwrap();
            results.push((complexity.to_string(), times));
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    save_results_to_file(&results.lock().unwrap(), "results.txt");
    plot_all_results(&results.lock().unwrap());
}

fn hire(array: &[i32], complexity: &str) -> (std::time::Duration, i32) {
    let mut hired = 0;
    let start = Instant::now();
    
    match complexity {
        "n" => hiring_on(array),
        "logn" => hiring_ologn(array),
        "nlogn" => hiring_onlogn(array),
        _ => panic!("Unknown complexity! Use 'n', 'logn', or 'nlogn'."),
    }
    
    (start.elapsed(), hired)
}

fn hiring_on(array: &[i32]) {
    let mut _sum = 0;
    for &val in array {
        _sum = val as i128 * 2;
    }
}

fn hiring_ologn(array: &[i32]) {
    let mut n = array.len() as i32;
    while n > 0 {
        n /= 2;
    }
}

fn hiring_onlogn(array: &[i32]) {
    let n = array.len() as i32;
    for _i in 0..n {
        let mut count = 0;
        for _j in (1..=n).step_by(2) {
            count = 1;
        }
    }
}

fn generate_random_array(n: u128) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..1001)).collect()
}

fn save_results_to_file(results: &[(String, Vec<(u128, f64)>)], filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for (complexity, times) in results {
        writeln!(writer, "Complexity: {}", complexity).unwrap();
        for &(size, time) in times {
            writeln!(writer, "{} {}", size, time).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

fn plot_all_results(results: &[(String, Vec<(u128, f64)>)]) {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (min_size, max_size) = (10_u128.pow(3) as i64, 10_u128.pow(8) as i64); // Set appropriate range
    let min_time = 0.0;
    let max_time = results.iter()
        .flat_map(|(_, times)| times.iter().map(|&(_, time)| time))
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Time Complexity Comparisons", ("sans-serif", 20))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .margin(5)
        .build_cartesian_2d((min_size..max_size).log_scale(), (min_time..max_time).log_scale())
        .unwrap();

    chart.configure_mesh().x_desc("Size").y_desc("Time (s)").draw().unwrap();

    let colors = vec![RED, GREEN, BLUE];
    for (i, (complexity, times)) in results.iter().enumerate() {
        let color = colors[i]; // Copy color for this iteration
        chart.draw_series(LineSeries::new(
            times.iter().map(|&(size, time)| (size as i64, time)),
            &color,
        )).unwrap()
        .label(complexity)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color)); // Use color reference here
    }

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
}
