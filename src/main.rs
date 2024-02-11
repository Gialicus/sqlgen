mod parser;
mod template;
mod utils;

use clap::Parser;

use crate::{
    parser::parser::parse_field,
    template::{
        create_table::create_table_template, insert_into::insert_into_template,
        update_table::update_template,
    },
};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "Example: cargo run -- -t users -f id:integer:primarykey -f email:notnull:unique -f age:integer"
)]
struct Cli {
    #[arg(short, long)]
    table: String,

    #[arg(short, long)]
    fields: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("Table: {}", cli.table);
    println!("Fields: {:?}\n", cli.fields);
    let mut fields = Vec::new();
    cli.fields.iter().for_each(|i| fields.push(parse_field(i)));
    let create_table = create_table_template(&cli.table, &fields);
    println!("{}", create_table);
    let insert = insert_into_template(&cli.table, &fields);
    println!("{}", insert);
    let update = update_template(&cli.table, &fields);
    println!("{}", update);
}
