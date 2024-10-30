use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow:: Result <()>{
   let got = Grammar:: parse(Rule::field, "-6.98,-14\n")?;
   println!("{:?}", got);
   Ok(())
}