use chumsky::prelude::*;
use s_text_input_f::{Paragraph, ParagraphItem};

#[derive(Debug)]
struct CorrectParagraphItem {
    input: ParagraphItem,
    answer: Option<String>,
}
#[derive(Debug)]
pub struct CorrectParagraph {
    pub input: Paragraph,
    pub answer: Vec<String>,
}
impl FromIterator<CorrectParagraphItem> for CorrectParagraph {
    fn from_iter<T: IntoIterator<Item = CorrectParagraphItem>>(iter: T) -> Self {
        let (input, answers): (Vec<_>, Vec<_>) = iter
            .into_iter()
            .map(|CorrectParagraphItem { input, answer }| (input, answer))
            .unzip();
        let answer = answers.into_iter().flatten().collect::<Vec<_>>();
        Self { input, answer }
    }
}

pub fn parse_paragraph(input: &str) -> Result<CorrectParagraph, Vec<Simple<char>>> {
    let text = filter::<_, _, Simple<char>>(|&ch| {
        ch != '`'
            && (char::is_alphanumeric(ch)
                || char::is_whitespace(ch)
                || char::is_ascii_punctuation(&ch))
    })
    .repeated()
    .at_least(1)
    .collect::<String>();
    let input_field = just('`')
        .ignore_then(
            filter(|&ch| {
                ch != '`'
                    && (char::is_alphanumeric(ch)
                        || char::is_whitespace(ch)
                        || char::is_ascii_punctuation(&ch))
            })
            .repeated()
            .collect::<String>(),
        )
        .then_ignore(just('`'));

    let paragraph_item = choice((
        text.map(|text| CorrectParagraphItem {
            input: ParagraphItem::Text(text),
            answer: None,
        }),
        input_field.map(|text| CorrectParagraphItem {
            input: ParagraphItem::Placeholder,
            answer: Some(text),
        }),
    ));
    let paragraph = paragraph_item
        .repeated()
        .at_least(1)
        .then_ignore(end()) // NOTE: remove when create Block parser
        .map(CorrectParagraph::from_iter);

    paragraph.parse(input)
}
