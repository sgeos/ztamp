defmodule Ztamp.Repo.Migrations.SplitEmployerFields do
  use Ecto.Migration

  def up do
    alter table(:job_search_entries) do
      add :employer_name, :string
      add :employer_address, :string
      add :applied_via_recruiter, :boolean, default: false, null: false
      add :remote, :boolean, default: false, null: false
    end

    execute """
    UPDATE job_search_entries
    SET employer_name = employer_name_address,
        employer_address = ''
    """

    alter table(:job_search_entries) do
      modify :employer_name, :string, null: false
      modify :employer_address, :string, null: false
    end

    alter table(:job_search_entries) do
      remove :employer_name_address
    end
  end

  def down do
    alter table(:job_search_entries) do
      add :employer_name_address, :string
    end

    execute """
    UPDATE job_search_entries
    SET employer_name_address =
      CASE
        WHEN applied_via_recruiter AND remote THEN
          'Unknown (applied via ' || employer_name || '), ' || employer_address || ' (remote)'
        WHEN applied_via_recruiter THEN
          'Unknown (applied via ' || employer_name || '), ' || employer_address
        WHEN remote THEN
          employer_name || ', ' || employer_address || ' (remote)'
        ELSE
          employer_name || ', ' || employer_address
      END
    """

    alter table(:job_search_entries) do
      modify :employer_name_address, :string, null: false
    end

    alter table(:job_search_entries) do
      remove :employer_name
      remove :employer_address
      remove :applied_via_recruiter
      remove :remote
    end
  end
end
