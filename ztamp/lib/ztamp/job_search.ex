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

  @doc "Return a changeset for tracking entry changes."
  def change_entry(%Entry{} = entry, attrs \\ %{}) do
    Entry.changeset(entry, attrs)
  end
end
