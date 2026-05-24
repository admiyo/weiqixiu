use iced::{
    Alignment, Color, Length, Point, Rectangle, Renderer, Theme, Vector, mouse,
    widget::{
        Canvas,
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
        let start_x: f32 = 212.0;
        let start_y: f32 = 75.0;

        const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);
        const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);

        for i in (0..BOARD_SIZE).step_by(2) {
            for j in (0..BOARD_SIZE).step_by(3) {
                let color: Color = { if i < 10 { WHITE } else { BLACK } };

                let pos: Point = Point::new(
                    start_x + INCREMENT * (i as f32),
                    start_y + INCREMENT * (j as f32),
                );
                frame.fill(&Path::circle(pos, PIECE_RADIUS), color);
            }
        }
        vec![frame.into_geometry()]
    }
}
