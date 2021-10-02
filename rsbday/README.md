# BDay.rs

Get the people having their birthdays yesterday, today, and tomorrow.

On my flight from Canada back to China, I read [Digital Minimalism](https://www.goodreads.com/book/show/40672036-digital-minimalism?ac=1&from_search=true&qid=ppUnvMNA6U&rank=1) by Cal Newport. In the word of the author, minimalism is _the art of knowing how much is just enough_. It helps answering the question of why a lot of people (myself included) are so addicted to digital products. It also lays out a series of steps for how to becoming more focused in such a noisy world.

One of the steps is to curate a set of apps by asking yourself for the stuff you truely value. Only those apps should be kept.

This got me into thinking (while I'm forced to be in quarantine in a hotel): how about de-clutter myself from those apps?


## Motivation

The only functionality of Facebook that I use is to check if I need to celebrate any birthday of my friends or acquaintances. What annoys me (and it's my own fault) is that I would involuntarily start browsing other stuff which would take a huge chunk of my time, compromising my productivity. Moreover, I just don't need the other features: I can use Eventbrite if I'm interested in events, Kijiji if I want to buy or sell stuff, and Messenger/WeChat/iMessage if I need to message someone.

## Architecture

This is a CLI app written in Rust. My vision for it is that it will become a more automated app that is scheduled to run periodically.

### Database

I cannot disclose any of my friends' birthdays so there's no DB file in this repo. It's just a portable SQLite file that is read by the program to tell me if someone is having his/her birthday. I chose SQLite because it's a universally deployed database engine, meaning that I can port the logic of this program to mobile if I need to.

### Application

The application itself is a CLI app that doesn't take any argument for now. ~~I just run the binary and it prints out the people having their birthdays yesterday, today, and tomorrow.~~ See the help page of the CLI:

```plaintext
rsbday 1.0.0
yiren.chow@gmail.com <yiren.chow@gmail.com>
Check out the people you love who are having birthdays recently!

USAGE:
    rsbday [OPTIONS] --db-path <DB> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --day-range <DAY RANGE>    Sets a custom range of days
        --db-path <DB>             The path to your database file

SUBCOMMANDS:
    add      Add a new person's birthday
    help     Prints this message or the help of the given subcommand(s)
    reset    Reset your sqlite database
```

For example, the following command returns people celebrating their birthdays within today ± 10 days:

```bash
rsbday --db-path ./data/bdays.db -r 10
```

There are certain TODOs for the future:

- [x] More granular control on date selection
- [x] Ability to add new birthdays/entries
