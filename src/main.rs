use std::{collections::HashSet, fmt::Display, str::FromStr};

use arrayvec::ArrayVec;
use repeated::repeated;
// use crossterm::{cursor, execute, terminal::enable_raw_mode, Result};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}
impl Card {
    fn is_sequential(&self, other: &Card) -> bool {
        self.rank.is_sequential(&other.rank)
    }

    fn repr(&self) -> [&'static str; 2] {
        [self.rank.repr(), self.suit.repr()]
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [rank, suit] = self.repr();
        write!(f, "{rank}{suit}")
    }
}
impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }
        let rank = match s.as_bytes()[0] {
            b'a' | b'A' => Rank::Ace,
            b'2' => Rank::Two,
            b'3' => Rank::Three,
            b'4' => Rank::Four,
            b'5' => Rank::Five,
            b'6' => Rank::Six,
            b'7' => Rank::Seven,
            b'8' => Rank::Eight,
            b'9' => Rank::Nine,
            b't' | b'T' => Rank::Ten,
            b'j' | b'J' => Rank::Jack,
            b'q' | b'Q' => Rank::Queen,
            b'k' | b'K' => Rank::King,
            _ => return Err(()),
        };
        let suit = match s.as_bytes()[1] {
            b'c' | b'C' => Suit::Clubs,
            b'd' | b'D' => Suit::Diamonds,
            b'h' | b'H' => Suit::Hearts,
            b's' | b'S' => Suit::Spades,
            _ => return Err(()),
        };
        Ok(Card { rank, suit })
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}
impl Rank {
    fn is_sequential(&self, other: &Rank) -> bool {
        (self.value() + 1) % 13 == other.value() % 13 || self.value() - 1 == other.value() % 13
    }
    fn value(&self) -> usize {
        *self as usize
    }
    fn repr(&self) -> &'static str {
        match *self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl Suit {
    fn repr(&self) -> &'static str {
        match *self {
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
            Suit::Hearts => "h",
            Suit::Spades => "s",
        }
    }
}

#[derive(Clone, Copy)]
struct TriPeaksBoard {
    cards: [Option<Card>; 28],
}
impl TriPeaksBoard {
    fn is_cleared(&self) -> bool {
        self.cards.iter().all(|x| x.is_none())
    }

    fn free_card_indices(&self) -> ArrayVec<usize, 10> {
        let mut free_indices = ArrayVec::new();
        let mut found = 0;
        for i in (0..self.cards.len()).rev() {
            if self.cards[i].is_none() {
                continue;
            }
            let second_offset = i.saturating_sub(3) / 2;
            // the highest row of cards is always free
            if (i >= 18)
            // the third row
            || ((9..18).contains(&i) && self.cards[i + 9].is_none() && self.cards[i + 10].is_none())
            // the second row
            || ((3..9).contains(&i) && self.cards[i + 6 + second_offset].is_none()
                && self.cards[i + 7 + second_offset].is_none())
            // the peaks
            || ((0..3).contains(&i) && self.cards[i + 3 + i].is_none() && self.cards[i + 4 + i].is_none())
            {
                free_indices.push(i);
                found += 1;
            }
            if found == 10 {
                break;
            }
        }
        free_indices
    }
}
impl Display for TriPeaksBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        repeated!(
        %%write prelude
        write!(
            f,
            concat!(
                "\n",
                "      {}          {}          {}\n",
                "    {}  {}      {}  {}      {}  {}\n",
                "  {}  {}  {}  {}  {}  {}  {}  {}  {}\n",
                "{}  {}  {}  {}  {}  {}  {}  {}  {}  {}\n",
            ),
            prelude write%%
            for i in [0;27] {
                match self.cards[%%i%%] {
                    Some(card) => card.to_string(),
                    None => "  ".to_string(),
                },
            }
        %%write postlude
            )
        postlude write%%)
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Stock(Card),
    Pyramid(Card),
}
impl Move {
    fn output(&self) {
        match *self {
            Move::Stock(card) => {
                println!("Reveal {} from the stock", card.repr().join(""));
            }
            Move::Pyramid(card) => {
                println!("Move {} onto the stock", card.repr().join(""));
            }
        }
    }
}
impl ToString for Move {
    fn to_string(&self) -> String {
        match *self {
            Move::Stock(card) => {
                format!("Reveal {} from the stock", card.repr().join(""))
            }
            Move::Pyramid(card) => {
                format!("Move {} onto the stock", card.repr().join(""))
            }
        }
    }
}

