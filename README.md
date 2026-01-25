# DS 210 A1 - Code

This repo contains the code used during [DS 210 A1](https://rust4ds.github.io/ds210-sp26-a1/) lectures and discussion sections.

## Organization

The code is organized by lecture or discussion sections. It may include code that the instructors presented or
live-coded in class, or solutions to in-class activities or exercises.

The `/lecture_<n>_<title>/` and `/discussion_<n>_title/` directories contain code from the nth lecture or discussion section, respectively.
Lectures and discussion sections that do not introduce new code examples or contain coding activities will not have a corresponding
directory in this repo, e.g., lecture 1 which covered the course's syllabus and policies.

Each lecture or discussion section directory contains one or more sub directory of the form `<n>_<title>`.
This sub directory is a standalone Rust project that contains a logical unit with one or more code examples.
If we saw multiple, unrelated code examples in class, they will each have their own subdirectory in the order they appeared in class.

Finally, if a sub directory starts with `<n>_q_`, it represents code given to students as part of an exercise, with its answer later published in
a corresponding sub directory starting with `<n>_a_`.

## Running and tinkering with the code

Each code example or logical unit is structured as a standalone Rust project.
You can open that project in VSCode to view or run its individual parts.

Further instructions may be provided in README files attached to the code example, as appropriate.

For an example, check out [lecture_2_why_rust/1_rust_vs_python.md](lecture_2_why_rust/1_rust_vs_python.md).
