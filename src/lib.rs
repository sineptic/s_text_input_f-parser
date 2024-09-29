use chumsky::prelude::*;
use s_text_input_f::{Block, Blocks};

#[derive(Debug)]
pub struct CorrectBlocks {
    pub blocks: Blocks,
    pub answer: Vec<Vec<String>>,
}
impl FromIterator<CorrectBlock> for CorrectBlocks {
    fn from_iter<T: IntoIterator<Item = CorrectBlock>>(iter: T) -> Self {
        let (blocks, answer) = iter.into_iter().map(|x| (x.block, x.answer)).unzip();
        Self { blocks, answer }
    }
}

#[derive(Debug)]
pub struct CorrectBlock {
    pub block: Block,
    pub answer: Vec<String>,
}
impl From<paragraph::CorrectParagraph> for CorrectBlock {
    fn from(value: paragraph::CorrectParagraph) -> Self {
        Self {
            block: Block::Paragraph(value.input),
            answer: value.answer,
        }
    }
}
impl From<one_of::CorrectOneOf> for CorrectBlock {
    fn from(value: one_of::CorrectOneOf) -> Self {
        Self {
            block: Block::OneOf(value.variants),
            answer: vec![value.correct.to_string()],
        }
    }
}
impl From<any_of::CorrectAnyOf> for CorrectBlock {
    fn from(value: any_of::CorrectAnyOf) -> Self {
        Self {
            block: Block::AnyOf(value.variants),
            answer: value.correct.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

pub fn parse_paragraph(input: &str) -> Result<paragraph::CorrectParagraph, Vec<Simple<char>>> {
    paragraph::paragraph_parser()
        .then_ignore(end())
        .parse(input)
}
pub fn parse_one_of(input: &str) -> Result<one_of::CorrectOneOf, Vec<Simple<char>>> {
    one_of::one_of_parser().then_ignore(end()).parse(input)
}
pub fn parse_any_of(input: &str) -> Result<any_of::CorrectAnyOf, Vec<Simple<char>>> {
    any_of::any_of_parser().then_ignore(end()).parse(input)
}

pub mod any_of;
pub mod one_of;
pub mod paragraph;

pub fn parse_block(input: &str) -> Result<CorrectBlock, Vec<Simple<char>>> {
    block_parser().then_ignore(end()).parse(input)
}

fn block_parser() -> impl Parser<char, CorrectBlock, Error = Simple<char>> {
    choice((
        any_of::any_of_parser().map(CorrectBlock::from),
        one_of::one_of_parser().map(CorrectBlock::from),
        paragraph::paragraph_parser().map(CorrectBlock::from),
        todo(),
    ))
}

pub fn parse_blocks(input: &str) -> Result<CorrectBlocks, Vec<Simple<char>>> {
    blocks_parser().then_ignore(end()).parse(input)
}
fn blocks_parser() -> impl Parser<char, CorrectBlocks, Error = Simple<char>> {
    block_parser()
        .separated_by(just('\n').repeated().at_least(1))
        .at_least(1)
        .collect()
}
