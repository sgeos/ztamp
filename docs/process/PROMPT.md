# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions.

1. I split the last page from the government PDF and put it in `secret`.
2. I believe the intent is proof of submission.
   The bureaucrats want proof of submission, and a "take my word for it."
   I do not actually know what their criteria for proof is.
3. No need to retroactively do anything.
   Process establishment takes a few commits, and this is expected.
4. Additional phases should be mapped.
   We need the following:
   - Stub project structure
   - PDF write PoC
   - PDF concatenation PoC
   - Browser screenshot PoC
   - Per platform submission logic
   - Submission track (where, when, etc)
   - Per application resume and cover letter customization
     - Multiple subtasks
   The goal is a managed submission process with tracking for compliance.
   Another goal is high volume submissions.

Blocking decisions.

1. D1. Ideally, I want to stick to Rust tooling.
       I think we can make a command line utility that writes strings
       and places images PDF files.
       `utility --do-task param form.pdf`
       This utility can then be called multiple times to fill out the form.
       This strategy may require a file to track field positions and widths.
2. D2. It looks like `wallaby` makes sense for D3.
       In isolation, I would say `chromic_pdf`,
       standardizing on `wallaby` probably makes sense.
3. D3. `wallaby` appears to be the winner for human in the loop applications.

## Objectives

### Attempt to Determine PDF Type

I split the last page from the government PDF and put it in `secret`.
I am open to type conversion, if possible.

### Document Version Phases

Document version phases.
This is very loose feature list.
   - Stub project structure
   - PDF write PoC
   - PDF concatenation PoC
   - Browser screenshot PoC
   - Per platform submission logic
   - Submission track (where, when, etc)
   - Per application resume and cover letter customization
     - Multiple subtasks

### Document Blocker Decisions

Document blocker decisions.

### Globally Whitelist Example and .gitkeep Files

Globally whitelist `*.example` and `.gitkeep` files in `.gitignore`.
I am pretty sure this is possible.

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

- Attempt to determine PDF type. Report in reverse prompt.
- Document version phases.
- Document blocker decisions.
- Globally whitelist `*.example` and `.gitkeep` files in `.gitignore`.

## Notes

I would like the tech stack to be Phoenix/Elixir with Liveview,
and a Rust NIF for number crunching and modify in place algorithms.
If custom command line tools are needed, they can be written in Rust.
