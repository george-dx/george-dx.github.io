use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};
use ratzilla::{event::{KeyCode, KeyEvent}, DomBackend, WebRenderer};
use std::{cell::RefCell, io, rc::Rc};

fn main() -> io::Result<()> {
    // set up the "terminal" in the browser
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    // shared app state
    let app = Rc::new(App::default());

    // key handling
    {
        let app = Rc::clone(&app);
        terminal.on_key_event(move |ev| {
            app.handle_event(ev);
        });
    }

    // drawing
    terminal.draw_web(move |f| {
        app.render(f);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    tabs: Vec<&'static str>,
    current_tab: RefCell<usize>,
    scroll: RefCell<u16>,
}

impl App {
    fn default() -> Self {
        Self {
            tabs: vec![
                "ABOUT ME",
                "CONTRIBUTIONS",
                "TECH BLOG",
                "BOOKS REVIEW",
                "SOCIAL",
            ],
            current_tab: RefCell::new(0),
            scroll: RefCell::new(0),
        }
    }

    fn handle_event(&self, ev: KeyEvent) {
        match ev.code {
            KeyCode::Left => {
                let mut ct = self.current_tab.borrow_mut();
                *ct = if *ct == 0 { self.tabs.len() - 1 } else { *ct - 1 };
            }
            KeyCode::Right => {
                let mut ct = self.current_tab.borrow_mut();
                *ct = if *ct == self.tabs.len() - 1 { 0 } else { *ct + 1 };
            }
            KeyCode::Up => {
                let mut s = self.scroll.borrow_mut();
                if *s > 0 {
                    *s -= 1;
                }
            }
            KeyCode::Down => {
                let mut s = self.scroll.borrow_mut();
                *s = s.saturating_add(1);
            }
            // KeyCode::Esc => {
            //     ev.ctx().shutdown();
            // }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // Outer border
        let outer = Block::default().bg(Color::Rgb(7, 54, 66)).borders(Borders::ALL);
        frame.render_widget(outer, area);

        // Split full canvas into header/body/footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // header height
                Constraint::Min(0),    // body
                Constraint::Length(3), // footer height
            ])
            .split(area);
        let header_area = chunks[0];
        let body_area   = chunks[1];
        let footer_area = chunks[2];

        // Header
        // Insert the header box itself from header_area
        let header_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(95),
            ])
            .split(header_area);
        let header_box_area = header_cols[1];

        // Draw the header box
        let header_block = Block::default();
        frame.render_widget(&header_block, header_box_area);

        // Draw tabs inside that box
        let header_inner = header_block.inner(header_box_area);
        let ct = *self.current_tab.borrow();
        let tabs = Tabs::new(self.tabs.clone())
            .block(Block::default())
            .divider(Span::raw(" | "))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default().fg(Color::Rgb(181, 137, 0)).add_modifier(Modifier::BOLD),
            )
            .select(ct);
        frame.render_widget(tabs, header_inner);

        // Body placeholder
        frame.render_widget(Block::default(), body_area);

        // Footer
        // Insert the footer box itself from footer_area
        let footer_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(footer_area);
        let footer_box_area = footer_cols[1];

        // Draw the footer box
        let footer_block = Block::default().borders(Borders::ALL);
        frame.render_widget(&footer_block, footer_box_area);

        // Draw instructions inside that box
        let footer_inner = footer_block.inner(footer_box_area);
        let instructions = vec![Line::from(vec![
            Span::raw("Use "),
            Span::styled("left and right ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(133, 153, 0))),
            Span::raw("to switch tabs, "),
            Span::styled("up and down ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(133, 153, 0))),
            Span::raw("to scroll, "),
            Span::styled("enter ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(133, 153, 0))),
            Span::raw("to select, and "),
            Span::styled("esc ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(133, 153, 0))),
            Span::raw("to go back.")
        ])];
        let paragraph = Paragraph::new(
            instructions
        )
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, footer_inner);
    }


}
