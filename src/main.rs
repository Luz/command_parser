extern crate ncurses;
use ncurses::*;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cmd.pest"]
struct IdentParser;

fn main() {
    let mut command = String::new();
    let mut quitnow = false;

    initscr();

    while quitnow == false {
        let key = std::char::from_u32(getch() as u32).unwrap();
        addstr(&format!("   {:?}   ", key));
        command.push_str(&key.clone().to_string());

        let parsethisstring = command.clone();
        let commands = IdentParser::parse(Rule::cmd_list, &parsethisstring).unwrap();

        let mut clear = true;
        for cmd in commands {
            match cmd.as_rule() {
                Rule::down => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::up => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::left => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::right => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::start => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::end => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::bottom => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::replace => {
                    addstr("Next char will be the replacement!");
                    clear = false;
                }
                Rule::replacement => {
                    addstr(&format!("Replacement: {:?}", cmd.as_str()));
                    clear = true;
                }
                Rule::replaceend => {
                    addstr("Replacement canceled.");
                    clear = true;
                }
                Rule::remove => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                }
                Rule::dd => {
                    let amount: usize = cmd.as_str().parse().unwrap_or(1);
                    addstr(&format!("Delete {:?} lines", amount));
                    clear = true;
                }
                Rule::insert => {
                    addstr("next chars will be inserted!");
                    clear = false;
                }
                Rule::insertstuff => {
                    let last_char = command.chars().last().unwrap();
                    addstr(&format!("Just inserted: {:?}", last_char));
                    // Do not pop, keep it for history
                    clear = false;
                }
                Rule::insertend => {
                    addstr(&format!("Insert ended. ({:?})", command.clone()));
                    clear = true;
                }
                Rule::jumpascii => {
                    addstr(&format!("{:?}", cmd.as_rule()));
                    clear = true;
                }
                Rule::helpfile => {
                    command.pop();
                    addstr("No helpfile yet");
                }
                Rule::repeat => {
                    addstr("Repeating");
                    //clear = false;
                }
                Rule::searchend => (),
                Rule::hexsearchend => (),
                Rule::gg => {
                    let linenr: usize = cmd.as_str().parse().unwrap_or(0);
                    addstr(&format!("Jump to line: {:?}", linenr));
                    clear = true;
                }
                Rule::escape => {
                    addstr("Escape");
                }
                Rule::backspace => {
                    command.pop(); // Removes the backspace
                    command.pop(); // Removes the char before the backspace
                    clear = false;
                }
                Rule::saveandexit => {
                    addstr("Saving...");
                    quitnow = true;
                }
                Rule::exit => quitnow = true,
                Rule::save => {
                    addstr("Saving");
                }
                Rule::gatherall => {
                    clear = false;
                }
                _ => {
                    addstr(&format!("no rule for {:?} ", cmd.as_rule()));
                    clear = false;
                }
            }

            for inner in cmd.into_inner() {
                match inner.as_rule() {
                    Rule::searchstr => {
                        addstr(&format!("Searching for ascii: {:?}", inner.as_str()));
                    }
                    Rule::searchbytes => {
                        addstr(&format!("Searching for bytes: {:?}", inner.as_str()));
                    }
                    _ => {
                        addstr(&format!("no inner rule for {:?} ", inner.as_rule()));
                        clear = false;
                    }
                };
            }
            if clear {
                command.clear();
            } else {
                addstr(&format!(" {:?}", command));
            }
            addstr("\n");
            refresh();
        }
    }
    endwin();
}
