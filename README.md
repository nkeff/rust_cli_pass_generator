# rust_cli_pass_generator
Simple cli password generator written in rust

# Usage
```
Usage: passwd_generator [OPTIONS]

Options:
  -l, --length <LENGTH>    длина будущего пароля [default: 16]
  -i, --include <INCLUDE>  Символы, используемые в пароле (numbers n | upppercase u | lowercase l | symbols s) [default: nuls]
  -h, --help               Print help
  -V, --version            Print version
```

# Run and build
``` bash
git clone https://github.com/nkeff/rust_cli_pass_generator
```

### Run
``` bash
cargo run
```

### Build
``` bash
cargo build -r
```

