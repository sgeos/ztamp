# Ztamp

TANF Job Search Automation web application built with Phoenix and LiveView.

## Prerequisites

- **Elixir** and **Erlang** (see `.tool-versions` or `mix.exs` for version requirements)
- **Rust** toolchain (required for NIF compilation via Rustler)
- **PostgreSQL** running locally
- **ChromeDriver** matching the installed Chrome major version (required for the screenshot capture workflow)

ChromeDriver can be downloaded from [Chrome for Testing](https://googlechromelabs.github.io/chrome-for-testing/). The ChromeDriver major version must match the installed Google Chrome major version.

## Setup

```sh
mix setup
mix ecto.create
mix ecto.migrate
```

## Running

Start the Phoenix server:

```sh
mix phx.server
```

Or start inside IEx:

```sh
iex -S mix phx.server
```

Then visit [`localhost:4000`](http://localhost:4000) from your browser.

## Job Search Workflow

The job search dashboard is available at [`localhost:4000/job-search`](http://localhost:4000/job-search). It provides browser-automated screenshot capture for job application confirmations.

1. Click "Start Browser" to launch a Wallaby-controlled Chrome window.
2. The browser opens to a landing page with links to job search sites.
3. Navigate to a job application and reach the confirmation page.
4. Fill in the form fields on the dashboard and click "Take Screenshot & Save Entry."
5. The screenshot is saved to `secret/screenshots/` and the entry is persisted to the database.

Screenshots are served at `/screenshots/:filename` for viewing within the application.

## Learn more

* Official website: https://www.phoenixframework.org/
* Guides: https://hexdocs.pm/phoenix/overview.html
* Docs: https://hexdocs.pm/phoenix
* Forum: https://elixirforum.com/c/phoenix-forum
* Source: https://github.com/phoenixframework/phoenix
