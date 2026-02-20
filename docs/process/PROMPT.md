# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions in the reverse prompt.

1. Manual verification complete.
2. V0.4 can be complete.

I used the system to apply for 23 jobs with screenshots taken.
I want a few changes.

After this prompt, the goal is to implement PDF export using
the application data I collected as a PoC for the process.

## Objectives

### Merge V0.4 into Master

Merge V0.4 into Master

### Update Job Search Dashboard

The following should be selected by default:

- How Contact Made: Online
- T/F/E/O: Online Application (modify to add this option)

Employer Name & Address should be split into:

- Employer Name
- Employer Address

Next to employer name, there should be a checkmark for:

- Applied via Recruiter

Next to addres, there should be two checkmarks for:

- United States (greys out address if selected and submits "United States" as the address)
- Remote

Database needs to be modified to accomodate these changes.

#### Update Captured Entries

Captured entries should have:

- From date
- To date
- Checkmark for "To Present"
- List the total number of applications

Only applications in the range should be listed and counted towards to total

#### Detailed Entry Display and Modification

There should be a link for each entries that brings up a dialog to:

- See complete details, and
- Modify details, and
- Optionally delete the entry

All database fields should be listed in this dialog,
and a small version of the captured screenshot should be displayed.
Clicking on the screenshot should open a full sized version in another browser tab.

### Document All Installation and Startup Instructions

Update the ztamp and rztamp `README.md` files to note all project
and subproject relevant setup and startup instructions.
Update the top level readme to point to the project specific readmes
with a short note about setup and startup instructions.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- V0.4 merged into Master.
- Update database schema as requested.
- Update job search dashboard as requested.
  - Job search fields.
  - Captured entries.
  - Detailed entry modification dialog.
- Document installation and startup instructions in relevant readmes.

## Notes

(none)
