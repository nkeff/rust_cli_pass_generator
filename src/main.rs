use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Parser)]
#[clap(
    about = "Генератор паролей",
    version = "0.1",
    author = "@art"
)]
#[command(author, version, about, long_about = None)]
struct Args  {
    /// длина будущего пароля
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Символы, используемые в пароле (numbers n | upppercase u | lowercase l | symbols s)
    #[arg(short, long, default_value_t = String::from("nuls"))]
    include: String,
}

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();
    let mut pass_string = String::from("");

    if args.include.contains("n") {
        pass_string += "0123456789";
    }

    if args.include.contains("l") {
        pass_string += "abcdefghijklmnopqrstuvwxyz";
    }

    if args.include.contains("u") {
        pass_string += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    if args.include.contains("s") {
        pass_string += ")(*&^%$#@!~";
    }

    let password = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0, pass_string.len());
            pass_string.chars().nth(idx).unwrap()
        })
        .collect::<String>();
    println!("{}", password);
}