defmodule ZtampWeb.JobSearchLive do
  @moduledoc """
  LiveView for the job search screenshot capture workflow.

  Provides a form for entering job application details, browser control
  buttons, a screenshot capture button, date-filtered entry listing with
  cumulative time tracking, and an entry detail/edit/delete modal.
  """

  use ZtampWeb, :live_view

  alias Ztamp.JobSearch
  alias Ztamp.JobSearch.Entry
  alias Ztamp.BrowserServer

  @impl true
  def mount(_params, _session, socket) do
    entries = JobSearch.list_entries_in_range(nil, Date.utc_today())
    changeset = JobSearch.change_entry(%Entry{}, default_attrs())
    browser_status = BrowserServer.status()

    {:ok,
     socket
     |> assign(:entries, entries)
     |> assign(:cumulative_time, compute_cumulative_time(entries))
     |> assign(:browser_status, browser_status)
     |> assign(:use_submission_time, true)
     |> assign(:united_states, false)
     |> assign(:remote, false)
     |> assign(:filter_from, nil)
     |> assign(:filter_to, nil)
     |> assign(:to_present, true)
     |> assign(:show_entry_modal, false)
     |> assign(:selected_entry, nil)
     |> assign(:edit_form, nil)
     |> assign_form(changeset)}
  end

  # -- Form Events --

  @impl true
  def handle_event("validate", %{"entry" => entry_params}, socket) do
    changeset =
      %Entry{}
      |> JobSearch.change_entry(entry_params)
      |> Map.put(:action, :validate)

    {:noreply, assign_form(socket, changeset)}
  end

  def handle_event("take_screenshot", %{"entry" => entry_params}, socket) do
    case BrowserServer.take_screenshot() do
      {:ok, screenshot_path} ->
        attrs =
          entry_params
          |> Map.put("screenshot_path", screenshot_path)
          |> maybe_set_submission_time(socket.assigns.use_submission_time)
          |> maybe_set_united_states(socket.assigns.united_states)
          |> maybe_set_remote(socket.assigns.remote)

        case JobSearch.create_entry(attrs) do
          {:ok, _entry} ->
            entries = reload_entries(socket)
            changeset = JobSearch.change_entry(%Entry{}, default_attrs())

            {:noreply,
             socket
             |> assign(:entries, entries)
             |> assign(:cumulative_time, compute_cumulative_time(entries))
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

  # -- Browser Events --

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

  def handle_event("refresh_status", _params, socket) do
    {:noreply, assign(socket, :browser_status, BrowserServer.status())}
  end

  # -- Toggle Events --

  def handle_event("toggle_submission_time", _params, socket) do
    {:noreply, assign(socket, :use_submission_time, !socket.assigns.use_submission_time)}
  end

  def handle_event("toggle_united_states", _params, socket) do
    {:noreply, assign(socket, :united_states, !socket.assigns.united_states)}
  end

  def handle_event("toggle_remote", _params, socket) do
    {:noreply, assign(socket, :remote, !socket.assigns.remote)}
  end

  # -- Filter Events --

  def handle_event("filter_entries", params, socket) do
    from_str = Map.get(params, "from", "")
    to_str = Map.get(params, "to", "")
    from_date = parse_date(from_str)

    to_date =
      if socket.assigns.to_present do
        Date.utc_today()
      else
        parse_date(to_str)
      end

    entries = JobSearch.list_entries_in_range(from_date, to_date)

    {:noreply,
     socket
     |> assign(:filter_from, from_str)
     |> assign(:filter_to, to_str)
     |> assign(:entries, entries)
     |> assign(:cumulative_time, compute_cumulative_time(entries))}
  end

  def handle_event("toggle_to_present", _params, socket) do
    new_val = !socket.assigns.to_present

    to_date =
      if new_val do
        Date.utc_today()
      else
        parse_date(socket.assigns.filter_to)
      end

    from_date = parse_date(socket.assigns.filter_from)
    entries = JobSearch.list_entries_in_range(from_date, to_date)

    {:noreply,
     socket
     |> assign(:to_present, new_val)
     |> assign(:entries, entries)
     |> assign(:cumulative_time, compute_cumulative_time(entries))}
  end

  # -- Modal Events --

  def handle_event("show_entry", %{"id" => id}, socket) do
    entry = JobSearch.get_entry!(id)
    changeset = JobSearch.change_entry(entry)

    {:noreply,
     socket
     |> assign(:show_entry_modal, true)
     |> assign(:selected_entry, entry)
     |> assign(:edit_form, to_form(changeset, as: :edit_entry))}
  end

  def handle_event("close_entry_modal", _params, socket) do
    {:noreply,
     socket
     |> assign(:show_entry_modal, false)
     |> assign(:selected_entry, nil)
     |> assign(:edit_form, nil)}
  end

  def handle_event("validate_edit", %{"edit_entry" => params}, socket) do
    changeset =
      socket.assigns.selected_entry
      |> JobSearch.change_entry(params)
      |> Map.put(:action, :validate)

    {:noreply, assign(socket, :edit_form, to_form(changeset, as: :edit_entry))}
  end

  def handle_event("save_edit", %{"edit_entry" => params}, socket) do
    case JobSearch.update_entry(socket.assigns.selected_entry, params) do
      {:ok, _entry} ->
        entries = reload_entries(socket)

        {:noreply,
         socket
         |> assign(:entries, entries)
         |> assign(:cumulative_time, compute_cumulative_time(entries))
         |> assign(:show_entry_modal, false)
         |> assign(:selected_entry, nil)
         |> assign(:edit_form, nil)
         |> put_flash(:info, "Entry updated.")}

      {:error, changeset} ->
        {:noreply, assign(socket, :edit_form, to_form(changeset, as: :edit_entry))}
    end
  end

  def handle_event("delete_entry", %{"id" => id}, socket) do
    entry = JobSearch.get_entry!(id)
    {:ok, _} = JobSearch.delete_entry(entry)
    entries = reload_entries(socket)

    {:noreply,
     socket
     |> assign(:entries, entries)
     |> assign(:cumulative_time, compute_cumulative_time(entries))
     |> assign(:show_entry_modal, false)
     |> assign(:selected_entry, nil)
     |> assign(:edit_form, nil)
     |> put_flash(:info, "Entry deleted.")}
  end

  # -- Export Events --

  def handle_event("export_pdf", _params, socket) do
    entries = socket.assigns.entries

    if entries == [] do
      {:noreply, put_flash(socket, :error, "No entries to export.")}
    else
      case Ztamp.PdfExport.export(entries) do
        {:ok, output_path} ->
          filename = Path.basename(output_path)

          {:noreply,
           put_flash(socket, :info, "PDF exported to secret/#{filename}")}

        {:error, reason} ->
          {:noreply, put_flash(socket, :error, "PDF export failed: #{reason}")}
      end
    end
  end

  # -- Private Helpers --

  defp assign_form(socket, %Ecto.Changeset{} = changeset) do
    assign(socket, :form, to_form(changeset, as: :entry))
  end

  defp default_attrs do
    {date, time_str} = local_date_and_time()

    %{
      "date" => date,
      "time_in" => time_str,
      "time_out" => time_str,
      "how_contact_made" => "Internet",
      "telephone_fax" => "O"
    }
  end

  defp reload_entries(socket) do
    from_date = parse_date(socket.assigns.filter_from)

    to_date =
      if socket.assigns.to_present do
        Date.utc_today()
      else
        parse_date(socket.assigns.filter_to)
      end

    JobSearch.list_entries_in_range(from_date, to_date)
  end

  defp maybe_set_submission_time(attrs, true) do
    {_date, time_str} = local_date_and_time()
    Map.put(attrs, "time_out", time_str)
  end

  defp maybe_set_submission_time(attrs, false), do: attrs

  defp maybe_set_united_states(attrs, true), do: Map.put(attrs, "employer_address", "United States")
  defp maybe_set_united_states(attrs, false), do: attrs

  defp maybe_set_remote(attrs, true), do: Map.put(attrs, "remote", "true")
  defp maybe_set_remote(attrs, false), do: attrs

  defp local_date_and_time do
    {{year, month, day}, {hour, minute, _second}} = :calendar.local_time()

    date = :io_lib.format("~4..0B-~2..0B-~2..0B", [year, month, day]) |> IO.iodata_to_binary()
    time = :io_lib.format("~B:~2..0B", [hour, minute]) |> IO.iodata_to_binary()

    {date, time}
  end

  defp parse_date(""), do: nil
  defp parse_date(nil), do: nil

  defp parse_date(str) do
    case Date.from_iso8601(str) do
      {:ok, date} -> date
      _ -> nil
    end
  end

  defp compute_cumulative_time(entries) do
    total_minutes =
      entries
      |> Enum.map(fn entry -> duration_minutes(entry.time_in, entry.time_out) end)
      |> Enum.sum()

    hours = div(total_minutes, 60)
    minutes = rem(total_minutes, 60)

    cond do
      hours > 0 and minutes > 0 -> "#{hours}h #{minutes}m"
      hours > 0 -> "#{hours}h"
      true -> "#{minutes}m"
    end
  end

  defp duration_minutes(time_in, time_out) do
    with {:ok, start_mins} <- parse_time_to_minutes(time_in),
         {:ok, end_mins} <- parse_time_to_minutes(time_out),
         diff when diff >= 0 <- end_mins - start_mins do
      diff
    else
      _ -> 0
    end
  end

  defp parse_time_to_minutes(time_str) when is_binary(time_str) do
    case String.split(time_str, ":") do
      [h, m] ->
        with {hours, ""} <- Integer.parse(h),
             {minutes, ""} <- Integer.parse(m) do
          {:ok, hours * 60 + minutes}
        else
          _ -> :error
        end

      _ ->
        :error
    end
  end

  defp parse_time_to_minutes(_), do: :error

  @doc false
  def format_employer(entry) do
    name =
      if entry.applied_via_recruiter do
        "Unknown (applied via #{entry.employer_name})"
      else
        entry.employer_name
      end

    address =
      if entry.remote do
        "#{entry.employer_address} (remote)"
      else
        entry.employer_address
      end

    case address do
      "" -> name
      _ -> "#{name}, #{address}"
    end
  end
end
