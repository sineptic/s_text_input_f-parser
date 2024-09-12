use s_text_input_f_parser::parse_paragraph;

fn test(input: &str) {
    let output = parse_paragraph(input);
    println!("------------------------------------------------------------------");
    println!("input: {input}");
    println!("{output:#?}");
    println!();
}

fn main() {
    test("hello `world`!");

    test("`precipice` - a very steep side of a cliff or a mountain");
    test("precipice - a very `steep` side of a `cliff` or a `mountain`");

    test("`in advance` - before a particular time, or before doing a particular thing");
    test("in advance - `before` a particular time, or `before` doing a particular thing");

    test("`consequence` - a result of a particular action or situation, often one that is bad or not convenient");
    test("consequence - a `result` of a particular action or situation, often one that is `bad` or `not convenient`");
}
