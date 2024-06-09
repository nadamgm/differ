pub mod diff;


pub mod arg_parsing {
    use std::str::FromStr;

    use clap::{Parser, ValueEnum};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Arguments {
        #[arg(default_value_t = DiffmodeArg::Delta)]
        pub diff_mode: DiffmodeArg,
        #[arg(short = 'b')]
        pub baseline_file_path: String,
        #[arg(short = 'c')]
        pub comparee_file_path: String,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
    pub enum DiffmodeArg {
        Delta,
        Pairs,
    }

impl ToString for DiffmodeArg {
    fn to_string(&self) -> String {
        match self {
            DiffmodeArg::Delta => String::from_str("delta").unwrap(),
            DiffmodeArg::Pairs => String::from_str("pairs").unwrap(),
        }
    }
}

}