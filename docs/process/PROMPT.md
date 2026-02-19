# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Image extracted and fields cleared.
Attemping to straignten decreased the image quality.

- `secret/form.pdf` is the original PDF.
  This has confidential information.
- `secret/form.tiff` is the cleared form.
  I do not think there is any confidential information on it.
- `secret/form.png` was exported if you need it.
  You seem to be erroring out with:
  ```
  API Error: 400 {"type":"error","error":{"type":"invalid_request_error","message":"messages.5.content.12.image.source.base64.data: Image does not match the provided media type image/png"},"request_id":"req_011CYJEqfFdJ7eUzfobrMXDz"}
  ```

We need a directory for non-confidential information,
like the form and its offsets.
Please suggest.

Answers to questions.

1. Manual cleanup done.
2. We can add the `tools` directory when we need it.
3. I ran `mix ecto.create`.

Proposed field definition format is acceptable.

I think it makes sense to define PDF manipulation/generation routines in the `rztamp` library.
This way, they could be used in a CLI tool, or directly by the elixir app.

## Objectives

### Extract Fields

If possible, extract information from the fields in the PDF and write it to `secret/secrets.toml`.
Here are the filled out fields I can identify:
- Case Name
- UPI No
- Job Search from
- Job Search to
- Job Search hours
- Submission deadline time
- Submission deadline date
- Submission location

### Generate Form Offsets

If possible, generate form field offsets and store in `secrets/form_offsets.toml`.
Field offsets include all of the above secret fields, and the following.

Above Job Application Table

- Participant's Signature (Above Job Search Table)
- Participant's Signature Date (Above Job Search Table)

For each job application row.

- Date
- Employer's Name, Address
- How Contact was Made
- Telephone/Fax (T=Telephone F=Fax)
- Telephone # of Person Seen - or # Resume Faxed to
- Internet Confirmation Attached
- Time In
- Time Out
- (Office Use Onlye)

"Should you become employed" fields.

- Name, Address / Telephone No. of Employer (3 lines)
- Start Date
- No. of hours per week
- Hourly rate of pay
- How often paid (circle one)
  - Weekly
  - Bi-weekly
  - Semi-monthly
  - Monthly
- Date of first check
  - Month
  - Day
  - Year
- Tips
- Job Title
- Insurance
  - None
  - Employer Paid
  - Employee Paid
  - Both Paid

Bottom of Form

- Participant's Signature (Bottom of Form)
- Participant's Signature Date (Bottom of Form)

### Update Documentation

Update documentation where relevant.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Secrets extracted to `secret/secrets.toml`, if possible.
- Form field offsets extracted to `secrets/form_offsets.toml`, if possible.
- Relevant documentation updated.

## Notes

(none)
