use clap:: Parser;
use tax_code;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    state: String,
    income: f64,
    #[clap(short, long, default_value = "single")]
    filing_status: String,
}

fn main() {
    let args = Args::parse();

    println!("Calculating taxes for: {}, income: ${:.2}, filing status: {}",
            args.state, args.income, args.filing_status);

    if args.state.to_uppercase() == "CA" {
        let taxes = tax_code::cali_income_tax(args.income, &args.filing_status);
        println! ("California Income Tax: ${:.2}", taxes);
    } else {
        println! ("Tax calculation for '{}' is not yet supported", args.state);
    }
}