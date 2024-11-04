// Start
use rand::Rng;
use std::fs::File;
use std::time::Instant;
use std::io::{self, Write};
use plotters::prelude::*;


fn bucket_sort(array: &mut [i64], k: usize) {
    let mut buckets: Vec<Vec<i64>> = vec![Vec::new(); k];
    let max_value = *array.iter().max().unwrap();
    for &value in array.iter() {
        let index = ((k as i64 * value) / (max_value + 1)) as usize;
        buckets[index].push(value);
    }
    for bucket in &mut buckets {
        selection_sort(bucket);
    }
    let mut idx = 0;
    for bucket in buckets {
        for &value in bucket.iter() {
            array[idx] = value;
            idx += 1;
        }
    }
}

fn selection_sort(arr: &mut Vec<i64>) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx); 
    }
}


fn plot_all_results(results: &[(String, Vec<(u128, f64)>)]) {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (min_size, max_size) = (10_u128.pow(3) as i64, 3*10_u128.pow(8) as i64); 
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
    for (i, (sorting, times)) in results.iter().enumerate() {
        let color = colors[i]; // Copy color for this iteration
        chart.draw_series(LineSeries::new(
            times.iter().map(|&(size, time)| (size as i64, time)),
            &color,
        )).unwrap()
        .label(sorting)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
}

fn main() -> io::Result<()> {
    let sizes_bucket = [1000, 
    5000, 10000, 50000, 100000, 500000, 
    1000000, 5000000, 10000000, 50000000];

    let mut results: Vec<(String, Vec<(u128, f64)>)> = Vec::new();
    let mut bucket_sort_times: Vec<(u128, f64)> = Vec::new();
    let mut selection_sort_times: Vec<(u128, f64)> = Vec::new();
    
    for &size in sizes_bucket.iter() {
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i64> = (0..size).map(|_| rng.gen_range(0..=size as i64)).collect();
        
        let start = Instant::now();
        
        let num_buckets = (size as f64).sqrt().floor() as usize;
        bucket_sort(&mut arr, num_buckets);
        
        let duration = start.elapsed().as_secs_f64();
        
        bucket_sort_times.push((size as u128, duration));
        
        println!("Bucket sorted array of size {} in {} seconds", size, duration);

        if size <= 1_00000 {
            let start = Instant::now();
            selection_sort(&mut arr);
            let duration = start.elapsed().as_secs_f64();
            selection_sort_times.push((size as u128, duration));
            println!("Selection sorted array of size {} in {} seconds", size, duration);
        }
    }
    
    results.push(("Bucket Sort".to_string(), bucket_sort_times));
    results.push(("Selection Sort".to_string(), selection_sort_times));
    
    let mut file = File::create("bucket_sort_times.txt")?;
    for (algorithm, times) in results.iter() {
        writeln!(file, "Algorithm: {}", algorithm)?;
        for (size, time) in times {
            writeln!(file, "Size: {}, Time: {}", size, time)?;
        }
    }
    
    plot_all_results(&results);
    Ok(())
}
