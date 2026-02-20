defmodule Ztamp.JobSearch.Entry do
  @moduledoc """
  Schema for a single job search contact entry.

  Each entry records one job application or contact made during a job
  search period. The screenshot_path references the captured confirmation
  page screenshot stored in the secret/screenshots/ directory.
  """

  use Ecto.Schema
  import Ecto.Changeset

  schema "job_search_entries" do
    field :date, :date
    field :employer_name_address, :string
    field :how_contact_made, :string
    field :telephone_fax, :string
    field :telephone_number, :string
    field :internet_confirmation, :string
    field :time_in, :string
    field :time_out, :string
    field :screenshot_path, :string

    timestamps(type: :utc_datetime)
  end

  @required_fields ~w(date employer_name_address how_contact_made
                      time_in time_out screenshot_path)a
  @optional_fields ~w(telephone_fax telephone_number internet_confirmation)a

  @doc "Build a changeset for an entry, validating required fields and constraints."
  def changeset(entry, attrs) do
    entry
    |> cast(attrs, @required_fields ++ @optional_fields)
    |> validate_required(@required_fields)
    |> validate_inclusion(:telephone_fax, ~w(T F E),
      message: "must be T (Telephone), F (Fax), or E (Email)"
    )
  end
end
