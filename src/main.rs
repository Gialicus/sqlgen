mod parser;
mod template;
mod utils;

use clap::Parser;

use crate::{
    parser::parser::parse_field,
    template::{create_table::create_table_template, insert_into::insert_into_template},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    fields: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("Table: {}", cli.name);
    println!("fields: {:?}", cli.fields);
    let mut fields = Vec::new();
    cli.fields.iter().for_each(|i| fields.push(parse_field(i)));
    let create_table = create_table_template(&cli.name, &fields);
    println!("{}", create_table);
    let insert = insert_into_template(&cli.name, &fields);
    println!("{}", insert);
}
