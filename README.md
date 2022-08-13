# wrend

**W**ebGL2 **Rend**er Library

Note: **This library is currently experimental and not considered stable for public use**.

## About 

Wrend is a wrapper library around raw WebGL2, written in Rust, to make working with WebGL & Rust together in the browser more convenient.

## Why

This library exists because I found myself writing the same verbose, (often unsafe) WebGL code over and over again, constantly struggling to find the right level and type of abstraction. Wrend is designed to ease the pain of working with low-level WebGL programming easier in Rust. One particular convenience Wrend provides is an abstraction over requestAnimationFrame calls, making continuous animations as simple as calling start and holding the returned handle in memory. Stopping is also as easy as dropping the handle and/or calling stop.