# Audio-Wave-Generator

# Overview

This program generates audio waveforms and saves them as a WAV file. It allows the user to specify the duration, frequency, and type of waveform to generate through command-line arguments. The supported waveform types are Sine, Square, Sawtooth, and Triangle. If no arguments are provided, the program uses default values for duration (2 seconds), frequency (440 Hz), and waveform type (Sine). The generated audio file is saved as "output.wav" in the current directory, but the filename can be specified as an optional fourth command-line argument. 

The program uses the byteorder and rand crates for generating the audio data and random frequency (if "random" is specified as the frequency argument), respectively.

[Software Demo Video](https://youtu.be/s65HqQkZ1sU)

# Development Environment

The following are the tools, libraries, and programming constructs were used in the development of the software:

- ByteOrder: A Rust crate that converts byte orders of primitive types between different endianness.
- LittleEndian variant of the byteorder crate is used to write audio samples in little-endian format.
- Rand: A Rust crate for generating random numbers, used to generate random frequencies in the function random_frequency.
- std::env: A Rust standard library module for accessing command-line arguments passed to the program.
- std::fs::File: A Rust standard library module for working with files, allowing opening, reading, writing, and closing of files.
- std::io::{BufWriter, Write}: Rust standard library modules for buffered I/O operations, where BufWriter is used to write audio data to a file efficiently, and Write is used to write the bytes of the WAV header.
- std::i16::MAX: A constant in Rust's standard library representing the maximum value of a signed 16-bit integer.


# Useful Websites

{Make a list of websites that you found helpful in this project}

- [TutorialsPoint](https://www.tutorialspoint.com/rust/index.html)
- [DOCS.rs](https://docs.rs/byteorder/latest/byteorder/index.html)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Code documentation 
- Refactoring
- Error handdling
