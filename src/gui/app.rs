#![cfg(feature = "ratatui")]
use ratatui::{DefaultTerminal, Frame};

pub(crate) fn tui(terminal :&mut DefaultTerminal) -> std::io::Result<()>{
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press(){

        }
    }
}

pub(crate) fn render(frame:&mut Frame){

}