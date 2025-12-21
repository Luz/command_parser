extern crate crossterm;
use crossterm::event::{read, Event};
use crossterm::{
    cursor, queue,
    style::Print,
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

mod keycodes;
use keycodes::extract;

use std::io::prelude::*;
use std::io::stdout;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cmd.pest"]
struct CmdParser;

fn main() -> Result<()> {
    let mut command = String::new();
    let mut autoparse = String::new();

    let mut out = stdout();

    let screensize = crossterm::terminal::size()?;
    let screenheight = screensize.1;

    queue!(out, terminal::Clear(terminal::ClearType::All))?;
    queue!(out, cursor::MoveTo(0, 0))?;
    queue!(out, Print("Screenheight is ".to_string()))?;
    queue!(out, Print(screenheight.to_string()))?;
    queue!(out, Print("\n\r"))?;
    out.flush()?;

    enable_raw_mode()?;

    let mut quitnow = false;
    while quitnow == false {
        if autoparse.is_empty() {
            let key = read()?;
            let mut keycode: char = '\u{00}';
            // This is close to the old c-style 'getch()':
            match key {
                Event::Key(event) => {
                    keycode = extract(event.code).unwrap_or('\u{00}');
                }
                Event::Mouse(_event) => (), // This can be handled later
                Event::FocusGained => (),   // This can be handled later
                Event::FocusLost => (),     // This can be handled later
                Event::Paste(_text) => (),  // This can be handled later
                Event::Resize(_width, _height) => (), // This can be handled later
            };
            command.push_str(&keycode.clone().to_string());
        } else {
            command.push(autoparse.chars().nth(0).unwrap());
            autoparse.remove(0);
        }

        //        command.push_str(&key.clone().to_string());

        let parsethisstring = command.clone();
        let cmd = CmdParser::parse(Rule::cmd_list, &parsethisstring)
            .unwrap()
            .next()
            .unwrap();

        let mut clear = true;

        match cmd.as_rule() {
            Rule::down => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                queue!(out, Print(&format!("{} {:?}", amount, cmd.as_rule())))?;
            }
            Rule::up => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                queue!(out, Print(&format!("{} {:?}", amount, cmd.as_rule())))?;
            }
            Rule::left => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                queue!(out, Print(&format!("{:?} left", amount)))?;
            }
            Rule::right => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                queue!(out, Print(&format!("{:?} right", amount)))?;
            }
            Rule::start => {
                queue!(out, Print(&format!("{:?}", cmd.as_rule())))?;
            }
            Rule::end => {
                queue!(out, Print(&format!("{:?}", cmd.as_rule())))?;
            }
            Rule::bottom => {
                let linenr: usize = cmd.as_str().parse().unwrap_or(0);
                let text: String = format!("Jump to line: {:?}", linenr);
                queue!(out, Print(text))?;
            }
            Rule::top => {
                let linenr: usize = cmd.as_str().parse().unwrap_or(0);
                let text: String = format!("Jump to line: {:?}", linenr);
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::replace => {
                let text = "Next char will be the replacement!";
                queue!(out, Print(text))?;
                clear = false;
            }
            Rule::replacement => {
                let k = command.chars().last().unwrap_or('x');
                let text = format!("Replacement: {:?}", k);
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::remove => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                let text: String = format!("Remove {:?} chars", amount);
                queue!(out, Print(text))?;
            }
            Rule::remove_left => {
                let amount: usize = cmd.into_inner().as_str().parse().unwrap_or(1);
                let text: String = format!("Remove {:?} chars to the left", amount);
                queue!(out, Print(text))?;
            }
            Rule::remove_right => {
                let amount: usize = cmd.into_inner().as_str().parse().unwrap_or(1);
                let text: String = format!("Remove {:?} chars to the right", amount);
                queue!(out, Print(text))?;
            }
            Rule::dd => {
                let amount: usize = cmd.as_str().parse().unwrap_or(1);
                let text: String = format!("Delete {:?} lines", amount);
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::bigd => {
                queue!(out, Print("Delete till end of line"))?;
                clear = true;
            }
            Rule::insert => {
                queue!(out, Print("next chars will be inserted!"))?;
                clear = false;
            }
            Rule::insertstuff => {
                let last_char = command.chars().last().unwrap();
                let text = format!("Just inserted: {:?}", last_char);
                queue!(out, Print(text))?;
                // Do not pop, keep it for history
                clear = false;
            }
            Rule::insertend => {
                let text: String = format!("Insert ended. ({:?})", command.clone());
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::jumpascii => {
                queue!(out, Print(&format!("{:?}", cmd.as_rule())))?;
                clear = true;
            }
            Rule::querry => {
                command.pop();
                queue!(out, Print("no querry yet"))?;
            }
            Rule::repeat => {
                queue!(out, Print("Repeating"))?;
                //clear = false;
            }
            Rule::searchend => {
                let searchstr = cmd.clone().into_inner().as_str();
                let text: String = format!("Searching for ascii: {:?}", searchstr);
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::hexsearchend => {
                let searchbytes = cmd.clone().into_inner().as_str();
                let text: String = format!("Searching for bytes: {:?}", searchbytes);
                queue!(out, Print(text))?;
                clear = true;
            }
            Rule::escape => {
                queue!(out, Print("Escape"))?;
            }
            Rule::backspace => {
                command.pop(); // Removes the backspace
                command.pop(); // Removes the char before the backspace
                clear = false;
            }
            Rule::saveandexit => {
                queue!(out, Print("Saving..."))?;
                quitnow = true;
            }
            Rule::exit => quitnow = true,
            Rule::save => {
                queue!(out, Print("Saving..."))?;
            }
            Rule::gatherall => {
                clear = false;
            }
            _ => {
                let text: String = format!("Forgot to make the rule for {:?}", cmd.as_rule());
                queue!(out, Print(text))?;
                clear = true;
            }
        }
        if clear {
            command.clear();
        } else {
            queue!(out, Print(command.clone()))?;
        }

        // Make a newline and go to the start of the line
        queue!(out, Print("\n\r"))?;
        // queue!(out, cursor::MoveTo(0, 0))?;
        out.flush()?;
    }
    queue!(out, terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}
