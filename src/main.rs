use std::process::Command;

use arguments::Arguments;
use clap::Parser;

pub mod arguments;

fn main() {
    let args = Arguments::parse();
    let mut total_time = std::time::Duration::default();
    let times = args.times as u128;

    for _ in 0..times {
        let instant = std::time::Instant::now();
        
        let mut command = &mut Command::new("sh");
        command = command.arg("-c").arg(args.command.clone().join(" "));
        if !args.stdout {
            command = command.stdout(std::process::Stdio::null());
        }

        _ = command.status();

        total_time += instant.elapsed();
    }

    println!(
        "Total time taken to run \"{}\" {times} time(s): {} ms\nAverage execution time: {} ms",
        args.command.join(" "),
        total_time.as_millis(),
        total_time.as_millis() / times
    );
}
