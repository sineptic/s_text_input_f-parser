use s_text_input_f_parser::parse_paragraph;

fn main() {
    let input = "hello `world`!";
    let output = parse_paragraph(input);
    println!("{output:#?}");
}
