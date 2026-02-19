defmodule Ztamp.Repo do
  use Ecto.Repo,
    otp_app: :ztamp,
    adapter: Ecto.Adapters.Postgres
end
