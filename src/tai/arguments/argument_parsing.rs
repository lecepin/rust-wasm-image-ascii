// use crate::tai::{Config, Style};
// const VERSION: &str = "0.0.8"; // program version

// pub fn parse(args: Vec<String>) -> Option<Config> {
//     // defaults
//     let mut config = Config::default();
//     let program = args[0].clone();
//     let mut opts = Options::new();

//     config.original_size = true;
//     config.style = check_style_arg(&style);
//     config.onechar = onechar;

//     config.scale = scale;

//     config.image_file = matches.free[0].to_string();
// }

// fn check_style_arg(arg: &str) -> Style {
//     match arg {
//         "ascii" => Style::Ascii,
//         "blocks" => Style::Blocks,
//         "braille" => Style::Braille,
//         "numbers" => Style::Numbers,
//         "onechar" => Style::OneChar,
//         _ => Style::default(),
//     }
// }
