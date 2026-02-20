defmodule ZtampWeb.JobSearchLive do
  @moduledoc """
  LiveView for the job search screenshot capture workflow.

  Provides a form for entering job application details, browser control
  buttons, and a screenshot capture button that saves both the screenshot
  and the form data to the database.
  """

  use ZtampWeb, :live_view

  alias Ztamp.JobSearch
  alias Ztamp.JobSearch.Entry
  alias Ztamp.BrowserServer

  @impl true
  def mount(_params, _session, socket) do
    entries = JobSearch.list_entries()
    changeset = JobSearch.change_entry(%Entry{}, default_attrs())
    browser_status = BrowserServer.status()

    {:ok,
     socket
     |> assign(:entries, entries)
     |> assign(:browser_status, browser_status)
     |> assign(:use_submission_time, true)
     |> assign_form(changeset)}
  end

  @impl true
  def handle_event("validate", %{"entry" => entry_params}, socket) do
    changeset =
      %Entry{}
      |> JobSearch.change_entry(entry_params)
      |> Map.put(:action, :validate)

    {:noreply, assign_form(socket, changeset)}
  end

  def handle_event("start_browser", _params, socket) do
    case BrowserServer.start_browser() do
      {:ok, _} ->
        {:noreply,
         socket
         |> assign(:browser_status, BrowserServer.status())
         |> put_flash(:info, "Browser started.")}

      {:error, reason} ->
        {:noreply, put_flash(socket, :error, "Failed to start browser: #{inspect(reason)}")}
    end
  end

  def handle_event("stop_browser", _params, socket) do
    BrowserServer.stop_browser()

    {:noreply,
     socket
     |> assign(:browser_status, BrowserServer.status())
     |> put_flash(:info, "Browser stopped.")}
  end

  def handle_event("take_screenshot", %{"entry" => entry_params}, socket) do
    case BrowserServer.take_screenshot() do
      {:ok, screenshot_path} ->
        attrs =
          entry_params
          |> Map.put("screenshot_path", screenshot_path)
          |> maybe_set_submission_time(socket.assigns.use_submission_time)

        case JobSearch.create_entry(attrs) do
          {:ok, _entry} ->
            entries = JobSearch.list_entries()
            changeset = JobSearch.change_entry(%Entry{}, default_attrs())

            {:noreply,
             socket
             |> assign(:entries, entries)
             |> assign(:browser_status, BrowserServer.status())
             |> assign_form(changeset)
             |> put_flash(:info, "Screenshot captured and entry saved.")}

          {:error, %Ecto.Changeset{} = changeset} ->
            {:noreply,
             socket
             |> assign_form(changeset)
             |> put_flash(:error, "Entry validation failed. Screenshot was saved to disk.")}
        end

      {:error, reason} ->
        {:noreply, put_flash(socket, :error, "Screenshot failed: #{inspect(reason)}")}
    end
  end

  def handle_event("refresh_status", _params, socket) do
    {:noreply, assign(socket, :browser_status, BrowserServer.status())}
  end

  def handle_event("toggle_submission_time", _params, socket) do
    {:noreply, assign(socket, :use_submission_time, !socket.assigns.use_submission_time)}
  end

  defp assign_form(socket, %Ecto.Changeset{} = changeset) do
    assign(socket, :form, to_form(changeset, as: :entry))
  end

  defp default_attrs do
    {date, time_str} = local_date_and_time()

    %{
      "date" => date,
      "time_in" => time_str,
      "time_out" => time_str
    }
  end

  defp maybe_set_submission_time(attrs, true) do
    {_date, time_str} = local_date_and_time()
    Map.put(attrs, "time_out", time_str)
  end

  defp maybe_set_submission_time(attrs, false), do: attrs

  defp local_date_and_time do
    {{year, month, day}, {hour, minute, _second}} = :calendar.local_time()

    date = :io_lib.format("~4..0B-~2..0B-~2..0B", [year, month, day]) |> IO.iodata_to_binary()
    time = :io_lib.format("~B:~2..0B", [hour, minute]) |> IO.iodata_to_binary()

    {date, time}
  end
end
