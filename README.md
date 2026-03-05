# Rust IO Project
**Author** Jawntae St.Germaine

**Date** 3/4/2026

## Description
This program takes an input file filled with data about a person, sorts it, and outputs it to a specified file via the command line.

## Purpose
This project was originally a C-based project for WCU CS352, but was built in Rust to become more familiar with the language and file IO.

## fields of data required (no other fields may exist other than these)
each field must be separated by a comma: 
- first name
- last name
- street address
- city
- state
- zip
- phone

## Requirements
- cargo 1.92.0 or newer
- must be running on Linux or WSL
- The desired input file must exist within the project directory
- If you decide to use your own custom input file, it must follow the field requirements above

## To run the program on Linux/WSL
To download the program:
```
git clone https://github.com/Jawntae/io-rust-sort
```
To run via command line:
```
cargo run -- <input_file.txt> <output_file.txt>
```
