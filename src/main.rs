extern crate mumble_link;
extern crate regex;

use mumble_link::*;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn get_data(captures: &regex::Captures) -> Result<([f32; 3], char, char), String> {
    // Function to convert float parsing errors to error strings so we can return that as a result.
    fn errstr(e: std::num::ParseFloatError) -> String {
        e.to_string()
    }

    // Get all the named captures.
    Ok(([captures["x"].parse().map_err(errstr)?, captures["y"].parse().map_err(errstr)?, captures["z"].parse().map_err(errstr)?,],
        captures["subject"].chars().next().ok_or("unable to get vec subject")?,
        captures["type"].chars().next().ok_or("unable to get vec type")?,
    ))
}

fn main() {
    // Hook into Mumble using the very handy crate somebody made.
    let mut link = MumbleLink::new("Minetest", "Minetest positional audio using a mod and wrapper.").expect("Unable to link to Mumble. Is it running?");

    // Default command to launch.
    let mut minetest_command = "/usr/bin/minetest".to_owned();

    // Look for an argument containing "minetest" to replace default command.
    for argument in std::env::args() {
        if argument.contains("minetest") {
            minetest_command = argument
        }
    }

    let mut child = Command::new(minetest_command)
        .stderr(Stdio::piped()) // We need the output to be piped. For some reason Minetest lua's print function goes to stderr...
        .spawn().unwrap(); // Spawn the process, panic if it fails.

    // This regex parses lines like "p l [1.0 1.0 1.0]".
    // the first letter (the subject) is either 'p' or 'c' denoting whether this is a player or camera vector.
    // the second letter (the type) is either 'p' or 'l' denoting whether this is a position or look vector.
    // then inside brakets are the x, y, and z components, respectively.
    let vec_regex_str = format!(r"(?P<subject>[cp]) (?P<type>[pl]) \[(?P<x>{f}) (?P<y>{f}) (?P<z>{f})\]", f=r"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?");
    let vec_regex = Regex::new(&vec_regex_str).unwrap();

    // This regex parses commands like "mumble id playername".
    let cmd_regex = Regex::new(r"mumble (?P<cmd>submit|id|context) *(?P<arg>[[:alpha:]]*)").unwrap();

    let mut player = Position::default();
    let mut camera = Position::default();

    // Run as long as the child proccess is running.
    while child.try_wait().unwrap().is_none() {
        // Get the output from the child process.
        if let Some(ref mut child_output) = child.stderr {
            // Using a BufReader allows us to go through all the lines in a loop.
            for line_result in BufReader::new(child_output).lines() {
                // Gotta make sure the line is valid...
                if let Ok(ref line) = line_result {
                    // Try getting the captures from the regex.
                    if let Some(captures) = vec_regex.captures(line) {
                        match get_data(&captures) {
                            Ok((vec, s, t)) => {
                                // Get the Position item that we need to set something on.
                                let mut target = match s {
                                    'p' => &mut player,
                                    'c' => &mut camera,
                                    _ => continue,
                                };
                                // Figure out which component to set.
                                match t {
                                    'p' => target.position = vec,
                                    'l' => target.front = vec,
                                    _ => continue,
                                }
                                println!("got {} {} {:?}", s, t, vec);
                            }
                            Err(err) => {
                                println!("error getting vec: {}", err);
                            }
                        }
                    } else if let Some(captures) = cmd_regex.captures(line) {
                        let arg = &captures["arg"];

                        match &captures["cmd"] {
                            "submit" => {
                                // Submit the gathered data to Mumble.
                                println!("Updating...");
                                link.update(player, camera)
                            },
                            "id" => {
                                println!("got identity: {}", arg);
                                link.set_identity(arg);
                            },
                            "context" => {
                                println!("got context: {}", arg);
                                link.set_context(arg.as_bytes());
                            },
                            cmd => {
                                println!("Invalid command {}!", cmd);
                            },
                        }
                    }
                }
            }
        }
    }
}
