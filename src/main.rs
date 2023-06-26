use std::io::stdin;

mod instr;
mod repr;

fn main() {
    let instr_list = stdin()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            u32::from_str_radix(&line.expect("IO error reading line from stdin"), 16)
                .expect(&format!("line {i}: is not a 8 digit hex number"))
        })
        .map(|instr_int| instr::Instr::from_u32(instr_int).unwrap())
        .collect::<Vec<_>>();

    match std::env::args().nth(1).as_deref() {
        Some("-l") => instr::print_labelled(&instr_list),
        Some("-u") => instr::print_unlabelled(&instr_list),
        _ => instr::print_labelled(&instr_list),
    }
}
