use core::cmp::min;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::env::consts;
use std::f64::consts::PI;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

const CONSISTENCY_RUNS: usize = 3;
const SAMPLE_SIZE: usize = 1230;

fn main() {
    println!("High Complexity Silicon Variation Detector");
    println!("=========================================");
    println!(
        "This program performs intensive computational tests to detect subtle silicon-level differences"
    );
    println!(
        "Each test will be run {} times to verify fingerprint consistency",
        CONSISTENCY_RUNS
    );

    let sys_info = format!(
        "System Information:\n\
        OS: {}\n\
        CPU: {}\n\
        Cores: {}\n",
        consts::OS,
        consts::ARCH,
        num_cpus::get()
    );
    println!("{}", sys_info);

    let filename = format!("fingerprint_{}-{}c.txt", consts::ARCH, num_cpus::get());

    let mut file = File::create(&filename).expect("Could not create output file");
    file.write_all(sys_info.as_bytes()).expect("Bruh nah");

    let test_names = [
        "Enhanced Denormal Numbers Test",
        "Transcendental Function Test",
    ];

    for &name in test_names.iter() {
        println!("\nRunning: {}", name);
        file.write_all(format!("\n\n{}\n", name).as_bytes())
            .expect("Not happening");

        let mut fingerprints = HashMap::new();
        let mut first_run_results = Vec::new();

        for run in 1..=CONSISTENCY_RUNS {
            println!("Run {}/{}...", run, CONSISTENCY_RUNS);

            let results = match name {
                "Enhanced Denormal Numbers Test" => enhanced_denormal_test(),
                "Transcendental Function Test" => transcendental_function_test(),
                _ => panic!("Bro..."),
            };

            if run == 1 {
                first_run_results = results.clone();
            }

            let fingerprint = calculate_fingerprint_full_precision(&results);

            *fingerprints.entry(fingerprint.clone()).or_insert(0) += 1;

            println!("→ Fingerprint: {}", fingerprint);
        }

        file.write_all(
            format!(
                "Raw results from first run ({} values, showing first 10):\n",
                first_run_results.len()
            )
            .as_bytes(),
        )
        .expect("Failed result preview");

        for i in 0..min(10, first_run_results.len()) {
            file.write_all(format!("{:4}: {:?}\n", i, first_run_results[i]).as_bytes())
                .unwrap();
        }

        file.write_all(format!("\nConsistency check over {} runs:\n", CONSISTENCY_RUNS).as_bytes())
            .expect("no");

        for (fingerprint, count) in fingerprints.iter() {
            let consistency_percentage = (*count as f64 / CONSISTENCY_RUNS as f64) * 100.0;

            let consistency_status = if *count == CONSISTENCY_RUNS {
                "CONSISTENT"
            } else {
                "INCONSISTENT"
            };

            file.write_all(
                format!(
                    "Fingerprint: {} - occurred {} out of {} times ({:.1}%) - {}\n",
                    fingerprint,
                    count,
                    CONSISTENCY_RUNS,
                    consistency_percentage,
                    consistency_status
                )
                .as_bytes(),
            )
            .unwrap();

            println!(
                "→ Consistency: {}/{} runs ({:.1}%) - {}",
                count, CONSISTENCY_RUNS, consistency_percentage, consistency_status
            );
        }
    }

    println!("\nTests completed! Results saved to {}", filename);
    println!("Run this program on different machines to compare silicon-level differences.");
}

// With lower sample sizes this will not be unique
fn enhanced_denormal_test() -> Vec<f64> {
    let mut results = Vec::with_capacity(SAMPLE_SIZE);

    let starting_values = [
        1e-308,
        2e-308,
        5e-308,
        1e-307,
        1e-320,
        2.2250738585072014e-308,
    ];

    for &start in starting_values.iter() {
        let mut x = start;
        let mut y = start * 1.112345;

        for i in 0..SAMPLE_SIZE / starting_values.len() {
            x = x / 1.1123156 + x * 0.9123545676;
            y = y * 0.951235467 + y / 1.05123245;

            let combined =
                x * (1.0 + (i as f64 * 0.01).sin()) + y * (1.0 + (i as f64 * 0.01).cos());

            let final_val =
                combined + (combined * 1e300).sin() * 1e-308 + (combined * 1e200).atan() * 1e-308;

            results.push(final_val);
        }
    }

    results
}

// This has appeared unique regardless of sample size
#[inline(never)]
fn transcendental_function_test() -> Vec<f64> {
    let mut results = Vec::with_capacity(SAMPLE_SIZE);
    let mut test_values = Vec::with_capacity(500);

    test_values.extend_from_slice(&[
        0.0,
        1e-15,
        PI / 6.0,
        PI / 4.0,
        PI / 3.0,
        PI / 2.0,
        PI,
        3.0 * PI / 2.0,
        2.0 * PI,
        1.0,
        -1.0,
        0.5,
        -0.534634634512312587,
        1e-10,
        -1e-10,
        1e15,
        -1e15,
    ]);

    for i in 0..500 {
        test_values.push(i as f64 * PI / 17.12344658922222221111154657);
    }

    for &val in test_values.iter() {
        let sin_val = val.sin();
        let cos_val = val.cos();

        let sin_of_sin = (sin_val * 10.0).sin();
        let exp_of_cos = cos_val.exp() - 1.0;

        let compound1 = val.sinh() * val.cosh() - 0.5 * (2.0 * val).sinh();
        let compound2 = (val.abs() + 1.0).log10() + (val.abs() + 2.0).log2();

        let atan_val = f64::atan(val);
        let tanh_val = f64::tanh(val);

        results.push(sin_val);
        results.push(cos_val);
        results.push(sin_of_sin);
        results.push(exp_of_cos);
        results.push(compound1);
        results.push(compound2);
        results.push(atan_val);
        results.push(tanh_val);

        let hypot = f64::hypot(sin_val, cos_val);
        results.push(hypot - 1.0);
    }

    results
}

fn calculate_fingerprint_full_precision(results: &[f64]) -> String {
    let mut hasher = DefaultHasher::new();

    for val in results {
        let bits = val.to_bits();
        bits.hash(&mut hasher);
    }

    format!("{:016x}", hasher.finish())
}
