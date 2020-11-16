use std::io;
use std::process::Command;

const INIT_ANSWER: &str = "y";

fn deny_shutdown() {
    println!("\r\n\r\n.................");
    println!("No auto shutdown schedule has been set");
    println!("Exiting program");
}

fn initiate_schedule(time_in_seconds: &str) {
    println!("\r\n\r\n.................");

    let output = Command::new("shutdown")
        .arg("-s")
        .arg("-t")
        .arg(time_in_seconds)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("shutdown schedule succeeded:\n{}", s);
        println!(
            "Your device has been scheduled to shutdown in {} seconds",
            time_in_seconds
        );
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("shutdown schedule failed:\n{}", s);
    }
}

fn set_shutdown_schedule() {
    println!("\r\n\r\n.................");
    println!("You are about to schedule an automatic shutdown");
    println!("Enter the time in seconds in which you would like the shutdown to occur");

    let mut time_in_seconds = String::new();

    io::stdin()
        .read_line(&mut time_in_seconds)
        .expect("Failed to read shutdown time in seconds");

    println!("\r\n\r\n.................");
    println!(
        "You are about to schedule a device shutdown in {} seconds.",
        &time_in_seconds.trim(),
    );
    println!("Are you sure to continue? [y/N]");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer");

    if answer.trim() == INIT_ANSWER {
        initiate_schedule(time_in_seconds.trim());
    } else {
        deny_shutdown();
    }
}

fn main() {
    println!("Would you like to schedule a shutdown timer? [y/N]");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    if answer.trim() == INIT_ANSWER {
        set_shutdown_schedule();
    } else {
        deny_shutdown();
    }
}
