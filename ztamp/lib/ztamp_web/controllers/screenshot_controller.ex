defmodule ZtampWeb.ScreenshotController do
  @moduledoc """
  Serves screenshot images from the configured screenshot directory.

  Validates filenames to prevent path traversal. Only serves PNG files.
  """

  use ZtampWeb, :controller

  @doc "Serve a screenshot file by filename."
  def show(conn, %{"filename" => filename}) do
    screenshot_dir = Application.get_env(:wallaby, :screenshot_dir)

    with :ok <- validate_filename(filename),
         path = Path.join(screenshot_dir, filename),
         true <- File.exists?(path) do
      conn
      |> put_resp_content_type("image/png")
      |> send_file(200, path)
    else
      _ ->
        conn
        |> put_status(:not_found)
        |> text("Not found")
    end
  end

  defp validate_filename(filename) do
    cond do
      String.contains?(filename, "..") -> :error
      String.contains?(filename, "/") -> :error
      String.contains?(filename, "\\") -> :error
      not String.ends_with?(filename, ".png") -> :error
      true -> :ok
    end
  end
end
