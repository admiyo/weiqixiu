use iced::{
    Alignment, Color, Length, Pixels, Point, Rectangle, Renderer, Theme, Vector, mouse,
    widget::{
        Canvas, canvas,
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column,
    },
};

fn main() -> iced::Result {
    iced::run(WeiQiXiu::update, WeiQiXiu::view)
}

#[derive(Clone, Debug)]
enum Message {}

#[derive(Default)]
struct WeiQiXiu;

impl WeiQiXiu {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            "围戏锈",
            Canvas::new(MyProgram)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

const BOARD_SPAN: f32 = 300.0;
const BOARD_SIZE: u32 = 19;
const PIECE_RADIUS: f32 = BOARD_SPAN / ((BOARD_SIZE as f32) - 1.0);
const INCREMENT: f32 = BOARD_SPAN / ((BOARD_SIZE as f32) - 1.0) * 2.0;

struct MyProgram;

impl<Message> Program<Message> for MyProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
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

        for next_move in &MOVES {
            let color: Color = {
                if next_move.0 == 'W' {
                    Color::WHITE
                } else {
                    Color::BLACK
                }
            };
            let pos: Point = pos_to_point(next_move.1, next_move.2);
            frame.fill(&Path::circle(pos, PIECE_RADIUS), color);
        }

        for i in 0..BOARD_SIZE {
            let my_text = canvas::Text {
                content: char::from_u32(i + 65).unwrap().to_string(),
                position: Point::new(206.0 + INCREMENT * (i as f32), 25.0),
                color: Color::BLACK,
                size: Pixels(16.0),
                ..canvas::Text::default() // 3. Use default fallback values
            };
            frame.fill_text(my_text);
        }

        for i in 0..BOARD_SIZE {
            let my_text = canvas::Text {
                content: (i + 1).to_string(),
                position: Point::new(170.0, 65.0 + INCREMENT * (i as f32)),
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

fn pos_to_point(row: char, col: char) -> Point {
    let start_x: f32 = 212.0;
    let start_y: f32 = 75.0;
    let base = 'a' as u32;

    let y: f32 = {
        if row < 'a' || row > 'z' {
            0.0
        } else {
            INCREMENT * (row as u32 - base) as f32 + start_y
        }
    };

    let x: f32 = {
        if col < 'a' || col > 'z' {
            0.0
        } else {
            INCREMENT * (col as u32 - base) as f32 + start_x
        }
    };
    Point::new(x, y)
}

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
