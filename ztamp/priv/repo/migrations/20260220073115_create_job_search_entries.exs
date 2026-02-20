defmodule Ztamp.Repo.Migrations.CreateJobSearchEntries do
  use Ecto.Migration

  def change do
    create table(:job_search_entries) do
      add :date, :date, null: false
      add :employer_name_address, :string, null: false
      add :how_contact_made, :string, null: false
      add :telephone_fax, :string
      add :telephone_number, :string
      add :internet_confirmation, :string
      add :time_in, :string, null: false
      add :time_out, :string, null: false
      add :screenshot_path, :string, null: false

      timestamps(type: :utc_datetime)
    end
  end
end
