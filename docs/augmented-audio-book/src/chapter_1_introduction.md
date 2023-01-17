# Chapter 1: Introduction

Welcome to the **Augmented Audio Book**!

This document should help you get started with writing audio software with the **Rust programming language** and
publishing your own `VST`/`AU`/`CLAP` plug-ins and apps.

## What is this book about?

In 2020, I started working on a project called **Augmented Audio**

My goal was to use the time I had during COVID lock-downs to learn as much as possible about digital audio. 
My goal was **not** to publish production quality software although some prototypes were published.

This document explains how to use the tooling I have been developing and it too is means for me to evolve as a
developer more than a comprehensive or correct guide. However, I do hope you will find it useful and that there's
something in here for everyone.

## Pre-requisites

In order to benefit from this book you should have a good understanding of the **Rust programming language**
and its tooling. You can refer to the official [Rust book](https://doc.rust-lang.org/book/) for an overview.

Additionally, you should be familiar with some digital audio concepts such as buffers, sample rates, and
general audio-effects terminology. I suggest going over the [Audio Effects: Theory, Implementation and Application](https://www.amazon.com/Audio-Effects-Theory-Implementation-Application/dp/1466560282) or
a similar book to get started.

### Tooling

I expect you have:

* A rust compiler
* Cargo
* A text editor