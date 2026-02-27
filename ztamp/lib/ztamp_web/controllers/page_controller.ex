defmodule ZtampWeb.PageController do
  use ZtampWeb, :controller

  @dev_routes Application.compile_env(:ztamp, :dev_routes, false)

  def home(conn, _params) do
    render(conn, :home, dev_routes: @dev_routes)
  end
end
