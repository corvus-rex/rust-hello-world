use rand::Rng;
use std::time::Instant;
use plotters::prelude::*;

fn main() {
    let sizes = vec![10_usize.pow(4), 10_usize.pow(5), 10_usize.pow(6), 10_usize.pow(7)];

    let complexity = "n"; // Change to "n", "logn", or "nlogn"
    let mut times = vec![(0, 0.0); sizes.len()]; 

    for (index, &n) in sizes.iter().enumerate() {
        let c = generate_random_array(n); 

        let (time_taken, _) = hire(&c, complexity); 
        times[index].0 = n; 
        times[index].1 = time_taken.as_secs_f64(); 
    }

    plot_results(&times, complexity); 
}

fn hire(c: &[i64], complexity: &str) -> (std::time::Duration, i64) {
    let mut hired = 0;
    let mut i = 0;
    let total_count = c.len() as i64;

    let start = Instant::now(); // Start timing
    while i < c.len() {
        println!("Current: {} / Total: {}", i as i64 + 1, total_count);
        if c[i] > hired {
            match complexity {
                "n" => hiring_on(c),         // O(n) algorithm
                "logn" => hiring_ologn(c),   // O(log n) algorithm
                "nlogn" => hiring_onlogn(c), // O(n log n) algorithm
                _ => panic!("Unknown complexity! Use 'n', 'logn', or 'nlogn'."),
            }
            hired = c[i];
        }
        i += 1; 
    }
    (start.elapsed(), hired) // Return the elapsed time and last hired value
}

fn hiring_on(c: &[i64]) {
    let _sum: i64 = c.iter().sum(); 
}

fn hiring_ologn(c: &[i64]) {
    let mut n = c.len() as i64; 
    while n > 0 {
        n /= 2; // Halve n to simulate logarithmic behavior
    }
}

fn hiring_onlogn(c: &[i64]) {
    let n = c.len() as i64;
    for _i in 0..n {
        for _j in (1..=n).step_by(2) {}
    }
}

fn generate_random_array(n: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..10001)).collect()  
}

fn plot_results(times: &[(usize, f64)], complexity: &str) {
    let filename = format!("{}.png", complexity); // Create filename based on complexity
    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (max_usize, max_f64) = find_max(times);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Time Taken vs Size of c ({})", complexity), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(1..max_usize, 1e-5..max_f64) // Adjust Y range as needed
        .unwrap();

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();

    chart
        .configure_mesh()
        .x_desc("Size of c")
        .y_desc("Time (seconds)")
        .x_label_formatter(&|x: &i64| x.to_string()) // Change to &i32
        .y_label_formatter(&|y: &f64| format!("{:.3}", y)) // Keep this as is
        .draw()
        .unwrap();

    // Draw the series
    chart
        .draw_series(LineSeries::new(
            times.iter().map(|&(size, time)| (size as i64, time)), // Cast size to i32
            &RED,
        ))
        .unwrap()
        .label(format!("Time Complexity: {}", complexity))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &RED));

    chart
        .configure_series_labels()
        .draw()
        .unwrap();
}

fn find_max(times: &[(usize, f64)]) -> (i64, f64) {
    let mut max_usize: Option<usize> = None;
    let mut max_f64: Option<f64> = None;

    for &(us, fl) in times {
        // Update max_usize if necessary
        max_usize = match max_usize {
            Some(max) if us > max => Some(us),
            Some(max) => Some(max),
            None => Some(us),
        };

        // Update max_f64 if necessary
        max_f64 = match max_f64 {
            Some(max) if fl > max => Some(fl),
            Some(max) => Some(max),
            None => Some(fl),
        };
    }

    // Convert max_usize to i64 and unwrap if they exist
    let max_usize = max_usize.map(|val| val as i64).unwrap_or_default();
    // Unwrap max_f64, returning 0.0 if it doesn't exist
    let max_f64 = max_f64.unwrap_or(0.0);

    (max_usize, max_f64)
}
