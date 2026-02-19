defmodule ZtampWeb.PageController do
  use ZtampWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
