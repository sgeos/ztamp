# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions.

1. Is it possible to extract the image?
   Ideally, I want to straighten it, and clear out the filled in fields.
   Information in the fields can be moved to `secrets.toml`.
   Having said that, proactively defining fields in `toml` or some other format makes sense.
2. In theory, it might make sense to extract the image and use it to generate
   new PDFs with `printpdf`.
   This likely requires a different approach than the one I mentioned for the CLI tool.
   I think `lopdf` is suitable for one at a time edits via CLI.
   Thoughts on the best strategy?
3. We can start V0.1. See below.
4. Please commit `secret/secrets.toml.example`.

The project structure is basically going to be same as the crypto reference project.
Feel free to view it.
`/Users/bsechter/projects/crypto/cordial_cantina`

- Top level directory.
- Rust subproject `rztamp` for portable Rust code.
- Elixir subproject `ztamp`.
- NIF subproject in Elixir subproject that pulls in `rztamp` as a dependancy.
- It probably makes sense to have a `tools` subproject for CLI tools
  based on `rztamp` and utility shell scripts.

## Objectives

### Generate Project Structure

- `rztamp` Rust library.
- `ztamp` Phoenix project with Liveview and Postgresql.
- NIF subproject in Elixir subproject that pulls in `rztamp` as a dependancy.
- Rust project loads stub NIF.

### Commit secrets.toml.example

Please commit `secret/secrets.toml.example`.

## Context

To meet my TANF requirements,
I will need to look for a job for 30 hours per week.
The problem is that this needs to be documented on
a government form that was distributed to me as a PDF.
I also need to include a screenshot of the post-submission
receipt page after every submission.
I therefore need an automate solution for applying to jobs
and tracking application.

## Constraints

We absolutely need to work with the supplied government PDF.
This PDF should not be included in the git repo.

## Success Criteria

- Specified project structure generated.
- `secret/secrets.toml.example` committed.
- Response to questions and discussion in reverse prompt.

## Notes

I would like the tech stack to be Phoenix/Elixir with Liveview,
and a Rust NIF for number crunching and modify in place algorithms.
If custom command line tools are needed, they can be written in Rust.
