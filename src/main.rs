// Only me and god knows how this code still running

use std::{io, iter::successors, vec};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Menu,
    Detail,
    Category_screen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Main,
    Dessert,
}

struct Recipe {
    name: &'static str,
    description: &'static str,
    emoji: &'static str,
    food_category: Category,
}

fn main() -> Result<(), io::Error> {
    let recipes = vec![
        Recipe {
            name: "Pizza",
            description: "just a dough",
            emoji: "",
            food_category: Category::Main,
        },
        Recipe {
            name: "Ratatouille",
            description: "idunno either",
            emoji: "🥘",
            food_category: Category::Main,
        },
    ];
    let categorys = vec!["Main", "Dessert"];
    let mut recipes_status = ListState::default();
    let mut category_status = ListState::default();
    let mut screen = Screen::Category_screen;
    recipes_status.select(Some(0));
    category_status.select(Some(0));
    let mut current_category = categorys[category_status.selected().unwrap_or(0)];
    let mut is_category_main = true;
    let mut is_category_dessert = false;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| match screen {
            Screen::Menu => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((3)),
                        Constraint::Length((1)),
                        Constraint::Min((10)),
                    ])
                    .split(f.area());

                let welcome = Paragraph::new("🦀 WELCOME TO THE RUSTY CHEF!!! 🦀")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red))
                    .block(Block::default().borders(Borders::ALL));

                let items: Vec<ListItem> = recipes
                    .iter()
                    .filter(|r| {
                        if is_category_main {
                            r.food_category == Category::Main
                        } else {
                            r.food_category == Category::Dessert
                        }
                    })
                    .map(|recipe| ListItem::new(recipe.name))
                    .collect();

                let list_widget = List::new(items)
                    .block(
                        Block::default()
                            .title("Select a recipe pls")
                            .borders(Borders::ALL),
                    )
                    .highlight_symbol(">> ")
                    .highlight_style(
                        ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                    );

                f.render_widget(welcome, chunks[0]);
                f.render_stateful_widget(list_widget, chunks[2], &mut recipes_status)
            }
            Screen::Detail => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((3)),
                        Constraint::Length((1)),
                        Constraint::Min((10)),
                    ])
                    .split(f.area());

                let selected = recipes_status.selected().unwrap_or(0);
                let food_recipe = Paragraph::new(recipes[selected].description)
                    .block(
                        Block::default()
                            .border_style(Style::default().fg(Color::Cyan))
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));
                let welcome_recipe =
                    Paragraph::new(format!("FOOD RECIPE {}", recipes[selected].emoji))
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(Color::Yellow))
                        .block(Block::default().borders(Borders::ALL));

                f.render_widget(welcome_recipe, chunks[0]);
                f.render_widget(food_recipe, chunks[2]);
            }
            Screen::Category_screen => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((3)),
                        Constraint::Length((1)),
                        Constraint::Min((10)),
                    ])
                    .split(f.area());

                let category_welcome = Paragraph::new("FOOD CATEGORYS")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Magenta))
                    .block(Block::default().borders(Borders::ALL));

                let category_name: Vec<ListItem> = categorys
                    .iter()
                    .map(|category| ListItem::new(*category))
                    .collect();
                let list_category_widget = List::new(category_name)
                    .block(
                        Block::default()
                            .title("Select a category pls")
                            .borders(Borders::ALL),
                    )
                    .highlight_symbol(">> ")
                    .highlight_style(
                        ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                    );

                f.render_widget(category_welcome, chunks[0]);
                f.render_stateful_widget(list_category_widget, chunks[2], &mut category_status);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(300))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            break;
                        }
                        KeyCode::Down => {
                            if screen == Screen::Menu {
                                let i = match recipes_status.selected() {
                                    Some(i) => {
                                        if i >= recipes.len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                recipes_status.select(Some(i));
                            }
                            if screen == Screen::Category_screen {
                                let i = match category_status.selected() {
                                    Some(i) => {
                                        if i >= categorys.len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                category_status.select(Some(i));
                            }
                        }
                        KeyCode::Up => {
                            if screen == Screen::Menu {
                                let i = match recipes_status.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            recipes.len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                recipes_status.select(Some(i));
                            }
                            if screen == Screen::Category_screen {
                                let i = match category_status.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            categorys.len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                category_status.select(Some(i));
                            }
                        }
                        KeyCode::Enter => {
                            if screen == Screen::Category_screen {
                                current_category =
                                    categorys[category_status.selected().unwrap_or(0)];
                                if current_category == "Main" {
                                    if is_category_main == false {
                                        is_category_main = true;
                                    }
                                    if is_category_dessert == true {
                                        is_category_dessert = false;
                                    }
                                } else if current_category == "Dessert" {
                                    if is_category_main == true {
                                        is_category_main = false;
                                    }
                                    if is_category_dessert == false {
                                        is_category_dessert = true;
                                    }
                                }
                                screen = Screen::Menu;
                            } else {
                                screen = Screen::Detail;
                            }
                        }
                        KeyCode::Backspace => {
                            if screen == Screen::Category_screen {
                            } else if screen == Screen::Menu {
                                screen = Screen::Category_screen;
                            } else if screen == Screen::Detail {
                                screen = Screen::Menu
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
