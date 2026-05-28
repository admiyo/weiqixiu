use array2d::Array2D;
use iced::{
    Alignment, Color, Length, Pixels, Point, Rectangle, Renderer, Theme, Vector, mouse,
    widget::{
        Canvas, canvas,
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column,
    },
};
use std::collections::HashSet;
use std::fmt;

const BOARD_SPAN: f32 = 300.0;
const BOARD_SIZE: usize = 19;
const PIECE_RADIUS: f32 = BOARD_SPAN / ((BOARD_SIZE as f32) - 1.0);
const INCREMENT: f32 = BOARD_SPAN / ((BOARD_SIZE as f32) - 1.0) * 2.0;

fn main() -> iced::Result {
    iced::run(WeiQiXiu::update, WeiQiXiu::view)
}

#[derive(Clone, Debug)]
enum Message {}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum Piece {
    #[default]
    NONE,
    BLACK,
    WHITE,
}

fn opposite(piece_1: Piece, piece_2: Piece) -> bool {
    let mut val: bool = true;
    if piece_1 == Piece::NONE || piece_2 == Piece::NONE {
        val = false;
    }
    if piece_1 == piece_2 {
        val = false;
    }
    val
}

impl From<char> for Piece {
    fn from(c: char) -> Piece {
        match c {
            'B' => Piece::BLACK,
            'W' => Piece::WHITE,
            _ => Piece::NONE,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: char = {
            match *self {
                Piece::NONE => '┼',
                Piece::WHITE => '●',
                Piece::BLACK => '○',
            }
        };
        write!(f, "{}", c)
    }
}

/*#[derive(Default)]*/
struct WeiQiXiu {
    board: Array2D<Piece>,
}

impl fmt::Display for WeiQiXiu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row_iter in self.board.rows_iter() {
            for element in row_iter {
                write!(f, "{} ", element).ok();
            }
            writeln!(f).ok();
        }
        Ok(())
    }
}

impl WeiQiXiu {
    fn _check_group(&mut self, piece: Piece, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut check_stack: Vec<(usize, usize)> = Vec::new();
        let mut capture_set: HashSet<(usize, usize)> = HashSet::new();
        let mut liberties = 0;

        check_stack.push((x, y));
        while !check_stack.is_empty() && liberties == 0 {
            let current = check_stack.pop().unwrap();
            for tan in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x_c: i64 = current.0 as i64 + tan.0;
                let y_c: i64 = current.1 as i64 + tan.1;
                if x_c >= 0 && y_c >= 0 && x_c < BOARD_SIZE as i64 || y_c < BOARD_SIZE as i64 {
                    let test_space = (x_c as usize, y_c as usize);

                    if self.board[test_space] == piece {
                        if !capture_set.contains(&test_space) {
                            check_stack.push(test_space)
                        }
                    } else if self.board[test_space] == Piece::NONE {
                        liberties += 1;
                        break;
                    }
                }
            }
            if liberties == 0 {
                capture_set.insert(current);
            }
        }
        if liberties > 0 {
            println!("clearing");
            capture_set.clear();
        }

        capture_set
    }

    fn add_piece(&mut self, piece: Piece, x: usize, y: usize) {
        self.board[(x, y)] = piece;
        for tan in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x_c: i64 = x as i64 + tan.0;
            let y_c: i64 = y as i64 + tan.1;
            if x_c >= 0 && y_c >= 0 && x_c < BOARD_SIZE as i64 || y_c < BOARD_SIZE as i64 {
                let current = (x_c as usize, y_c as usize);
                if opposite(self.board[current], piece) {
                    //TODO return the group from this function and remove all pieces in it from board
                    let capture_group: HashSet<(usize, usize)> =
                        self._check_group(self.board[current], current.0, current.1);

                    if capture_group.len() > 0 {
                        println!("removing");
                    }

                    for i in capture_group.iter() {
                        self.board[*i] = Piece::NONE;
                    }
                }
            }
        }
        //TODO look for suicide moves
        //TODO detect  Ko
        println!("{}", self);
    }
}

fn pos_from_char(c: char) -> usize {
    (c as u32 - 'a' as u32) as usize
}

