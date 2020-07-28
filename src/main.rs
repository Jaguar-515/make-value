mod enums;
mod trees;
mod generator;

use enums::{
    FileType,
    Value
};

use generator::generate;

use clap::{
    App,
    Arg
};

fn main() {
    let matches = App::new("make-value")
        .version("1.0.0")
        .about("a cool app")
        .author("Jaguar-515 <j.aguar.515.515@gmail.com>")
        .arg(Arg::with_name("value_type")
            .short("v")
            .long("value")
            .takes_value(true)
            .help("the value you want to generate")
            .required(true)
        )

        .arg(Arg::with_name("file_type")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("the file type for the model you want to generate")
            .required(true)
        )

        .arg(Arg::with_name("file_name")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("the name of the model file you want to generate")
            .required(true)
        )
    .get_matches();

    let value_type = matches.value_of("value_type").unwrap();
    let file_type = matches.value_of("file_type").unwrap();
    let file_name = matches.value_of("file_name").unwrap();

    let value = Value::detect(value_type).unwrap();
    let filetype = FileType::detect(file_type).unwrap();
    
    generate(value, filetype, file_name)
}