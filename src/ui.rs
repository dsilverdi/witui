use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, Clear, Paragraph}, Frame
};

use crate::app::{App, AppState};
use crate::constant::TITLE;

pub fn render(frame: &mut Frame, app: &App) {
    let main_area = frame.area();
    let chunks = Layout::default()
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(main_area);
    render_menu_ui(frame, app, chunks[0]);

    if app.state == AppState::Search {
        render_search_popup(frame, app);
    }

    render_footer(frame, app, chunks[1]);
}

pub fn render_menu_ui(frame: &mut Frame, app: &App, area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(3, 5)
            ]
        ).split(area);

    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout[0]);

    let text: Vec<Line<'_>> = vec![Line::from("Wikipedia on your terminal made in 🦀")];
    let sub_title = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout[1]);
}

// fn render_link_article_ui(frame: &mut Frame, app: &App) {

// }

// popup for search fields
fn render_search_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());
    frame.render_widget(Clear, area);
    
    // Create the outer block
    let outer_block = Block::default()
        .title("Search")
        .borders(Borders::ALL);
        // .style(Style::default().bg(Color::DarkGray));
    
    frame.render_widget(outer_block, area);

    // Create a layout for the inner area
    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints( [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3)
        ])
        .margin(2)  // Add margin to separate from outer border
        .split(area);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    // Render the input widget
    frame.render_widget(input, inner_area[1]);

    // Move the cursor to the input line
    frame.set_cursor_position(
        Position{
            x: inner_area[1].x + app.input.len() as u16 + 1,
            y: inner_area[1].y + 1,
        }
    )
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let footer_text: Vec<Span<'_>> = get_footer_text(app);
    let footer = Paragraph::new(Line::from(footer_text))
        // .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}

fn get_footer_text(app: &App) -> Vec<Span<'_>> {
    if app.state == AppState::Search {
        return vec![
            Span::styled("Enter:", Style::default().fg(Color::Yellow)),
            Span::raw(" Search | "),
            Span::styled("esc:", Style::default().fg(Color::Yellow)),
            Span::raw(" Close Search"),
        ]
    }

    vec![
        Span::styled("Ctrl + `s`:", Style::default().fg(Color::Yellow)),
        Span::raw(" Search | "),
        Span::styled("q:", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit"),
    ]
}

// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
