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
    let mut keystring = String::new();
    let mut quitnow = false;

    initscr();

    while quitnow == false {
        let key = std::char::from_u32(getch() as u32).unwrap();
        printw(&format!("   {:?}   ", key));
        keystring.push_str(&key.clone().to_string());

        let parsethisstring = keystring.clone();
        let commands = IdentParser::parse(Rule::cmd_list, &parsethisstring)
            .unwrap_or_else(|e| panic!("{}", e));

        for cmd in commands {
            match cmd.as_rule() {
                Rule::movement => {
                    printw("movement: ");
                }
                Rule::quickstuff => {
                    printw("quickstuff: ");
                }
                _ => (),
            }

            let mut clear = true;
            for inner_cmd in cmd.into_inner() {
                match inner_cmd.as_rule() {
                    Rule::down => {
                        printw(&format!("{:?}", inner_cmd.as_rule()));
                    }
                    Rule::up => {
                        printw(&format!("{:?}", inner_cmd.as_rule()));
                    }
                    Rule::left => {
                        printw(&format!("{:?}", inner_cmd.as_rule()));
                    }
                    Rule::right => {
                        printw(&format!("{:?}", inner_cmd.as_rule()));
                    }
                    Rule::start => (),
                    Rule::end => (),
                    Rule::replace => (),
                    Rule::remove => (),
                    Rule::insert => (),
                    Rule::jumpascii => (),
                    Rule::helpfile => (),
                    Rule::search => (),
                    Rule::saveandexit => {
                        printw("Saving...");
                        quitnow = true;
                    }
                    Rule::exit => quitnow = true,
                    Rule::save => {
                        printw("Saving");
                    }
                    Rule::escape => {
                        printw("hi");
                    }
                    Rule::not_escape => {
                        printw("why");
                    }
                    Rule::gatherall => clear = false,
                    Rule::gatherone => clear = false,
                    _ => {
                        printw(&format!(
                            "no inner rule! {:?}",
                            inner_cmd.clone().into_span().as_str()
                        ));
                        clear = false;
                    }
                };
            }
            if clear {
                keystring.clear();
            } else {
                printw(&format!(" {:?}", keystring));
            }
            printw("\n");
            refresh();
        }
    }
    endwin();
}
