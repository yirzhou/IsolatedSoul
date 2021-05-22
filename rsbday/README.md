# rsbday

Get the people having their birthdays yesterday, today, and tomorrow.

## Motivation

The only functionality of Facebook that I use is to check if I need to celebrate any birthday of my friends or acquaintances. What annoys me (and it's my own fault) is that I would involuntarily start browsing other stuff which would take a huge chunk of my time, compromising my productivity. Moreover, I just don't need the other features: I can use Eventbrite if I'm interested in events, Kijiji if I want to buy or sell stuff, and Messenger/WeChat/iMessage if I need to message someone.

## Architecture

This is a CLI app written in Rust. My vision for it is that it will become a more automated app that is scheduled to run periodically.

### Database

I cannot disclose any of my friends' birthdays so there's no DB file in this repo. It's just a portable SQLite file that is read by the program to tell me if someone is having his/her birthday. I chose SQLite because it's a universally deployed database engine, meaning that I can port the logic of this program to mobile if I need to.

### Application

The application itself is a CLI app that doesn't take any argument for now. ~~I just run the binary and it prints out the people having their birthdays yesterday, today, and tomorrow.~~ See the help page of the CLI:

```bash
rsbday 0.1.0
yiren.chow@gmail.com <yiren.chow@gmail.com>
Check out the people you love who are having birthdays recently!

USAGE:
    rsbday [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --reset      Reset your sqlite database.
    -V, --version    Prints version information

OPTIONS:
    -d, --day-range <DAY RANGE>    Sets a custom range of days
```

There are certain TODOs for the future:

- [x] More granular control on date selection
- [ ] Ability to add new birthdays/entries
