defmodule ZtampWeb.LandingController do
  @moduledoc """
  Landing page for the Wallaby-controlled browser session.

  Provides links to common job search sites as a starting point
  for the screenshot capture workflow.
  """

  use ZtampWeb, :controller

  def index(conn, _params) do
    render(conn, :index)
  end
end
