defmodule Ztamp.JobSearch do
  @moduledoc """
  Context for managing job search entries.

  Provides CRUD operations for job search contact records, each of which
  includes form metadata and a reference to a captured screenshot.
  """

  import Ecto.Query, warn: false
  alias Ztamp.Repo
  alias Ztamp.JobSearch.Entry

  @doc "List all entries, most recent first."
  def list_entries do
    Entry
    |> order_by([e], desc: e.inserted_at)
    |> Repo.all()
  end

  @doc """
  List entries within a date range, most recent first.

  Either boundary may be nil to leave that side unbounded.
  """
  def list_entries_in_range(from_date, to_date) do
    Entry
    |> order_by([e], desc: e.inserted_at)
    |> maybe_filter_from(from_date)
    |> maybe_filter_to(to_date)
    |> Repo.all()
  end

  defp maybe_filter_from(query, nil), do: query
  defp maybe_filter_from(query, date), do: where(query, [e], e.date >= ^date)

  defp maybe_filter_to(query, nil), do: query
  defp maybe_filter_to(query, date), do: where(query, [e], e.date <= ^date)

  @doc "Get a single entry by ID. Raises if not found."
  def get_entry!(id), do: Repo.get!(Entry, id)

  @doc "Create a new entry from the given attributes."
  def create_entry(attrs \\ %{}) do
    %Entry{}
    |> Entry.changeset(attrs)
    |> Repo.insert()
  end

  @doc "Update an existing entry with the given attributes."
  def update_entry(%Entry{} = entry, attrs) do
    entry
    |> Entry.changeset(attrs)
    |> Repo.update()
  end

  @doc "Delete an entry."
  def delete_entry(%Entry{} = entry) do
    Repo.delete(entry)
  end

  @doc "Return a changeset for tracking entry changes."
  def change_entry(%Entry{} = entry, attrs \\ %{}) do
    Entry.changeset(entry, attrs)
  end
end
