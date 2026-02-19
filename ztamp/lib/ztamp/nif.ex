defmodule Ztamp.Nif do
  @moduledoc """
  NIF wrapper for the `rztamp` Rust library.

  Functions in this module delegate to Rust code compiled via Rustler.
  Each function has a fallback that raises `:nif_not_loaded` if the
  native library failed to load.
  """

  use Rustler,
    otp_app: :ztamp,
    crate: "nif"

  @doc "No-operation stub. Returns `:ok` when the NIF is loaded."
  def nop, do: :erlang.nif_error(:nif_not_loaded)

  @doc "Returns `true` when the `rztamp` library is linked and accessible."
  def alive, do: :erlang.nif_error(:nif_not_loaded)
end