impl Default for WeiQiXiu {
    fn default() -> Self {
        let mut wei_qi_xiu = WeiQiXiu {
            board: Array2D::filled_with(Piece::NONE, BOARD_SIZE as usize, BOARD_SIZE as usize),
        };
        for next_move in &MOVES {
            wei_qi_xiu.add_piece(
                Piece::from(next_move.0),
                pos_from_char(next_move.2),
                pos_from_char(next_move.1),
            );
        }
        wei_qi_xiu
    }
}

impl WeiQiXiu {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            "围戏锈",
            Canvas::new(WeiQiProgram)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

struct WeiQiProgram;

impl<Message> Program<Message> for WeiQiProgram {
    type State = WeiQiXiu;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb(0.803, 0.666, 0.490),
        );

        for i in 0..BOARD_SIZE {
            frame.stroke(
                &Path::line(
                    frame.center() + Vector::new(-BOARD_SPAN + INCREMENT * (i as f32), BOARD_SPAN),
                    frame.center() + Vector::new(-BOARD_SPAN + INCREMENT * (i as f32), -BOARD_SPAN),
                ),
                Stroke {
                    style: Color::BLACK.into(),
                    width: 1.0,
                    ..Default::default()
                },
            );
            frame.stroke(
                &Path::line(
                    frame.center() + Vector::new(-BOARD_SPAN, -BOARD_SPAN + INCREMENT * (i as f32)),
                    frame.center() + Vector::new(BOARD_SPAN, -BOARD_SPAN + INCREMENT * (i as f32)),
                ),
                Stroke {
                    style: Color::BLACK.into(),
                    width: 1.0,
                    ..Default::default()
                },
            );
        }

        let start_x: f32 = frame.center().x - BOARD_SPAN;
        let start_y: f32 = frame.center().y - BOARD_SPAN;

        for (y, row_iter) in state.board.rows_iter().enumerate() {
            for (x, element) in row_iter.enumerate() {
                if *element == Piece::NONE {
                    continue;
                }

                let color: Color = {
                    if *element == Piece::WHITE {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    }
                };
                let pos: Point = pos_to_point(start_x, start_y, x, y);
                frame.fill(&Path::circle(pos, PIECE_RADIUS), color);
            }
        }

        for i in 0..(BOARD_SIZE as u32) {
            let my_text = canvas::Text {
                content: char::from_u32(i + 'a' as u32).unwrap().to_string(),
                position: Point::new(start_x + INCREMENT * (i as f32), start_y - INCREMENT),
                color: Color::BLACK,
                size: Pixels(16.0),
                ..canvas::Text::default() // 3. Use default fallback values
            };
            frame.fill_text(my_text);
        }

        for i in 0..BOARD_SIZE as u32 {
            let my_text = canvas::Text {
                content: (19 - i).to_string(),
                position: Point::new(
                    start_x - INCREMENT,
                    (start_y - INCREMENT / 4.0) + INCREMENT * (i as f32),
                ),
                color: Color::BLACK,
                size: Pixels(16.0),
                ..canvas::Text::default() // 3. Use default fallback values
            };
            frame.fill_text(my_text);
        }

        // 3. Commit the text configuration directly into the frame layer

        vec![frame.into_geometry()]
    }
}

fn pos_to_point(start_x: f32, start_y: f32, row: usize, col: usize) -> Point {
    let x: f32 = {
        if row > BOARD_SIZE {
            0.0
        } else {
            INCREMENT * (row as u32) as f32 + start_x
        }
    };

    let y: f32 = {
        if col > BOARD_SIZE {
            0.0
        } else {
            INCREMENT * (col as u32) as f32 + start_y
        }
    };
    Point::new(x, y)
}

const MOVES_1: [(char, char, char); 5] = [
    ('B', 'j', 'd'),
    ('W', 'j', 'e'),
    ('B', 'k', 'e'),
    ('B', 'i', 'e'),
    ('B', 'j', 'f'),
];

