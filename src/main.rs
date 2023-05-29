use std::io::{self, BufRead};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use regex::Regex;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
struct App<> {
 list: ListState,
}
impl App {
    pub fn new() -> Self {

        App { list: ListState::default()}
    }


}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {

    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
        }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }}
}
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn hello() {
    let stdin = io::stdin();
    let paths: Vec<String> = stdin.lock().lines().filter_map(|x| x.ok()).collect();
    println!("{:?}",paths);

    let results: Vec<MatchResult> = paths.iter().map(|line| parse(line)).collect();
    println!("{:?}", results);
}
#[derive(Debug)]
enum Matched {
    Matched,
    Unmatched 
}
#[derive(Debug)]
struct MatchResult {
     result: Matched,
     line: String,
}


fn parse(line: &str) -> MatchResult {
       let re = Regex::new(r"(/?([a-z.A-Z0-9\-_]+/)+[@a-zA-Z0-9\-_+.]+\.[a-zA-Z0-9]{1,10})[:-]?(\d+)?").unwrap();

       if re.is_match(line) {
           return MatchResult { result: Matched::Matched, line:line.to_string() }
       }



       let re = Regex::new(r"([@%+a-z.A-Z0-9\-_]+\.[a-zA-Z]{1,10})(\s|$|:)+").unwrap();

       if re.is_match(line) {
          return MatchResult{result:Matched::Matched, line:line.to_string()}
        }
       return MatchResult{result:Matched::Unmatched, line:line.to_string()}

}


fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
}
