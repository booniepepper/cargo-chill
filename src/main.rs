use ratatui::prelude::*;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

static BANNER: &[u8] = include_bytes!("../art/banner.txt");
static CRATELIN: &[u8] = include_bytes!("../art/bots/cratelin.txt");
static EZ11: &[u8] = include_bytes!("../art/bots/ez11.txt");

#[derive(Debug, Default)]
enum CoolNumber {
    Overflow,
    Underflow,
    Square(u8),
    Cube(u8),
    #[default]    Not,
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    coolness: CoolNumber,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let banner_text = str::from_utf8(&BANNER).unwrap();
        let banner_lines = banner_text.lines().count().try_into().unwrap();

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(banner_lines), Constraint::Min(0)])
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer_layout[1]);

        frame.render_widget(
            Paragraph::new(banner_text)
                .centered()
                .fg(Color::Blue)
                .block(Block::new()),
            outer_layout[0],
        );

        frame.render_widget(
            Paragraph::new(str::from_utf8(&CRATELIN).unwrap()).block(Block::new()),
            inner_layout[0],
        );

        frame.render_widget(self, inner_layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
        self.coolness = match self.counter {
            255 => CoolNumber::Overflow,
            _ => CoolNumber::Not,
        };
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.coolness = match self.counter {
            0 => CoolNumber::Underflow,
            _ => CoolNumber::Not,
        };
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Cratelin ".bold());
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
            Line::from(vec![
                "There will be more here later, but for now here's a counter. We're all kinda into counting right now."
                    .into(),
            ]),
            Line::from(vec![]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec![]),
            Line::from(match self.coolness {
                CoolNumber::Overflow => vec!["Sick overflow!!!".into()],
                CoolNumber::Underflow => vec!["lol underflow".into()],
                _ => vec![],
            }),
        ]);

        let main = Paragraph::new(counter_text).centered().block(block);

        main.render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
