mod crypto;

use clap::Parser;
use crypto::gen_pass;

#[derive(Parser)]
#[clap(
    about = "CLI парольный менеджер-генератор",
    version = "0.1",
    author = "@art"
)]
#[command(author, version, about, long_about = None)]
struct Args  {
    /// адрес / уникальный идентификатор сайта
    #[arg(short, long)]
    login: String,

    /// мастер пароль
    #[arg(short, long)]
    password: String,
}


fn main() {
    let args = Args::parse();
    let salt: String = String::from("some salt string");
    println!("{}", gen_pass(&args.login, &args.password, &salt));
}
