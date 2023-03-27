use std::{cmp::Ordering, fmt::Display};

use crate::parser::Token;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block {
    height: usize,
    width: usize,
    s: String,
}

impl Block {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            s: (" ".repeat(width) + "\n").repeat(height),

            ..Default::default()
        }
    }

    pub fn set(&mut self, row: usize, col: usize, s: &impl AsRef<str>) {
        let mut row = row;

        for ln in s.as_ref().lines() {
            let offset = row * (self.width + 1) + col;
            let len = ln.chars().count();

            let start = self
                .s
                .char_indices()
                .nth(offset)
                .expect("char at offset start")
                .0;

            let end = self
                .s
                .char_indices()
                .nth(offset + len)
                .expect("char at offset end")
                .0;

            self.s.replace_range(start..end, ln);
            row += 1
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

fn resize((width, height): (usize, usize), ln: &str) -> (usize, usize) {
    (width.max(ln.chars().count()), height + 1)
}

impl From<&str> for Block {
    fn from(value: &str) -> Self {
        let (width, height) = value.lines().fold((0, 0), resize);
        let mut buf = Self::new(width, height);

        buf.set(0, 0, &value);
        buf
    }
}

impl AsRef<str> for Block {
    fn as_ref(&self) -> &str {
        &self.s
    }
}

fn render_conjunction(children: &Vec<Token>) -> Block {
    let child_blocks: Vec<_> = children.into_iter().map(render_token).collect();
    let (width, height) = child_blocks
        .iter()
        .fold((0, 0), |(w, h), b| (w + b.width, h.max(b.height)));

    let mut new_block = Block::new(width, height);
    let mut col = 0;

    for child_block in child_blocks.iter() {
        let row = (height - child_block.height) / 2;
        new_block.set(row, col, &child_block);
        col += child_block.width;
    }

    new_block
}

fn render_disjunction_line(idx: usize, len: usize, ordering: Ordering, width: usize) -> String {
    println!("idx={idx}, len={len}, ordering={ordering:?}");
    match ordering {
        Ordering::Equal if idx == 0 => format!("┬{}┬", "─".repeat(width)),
        Ordering::Equal if idx == len - 1 => format!("┴{}┴", "─".repeat(width)),
        Ordering::Equal => format!("┼{}┼", "─".repeat(width)),
        Ordering::Less if idx == 0 => format!("╭{}╮", "─".repeat(width)),
        Ordering::Greater if idx == len - 1 => format!("╰{}╯", "─".repeat(width)),
        _ => format!("├{}┤", "─".repeat(width)),
    }
}

fn render_disjunction(children: &Vec<Token>) -> Block {
    let child_blocks: Vec<_> = children.into_iter().map(render_token).collect();
    let (width, height) = child_blocks
        .iter()
        .fold((0, 0), |(w, h), b| (w.max(b.width), h + b.height));

    let height = (height + 1) % 2 + height;
    let middle = height / 2;

    let mut new_block = Block::new(width + 2, height);
    let mut row = 0;

    if child_blocks.len() >= 2 {
        let last = child_blocks.last().unwrap();
        let first_middle = child_blocks.get(0).unwrap().height / 2;
        let last_middle = last.height / 2 + child_blocks.iter().fold(0, |sum, b| sum + b.height)
            - last.height / 2
            - 1;

        for r in first_middle..last_middle {
            new_block.set(r, 0, &format!("│{}│", " ".repeat(width)));
        }
    }

    new_block.set(middle, 0, &format!("┤{}├", " ".repeat(width)));

    for (i, child_block) in child_blocks.iter().enumerate() {
        let child_middle = child_block.height / 2;

        new_block.set(
            row + child_middle,
            0,
            &render_disjunction_line(
                i,
                child_blocks.len(),
                (row + child_middle).cmp(&middle),
                width,
            ),
        );

        new_block.set(row, 1, &child_block);
        row += child_block.height;
    }

    new_block
}

fn render_quantifier(tok: &Token, min: usize, max: Option<usize>) -> Block {
    let block = render_token(tok);
    let zero = min == 0;
    let more_than_one = max.unwrap_or(2) > 1;
    let width = block.width.max(2);
    let mut new_block = Block::new(width + 2, block.height + 2);

    if zero {
        for i in 1..new_block.height / 2 {
            new_block.set(i, 0, &format!("│{}│", " ".repeat(width)));
        }

        new_block.set(0, 0, &format!("╭{}╮", "─".repeat(width)));
        new_block.set(new_block.height / 2, 0, &format!("┴{}┴", "─".repeat(width)));
    } else {
        new_block.set(new_block.height / 2, 0, &format!("─{}─", "─".repeat(width)));
    }

    if more_than_one {
        new_block.set(block.height + 1, 1, &format!("╰{}╯", "─".repeat(width - 2)));
    }

    new_block.set(1, 1, &block);
    new_block
}

pub fn render_token(tok: &Token) -> Block {
    match tok {
        Token::Literal(ch) => Block::from(format!("{ch}").as_str()),
        Token::Start => Block::from("^"),
        Token::End => Block::from("$"),
        Token::Alphanumeric => Block::from("\\w"),
        Token::NotAlphanumeric => Block::from("\\W"),
        Token::Digit => Block::from("\\d"),
        Token::NotDigit => Block::from("\\D"),
        Token::Whitespace => Block::from("\\s"),
        Token::NotWhitespace => Block::from("\\S"),
        Token::WordBoundary => Block::from("\\b"),
        Token::Any => Block::from("."),
        Token::Conjunction(tokens) => render_conjunction(tokens),
        Token::Disjunction(tokens) => render_disjunction(tokens),
        Token::LazyQuantifier(tok, min, max) => render_quantifier(tok, *min, *max),
        Token::GreedyQuantifier(tok, min, max) => render_quantifier(tok, *min, *max),
        Token::Capturing(tok, _) => render_token(tok),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_size() {
        let b1: Block = "My\nfirst\nblock".into();

        assert_eq!(b1.width, 5);
        assert_eq!(b1.height, 3);
    }

    #[test]
    fn test_set_block_in_block() {
        let b1: Block = "My\nfirst\nblock".into();
        let mut b2 = Block::new(10, 5);

        b2.set(2, 1, &b1);

        assert_eq!(
            b2.as_ref(),
            vec![
                "          ",
                "          ",
                " My       ",
                " first    ",
                " block    ",
                "",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_literal() {
        let b = render_token(&Token::Literal('a'));

        assert_eq!(b.width, 1);
        assert_eq!(b.height, 1);
        assert_eq!(&b.s, "a\n");
    }

    #[test]
    fn test_conjunction() {
        let b = render_token(&Token::Conjunction(vec![
            Token::Literal('h'),
            Token::Literal('e'),
            Token::Literal('l'),
            Token::Literal('l'),
            Token::Literal('o'),
        ]));

        assert_eq!(b.width, 5);
        assert_eq!(b.height, 1);
        assert_eq!(&b.s, "hello\n");
    }

    #[test]
    fn test_disjunction_odd() {
        let b = render_token(&Token::Disjunction(vec![
            Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ]),
            Token::Conjunction(vec![Token::Literal('a')]),
            Token::Whitespace,
        ]));

        assert_eq!(
            &b.s,
            &vec!["╭hello╮", "┼a────┼", "╰\\w───╯", "",].join("\n")
        );
    }

    #[test]
    fn test_disjunction_even() {
        let b = render_token(&Token::Disjunction(vec![
            Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ]),
            Token::Conjunction(vec![Token::Literal('a')]),
        ]));

        assert_eq!(&b.s, &vec!["╭hello╮", "┴a────┴", "       ", "",].join("\n"));
    }

    #[test]
    fn test_quantifier_0_n() {
        let b = render_token(&Token::LazyQuantifier(
            Box::new(Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ])),
            0,
            None,
        ));

        assert_eq!(&b.s, &vec!["╭─────╮", "┴hello┴", " ╰───╯ ", "",].join("\n"));
    }

    #[test]
    fn test_quantifier_0_2() {
        let b = render_token(&Token::LazyQuantifier(
            Box::new(Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ])),
            0,
            Some(2),
        ));

        assert_eq!(&b.s, &vec!["╭─────╮", "┴hello┴", " ╰───╯ ", "",].join("\n"));
    }

    #[test]
    fn test_quantifier_1_n() {
        let b = render_token(&Token::LazyQuantifier(
            Box::new(Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ])),
            1,
            None,
        ));

        assert_eq!(&b.s, &vec!["       ", "─hello─", " ╰───╯ ", "",].join("\n"));
    }

    #[test]
    fn test_quantifier_0_1() {
        let b = render_token(&Token::LazyQuantifier(
            Box::new(Token::Conjunction(vec![
                Token::Literal('h'),
                Token::Literal('e'),
                Token::Literal('l'),
                Token::Literal('l'),
                Token::Literal('o'),
            ])),
            0,
            Some(1),
        ));

        assert_eq!(&b.s, &vec!["╭─────╮", "┴hello┴", "       ", "",].join("\n"));
    }
}
