# Product Identity

> **Navigation**: [Overview](./README.md) | [Documentation Root](../README.md)

## Purpose

Automate job search documentation for Temporary Assistance for Needy Families (TANF) compliance. The system must track 30 hours per week of job search activity, populate a government-issued PDF form, and capture post-submission receipt screenshots.

## Problem Statement

TANF recipients must document job search activities on a government form distributed as a PDF. Each job application requires a screenshot of the post-submission receipt page. Manual tracking and form completion is time-consuming and error-prone. An automated solution reduces the documentation burden and ensures compliance.

## Core Requirements

1. **PDF Form Population.** Read and write to the supplied government PDF form. The form itself is sensitive and must not be committed to version control.
2. **Job Application Tracking.** Record job applications with metadata including employer, position, date, and submission status.
3. **Receipt Screenshot Capture.** Capture and store screenshots of post-submission receipt pages as evidence of application completion.
4. **Weekly Hour Tracking.** Track and report hours spent on job search activities to verify the 30-hour weekly requirement.

## Constraints

- The government PDF is stored in the `secret/` directory and excluded from version control.
- The system must work with the PDF as-supplied. Custom forms are not acceptable substitutes.

## Related Sections

- [Technology Stack](../architecture/TECHNOLOGY_STACK.md) for implementation details
