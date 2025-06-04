use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};

pub mod bots;
pub mod state;

pub static BANNER: &[u8] = include_bytes!("../art/banner.txt");
pub static _EZ11: &[u8] = include_bytes!("../art/bots/ez11.txt");

#[derive(Debug, Default)]
pub enum CoolNumber {
    #[default]
    Not,

    Overflow,
    Underflow,
    Perfect,
    Square(u8),
    Cube(u8),
    HyperCube(u8, u8),
    // TODO: Can I source more interesting sequences? (See: https://oeis.org, Combo Class, etc)
}

impl CoolNumber {
    pub fn coolness(from: u8, to: u8) -> Self {
        use CoolNumber::*;

        match (from, to) {
            (0, 255) => Underflow,
            (255, 0) => Overflow,
            (_, 6) | (_, 28) => Perfect,
            // TODO: Pre-calculate?
            _ => match to {
                4 => Square(2),
                8 => Cube(2),
                16 => HyperCube(2, 4),
                32 => HyperCube(2, 5),
                64 => HyperCube(2, 6),
                128 => HyperCube(2, 7),

                9 => Square(3),
                27 => Cube(3),
                81 => HyperCube(3, 4),
                243 => HyperCube(3, 5),

                25 => Square(5),
                125 => Cube(5),

                36 => Square(6),
                216 => Cube(6),

                49 => Square(7),
                100 => Square(10),
                121 => Square(11),
                144 => Square(12),
                169 => Square(13),
                196 => Square(14),
                225 => Square(15),
                _ => Not,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    coolness: CoolNumber,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let banner_text = str::from_utf8(BANNER).unwrap();
        let banner_lines = banner_text.lines().count().try_into().unwrap();

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(banner_lines), Constraint::Min(0)])
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(15), Constraint::Min(0)])
            .split(outer_layout[1]);

        frame.render_widget(
            Paragraph::new(banner_text)
                .centered()
                .fg(Color::Blue)
                .block(Block::new()),
            outer_layout[0],
        );

        frame.render_widget(
            Paragraph::new(bots::CRATELIN.portrait).block(Block::new()),
            inner_layout[0],
        );

        frame.render_widget(self, inner_layout[1]);
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        use KeyCode::*;

        match key_event.code {
            Char('q') => self.exit(),
            Char('c') | Char('d') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.exit()
                }
            }
            Left => self.decrement_counter(),
            Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        let before = self.counter;
        self.counter += 1;

        self.coolness = CoolNumber::coolness(before, self.counter);
    }

    fn decrement_counter(&mut self) {
        let before = self.counter;
        self.counter -= 1;

        self.coolness = CoolNumber::coolness(before, self.counter);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" {} ", bots::CRATELIN.name).bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec![]),
            Line::from(vec!["Hey welcome to the chill zone.".into()]),
            Line::from(vec![]),
            Line::from(vec!["I'm Cratelin. Us bots hang out here and organize crates.".into()]),
            Line::from(vec![]),
            Line::from(vec![
                "We put stuff in places... give out copies. Sometimes we'll yank a crate. In terms of clock cycles we mostly just chill."
                    .into(),
            ]),
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![]),
            Line::from(vec![
                "There's not a lot to do here right now. We've all kinda been into counting recently."
                    .into(),
            ]),
            Line::from(vec![]),
            Line::from(vec!["Byte: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec![]),
            Line::from(match self.coolness {
                CoolNumber::Overflow => vec!["Sick overflow!!!".into()],
                CoolNumber::Underflow => vec!["lol underflow".into()],
                CoolNumber::Square(n) => vec![format!("Nice, that's {} squared", n).into()],
                CoolNumber::Cube(n) => vec![format!("Yo that's {} cubed", n).into()],
                CoolNumber::HyperCube(n, k) => vec![format!("Whoa, a {}-dimensional hyper cube of {}!", k, n).into()],
                CoolNumber::Perfect => vec!["ah... perfection".into()],
                CoolNumber::Not => vec![],
            }),
        ]);

        let main = Paragraph::new(counter_text).centered().block(block);

        main.render(area, buf);
    }
}
