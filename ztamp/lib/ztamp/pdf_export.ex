defmodule Ztamp.PdfExport do
  @moduledoc """
  Orchestrates PDF packet export for TANF job search entries.

  Generates a complete proof-of-job-search packet containing a cover page,
  filled TANF form pages (10 entries per page), and screenshot pages
  (one per entry). Calls the `tanf-export` Rust CLI tool to produce
  the final PDF.
  """

  alias Ztamp.JobSearch.Entry

  @project_root Path.expand("../../..", __DIR__)
  @tools_bin Path.join(@project_root, "tools/target/release/tanf-export")
  @offsets_path Path.join(@project_root, "assets/form/form_offsets.toml")
  @secrets_path Path.join(@project_root, "secret/secrets.toml")
  @template_path Path.join(@project_root, "assets/form/template.tiff")
  @output_dir Path.join(@project_root, "secret")

  @doc """
  Export a PDF packet for the given entries.

  Entries should be sorted from oldest to newest. Returns
  `{:ok, output_path}` on success or `{:error, reason}` on failure.
  """
  @spec export(list(Entry.t())) :: {:ok, String.t()} | {:error, String.t()}
  def export(entries) when is_list(entries) do
    with :ok <- validate_prerequisites(),
         {:ok, manifest_path} <- write_manifest(entries),
         output_path = generate_output_path(),
         {:ok, _} <- run_export(manifest_path, output_path) do
      File.rm(manifest_path)
      {:ok, output_path}
    end
  end

  defp validate_prerequisites do
    cond do
      not File.exists?(@tools_bin) ->
        {:error, "tanf-export binary not found at #{@tools_bin}. Run: cd tools && cargo build --release"}

      not File.exists?(@offsets_path) ->
        {:error, "Form offsets not found at #{@offsets_path}"}

      not File.exists?(@secrets_path) ->
        {:error, "Secrets file not found at #{@secrets_path}"}

      not File.exists?(@template_path) ->
        {:error, "Template image not found at #{@template_path}"}

      true ->
        :ok
    end
  end

  defp write_manifest(entries) do
    sorted_entries = Enum.sort_by(entries, & &1.date, Date)

    manifest = %{
      date_from: format_date_for_form(List.first(sorted_entries).date),
      date_to: format_date_for_form(List.last(sorted_entries).date),
      total_hours: compute_total_hours(sorted_entries),
      total_time_display: compute_cumulative_time(sorted_entries),
      total_entries: length(sorted_entries),
      recruiter_count: Enum.count(sorted_entries, & &1.applied_via_recruiter),
      entries: Enum.map(sorted_entries, &entry_to_manifest/1)
    }

    manifest_path = Path.join(System.tmp_dir!(), "tanf_export_#{System.unique_integer([:positive])}.json")

    case File.write(manifest_path, Jason.encode!(manifest)) do
      :ok -> {:ok, manifest_path}
      {:error, reason} -> {:error, "Failed to write manifest: #{inspect(reason)}"}
    end
  end

  defp entry_to_manifest(%Entry{} = entry) do
    %{
      date: format_date_for_form(entry.date),
      employer_name_address: format_employer(entry),
      how_contact_made: entry.how_contact_made || "",
      telephone_fax: entry.telephone_fax || "",
      telephone_number: entry.telephone_number || "",
      internet_confirmation: entry.internet_confirmation || "",
      time_in: entry.time_in || "",
      time_out: entry.time_out || "",
      screenshot_path: entry.screenshot_path || ""
    }
  end

  defp format_employer(entry) do
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
      nil -> name
      _ -> "#{name}, #{address}"
    end
  end

  defp format_date_for_form(%Date{} = date) do
    month = String.pad_leading(Integer.to_string(date.month), 2, "0")
    day = String.pad_leading(Integer.to_string(date.day), 2, "0")
    "#{month}/#{day}/#{date.year}"
  end

  defp compute_total_hours(entries) do
    total_minutes =
      entries
      |> Enum.map(fn entry -> duration_minutes(entry.time_in, entry.time_out) end)
      |> Enum.sum()

    div(total_minutes, 60)
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

  defp generate_output_path do
    timestamp =
      NaiveDateTime.local_now()
      |> NaiveDateTime.to_iso8601()
      |> String.replace(~r/[:\-]/, "")
      |> String.replace("T", "_")
      |> String.slice(0..14)

    Path.join(@output_dir, "tanf_export_#{timestamp}.pdf")
  end

  defp run_export(manifest_path, output_path) do
    args = [
      "--manifest", manifest_path,
      "--offsets", @offsets_path,
      "--secrets", @secrets_path,
      "--template", @template_path,
      "--output", output_path
    ]

    case System.cmd(@tools_bin, args, stderr_to_stdout: true) do
      {output, 0} ->
        {:ok, output}

      {output, exit_code} ->
        {:error, "tanf-export failed (exit #{exit_code}): #{output}"}
    end
  end
end
