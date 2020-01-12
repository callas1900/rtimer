use std::cmp::min;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(StructOpt)]
#[structopt(about = "simple timer for command line.")]
struct Cli {
    #[structopt(help = "input time for timer.")]
    time: String,
    #[structopt(default_value = "sec", help = "select unit from `sec`, `min` or `hour`.")]
    unit: String
}

fn main() {
    let args = Cli::from_args();
    let mut input: u64 = match &args.time.trim().parse() {
        Ok(input) => *input,
        Err(_) => {
            println!("fail");
            0
        }
    };
    let unit: String = args.unit;
    input = match unit.as_str() {
        "sec" => input * 10,
        "min" => input * 600,
        "hour" => input * 36000,
        _ => input,
    };
    
    let mut cursor = 0;
    let pb = ProgressBar::new(input);

    clear_screen();
    pb.set_style(ProgressStyle::default_bar()
        .template(format!("rtimer: {}[{}] --> {{percent}}% \n {{spinner:.green}} [{{elapsed_precise}}] [{{wide_bar:.cyan/blue}}] {{spinner:.red}} {{eta_precise}}", args.time, unit).as_str())
        .progress_chars("█░"));

    while pb.position() < input {
        let new = min(cursor + 1, input);
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));
    }
    pb.finish();
}

fn clear_screen() {
    std::process::Command::new("clear").spawn().expect("failed to clear terminal screen.");
}