const MOVES: [(char, char, char); 132] = [
    ('B', 'p', 'p'),
    ('W', 'c', 'd'),
    ('B', 'd', 'q'),
    ('W', 'p', 'd'),
    ('B', 'c', 'o'),
    ('W', 'q', 'n'),
    ('B', 'n', 'q'),
    ('W', 'r', 'p'),
    ('B', 'q', 'q'),
    ('W', 'f', 'c'),
    ('B', 'n', 'c'),
    ('W', 'p', 'c'),
    ('B', 'k', 'c'),
    ('W', 'c', 'j'),
    ('B', 'p', 'j'),
    ('W', 'q', 'k'),
    ('B', 'p', 'g'),
    ('W', 'q', 'f'),
    ('B', 'q', 'g'),
    ('W', 'r', 'f'),
    ('B', 'r', 'g'),
    ('W', 'r', 'i'),
    ('B', 'q', 'i'),
    ('W', 'n', 'e'),
    ('B', 'h', 'c'),
    ('W', 'i', 'q'),
    ('B', 'g', 'q'),
    ('W', 'l', 'q'),
    ('B', 'c', 'l'),
    ('W', 'f', 'j'),
    ('B', 'h', 'l'),
    ('W', 'h', 'n'),
    ('B', 'i', 'o'),
    ('W', 'h', 'p'),
    ('B', 'h', 'q'),
    ('W', 'j', 'p'),
    ('B', 'i', 'p'),
    ('W', 'h', 'o'),
    ('B', 'i', 'n'),
    ('W', 'j', 'q'),
    ('B', 'h', 'm'),
    ('W', 'f', 'm'),
    ('B', 'g', 'n'),
    ('W', 'f', 'n'),
    ('B', 'f', 'o'),
    ('W', 'g', 'm'),
    ('B', 'g', 'o'),
    ('W', 'm', 'n'),
    ('B', 'c', 'g'),
    ('W', 'd', 'h'),
    ('B', 'd', 'g'),
    ('W', 'e', 'f'),
    ('B', 'e', 'g'),
    ('W', 'f', 'g'),
    ('B', 'e', 'h'),
    ('W', 'd', 'i'),
    ('B', 'f', 'h'),
    ('W', 'g', 'g'),
    ('B', 'g', 'i'),
    ('W', 'h', 'j'),
    ('B', 'g', 'j'),
    ('W', 'g', 'k'),
    ('B', 'h', 'k'),
    ('W', 'f', 'k'),
    ('B', 'h', 'i'),
    ('W', 'i', 'j'),
    ('B', 'k', 'l'),
    ('W', 'k', 'j'),
    ('B', 'm', 'k'),
    ('W', 'k', 'g'),
    ('B', 'j', 'i'),
    ('W', 'j', 'j'),
    ('B', 'k', 'h'),
    ('W', 'i', 'i'),
    ('B', 'i', 'h'),
    ('W', 'g', 'h'),
    ('B', 'l', 'h'),
    ('W', 'h', 'h'),
    ('B', 'f', 'i'),
    ('W', 'i', 'g'),
    ('B', 'j', 'h'),
    ('W', 'm', 'j'),
    ('B', 'j', 'g'),
    ('W', 'n', 'h'),
    ('B', 'm', 'i'),
    ('W', 'n', 'j'),
    ('B', 'l', 'j'),
    ('W', 'l', 'k'),
    ('B', 'l', 'i'),
    ('W', 'm', 'l'),
    ('B', 'k', 'k'),
    ('W', 'n', 'k'),
    ('B', 'n', 'i'),
    ('W', 'l', 'g'),
    ('B', 'j', 'f'),
    ('W', 'i', 'f'),
    ('B', 'i', 'e'),
    ('W', 'h', 'f'),
    ('B', 'g', 'f'),
    ('W', 'g', 'e'),
    ('B', 'm', 'f'),
    ('W', 'm', 'g'),
    ('B', 'n', 'f'),
    ('W', 'o', 'f'),
    ('B', 'n', 'g'),
    ('W', 'j', 'e'),
    ('B', 'h', 'e'),
    ('W', 'f', 'f'),
    ('B', 'm', 'e'),
    ('W', 'j', 'm'),
    ('B', 'l', 'l'),
    ('W', 'l', 'm'),
    ('B', 'j', 'l'),
    ('W', 'i', 'm'),
    ('B', 'i', 'l'),
    ('W', 'j', 'n'),
    ('B', 'g', 'p'),
    ('W', 'k', 'f'),
    ('B', 'k', 'e'),
    ('W', 'j', 'd'),
    ('B', 'i', 'c'),
    ('W', 'k', 'd'),
    ('B', 'l', 'e'),
    ('W', 'o', 'i'),
    ('B', 'o', 'h'),
    ('W', 'k', 'i'),
    ('B', 'b', 'h'),
    ('W', 'm', 'h'),
    ('B', 'p', 'f'),
    ('W', 'p', 'e'),
    ('B', 'd', 'k'),
    ('W', 'e', 'j'),
];
