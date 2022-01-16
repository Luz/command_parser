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
        printw(&format!("   {:?}   ", key));
        command.push_str(&key.clone().to_string());

        let parsethisstring = command.clone();
        let commands = IdentParser::parse(Rule::cmd_list, &parsethisstring)
            .unwrap_or_else(|e| panic!("{}", e));

        let mut clear = true;
        for cmd in commands {
            match cmd.as_rule() {
                Rule::down => {
                    printw(&format!("{:?}", cmd.as_rule()));
                }
                Rule::quickstuff => {
                    printw("quickstuff: ");
                }
                Rule::up => {
                    printw(&format!("{:?}", cmd.as_rule()));
                }
                Rule::left => {
                    printw(&format!("{:?}", cmd.as_rule()));
                }
                Rule::right => {
                    printw(&format!("{:?}", cmd.as_rule()));
                }
                Rule::start => (),
                Rule::end => (),
                Rule::bottom => {
                    printw(&format!("{:?}", cmd.as_rule()));
                }
                Rule::replace => {
//                    printw("next char will be the replacement!");
                    clear = false;
                }
                Rule::remove => (),
                Rule::insert => {
                    printw("next chars will be inserted!");
                    clear = false;
                }
                Rule::jumpascii => (),
                Rule::helpfile => (),
                Rule::search => (),
                Rule::escape => (),
                Rule::repeat => {
                    printw("Repeating");
                    //clear = false;
                }
                Rule::backspace => {
                    command.pop();
                    command.pop();
                    clear = false;
                },
                Rule::saveandexit => {
                    printw("Saving...");
                    quitnow = true;
                }
                Rule::exit => quitnow = true,
                Rule::save => {
                    printw("Saving");
                }
                _ => (),
            }

            for inner_cmd in cmd.into_inner() {
                match inner_cmd.as_rule() {
                    Rule::replacement => {
                        printw(&format!("Replacement: {:?}", inner_cmd.as_str() ));
                    }
                    Rule::dd_lines => {
                        let amount: usize = inner_cmd.as_str().parse().unwrap_or(1);
                        printw(&format!("Delete {:?} lines", amount ));
                    }
                    Rule::insertment => {
                        printw(&format!("Inserted: {:?}", inner_cmd.as_str()));
                        command.pop(); // remove the just inserted thing
                        clear = false;
                    }
                    Rule::searchstr => {
                        printw(&format!("Searching for ascii: {:?}", inner_cmd.as_str()));
                    }
                    Rule::searchbytes => {
                        printw(&format!("Searching for bytes: {:?}", inner_cmd.as_str()));
                    }
                    Rule::gg_line => {
                        let linenr: usize = inner_cmd.as_str().parse().unwrap_or(0);
                        printw(&format!("Jump to line: {:?}", linenr));
                    }
                    Rule::gatherone => clear = false,
                    _ => {
                        printw(&format!("no rule for {:?} ", inner_cmd.as_rule()));
                        clear = false;
                    }
                };
            }
            if clear {
                command.clear();
            } else {
                printw(&format!(" {:?}", command));
            }
            printw("\n");
            refresh();
        }
    }
    endwin();
}
