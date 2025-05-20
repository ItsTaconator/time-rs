use arguments::Arguments;
use clap::Parser;
use std::process::Command;
use std::time::Duration;

pub mod arguments;

fn main() {
    let args = Arguments::parse();
    let mut total_time = Duration::default();
    let times = args.times as u128;

    let mut min_elapsed = Duration::MAX;
    let mut max_elapsed = Duration::from_millis(0);

    for _ in 0..times {
        let instant = std::time::Instant::now();

        let mut command = &mut Command::new("sh");
        command = command.arg("-c").arg(args.command.clone().join(" "));
        if !args.stdout {
            command = command.stdout(std::process::Stdio::null());
        }

        _ = command.status();

        let elapsed = instant.elapsed();
        if elapsed < min_elapsed {
            min_elapsed = elapsed;
        }

        if elapsed > max_elapsed {
            max_elapsed = elapsed;
        }

        total_time += instant.elapsed();
    }

    println!(
        "Total time taken to run \"{}\" {times} time(s): {} ms\nMinimum execution time: {} ms\nMaximum execution time: {} ms\nAverage execution time: {} ms",
        args.command.join(" "),
        total_time.as_millis(),
        min_elapsed.as_millis(),
        max_elapsed.as_millis(),
        total_time.as_millis() / times,
    );
}