/// Solve a game of TriPeaks.
fn solve(board: TriPeaksBoard, stock: ArrayVec<Card, 24>, moves: &mut Vec<Move>) -> bool {
    if board.is_cleared() {
        return true;
    }
    if stock.is_empty() {
        return false;
    }

    let top_stock = *stock.last().unwrap();
    for pos in board.free_card_indices() {
        if !board.cards[pos].unwrap().is_sequential(&top_stock) {
            continue;
        }
        let mut this_board = board;
        let mut new_stock = stock.clone();

        let card = (&mut this_board.cards[pos]).take().unwrap();
        *new_stock.last_mut().unwrap() = card;
        moves.push(Move::Pyramid(card));
        if solve(this_board, new_stock, moves) {
            return true;
        }
        moves.pop();
    }

    let mut stock = stock;
    stock.pop().unwrap();
    if stock.is_empty() {
        return false;
    }
    moves.push(Move::Stock(*stock.last().unwrap()));
    if solve(board, stock, moves) {
        return true;
    }
    moves.pop();

    false
}

fn parse(state: impl Iterator<Item = String>) -> (TriPeaksBoard, ArrayVec<Card, 24>) {
    let mut seen = HashSet::<Card>::new();
    let mut cards = state
        .map(|x| {
            let card = x.parse().unwrap_or_else(|_| panic!("Invalid card {x}"));
            if seen.contains(&card) {
                panic!("Card {x} is duplicated");
            }
            seen.insert(card);
            card
        })
        .take(52);
    let board_cards: ArrayVec<_, 28> = cards.by_ref().take(28).map(Some).collect();
    let mut stock: ArrayVec<_, 24> = cards.collect();
    stock.reverse();
    assert!(stock.len() == 24, "Not enough cards provided");
    let board = TriPeaksBoard {
        cards: board_cards.into_inner().unwrap(),
    };
    (board, stock)
}

/// Solve a game of TriPeaks, emitting a progress bar for the top-level call.
fn solve_with_progress(
    board: TriPeaksBoard,
    stock: ArrayVec<Card, 24>,
    moves: &mut Vec<Move>,
) -> bool {
    if board.is_cleared() {
        return true;
    }
    if stock.is_empty() {
        return false;
    }

    let top_stock = *stock.last().unwrap();
    for (i, &pos) in board.free_card_indices().iter().enumerate() {
        eprint!("[{}{}]\r", "=".repeat(i), " ".repeat(10 - i));
        if !board.cards[pos].unwrap().is_sequential(&top_stock) {
            continue;
        }
        let mut this_board = board;
        let mut new_stock = stock.clone();

        let card = (&mut this_board.cards[pos]).take().unwrap();
        *new_stock.last_mut().unwrap() = card;
        moves.push(Move::Pyramid(card));
        if solve(this_board, new_stock, moves) {
            return true;
        }
        moves.pop();
    }

    let mut stock = stock;
    moves.push(Move::Stock(stock.pop().unwrap()));
    if solve_with_progress(board, stock, moves) {
        return true;
    }
    moves.pop();

    false
}

pub fn solve_with(state: impl Iterator<Item = String>) -> Option<Vec<Move>> {
    let (board, stock) = parse(state);
    let mut moves = Vec::new();
    if solve(board, stock, &mut moves) {
        Some(moves)
    } else {
        None
    }
}

fn main() {
    let cards = std::env::args().skip(1);

    let (board, stock) = parse(cards);
    let stock_str = stock
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{board}\n\n[{stock_str}]");
    let mut moves = Vec::new();
    if solve_with_progress(board, stock, &mut moves) {
        for r#move in moves {
            r#move.output();
        }
    } else {
        println!("No solution found");
    }
}
