# Ztamp

TANF Job Search Automation web application built with Phoenix and LiveView.

## Prerequisites

- **Elixir** and **Erlang** (see `.tool-versions` or `mix.exs` for version requirements)
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

## Learn more

* Official website: https://www.phoenixframework.org/
* Guides: https://hexdocs.pm/phoenix/overview.html
* Docs: https://hexdocs.pm/phoenix
* Forum: https://elixirforum.com/c/phoenix-forum
* Source: https://github.com/phoenixframework/phoenix
