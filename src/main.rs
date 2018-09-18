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
    let mut key;
    let mut keystring;
    let mut quitnow = false;

    initscr();

    printw("Starting command parsing:\n");

    while quitnow == false {
        key = getch() as u8 as char;
        keystring = key.clone().to_string();
        //        keystring.clear();
        //        keystring.push_str(":");

        let commands = IdentParser::parse(Rule::cmd_list, &keystring)
            .unwrap_or_else(|e| panic!("{}", e));

        for cmd in commands {
            let span = cmd.clone().into_span();
            printw(&format!(
                "{:?}: {{{},{}}} {}",
                cmd.as_rule(),
                span.start(),
                span.end(),
                span.as_str()
            ));

            for inner_cmd in cmd.into_inner() {
                let inner_span = inner_cmd.clone().into_span();
                match inner_cmd.as_rule() {
                    Rule::down => (),
                    Rule::up => (),
                    Rule::left => (),
                    Rule::right => (),
                    Rule::start => (),
                    Rule::end => (),
                    Rule::replace => (),
                    Rule::remove => (),
                    Rule::insert => (),
                    Rule::jumpascii => (),
                    Rule::helpfile => (),
                    Rule::search => (),
                    Rule::saveandexit => (),
                    Rule::exit => quitnow = true,
                    Rule::save => (),
                    Rule::gatherall => (),
                    _ => unreachable!(),
                };
                printw(&format!(" ({})", inner_span.as_str()));
            }
            printw("\n");
            refresh();

        }
    }
    endwin();
}
