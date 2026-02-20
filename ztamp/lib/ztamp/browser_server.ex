defmodule Ztamp.BrowserServer do
  @moduledoc """
  GenServer managing a single Wallaby Chrome browser session.

  The browser session is started on demand (not at boot) and provides
  screenshot capture for the job search workflow. Chrome runs in
  non-headless mode so the human pilot can browse job sites visually.
  """

  use GenServer
  require Logger

  # -- Client API --

  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @doc "Start a Chrome browser session. Returns {:ok, :started} or {:error, reason}."
  def start_browser do
    GenServer.call(__MODULE__, :start_browser, 30_000)
  end

  @doc "Stop the current browser session."
  def stop_browser do
    GenServer.call(__MODULE__, :stop_browser)
  end

  @doc """
  Take a screenshot of the current browser page.

  Options:
    - `:name` - custom filename without extension (default: timestamp-based)

  Returns `{:ok, screenshot_path}` or `{:error, reason}`.
  """
  def take_screenshot(opts \\ []) do
    GenServer.call(__MODULE__, {:take_screenshot, opts}, 15_000)
  end

  @doc "Returns the current browser status as a map with :active and :current_url keys."
  def status do
    GenServer.call(__MODULE__, :status)
  end

  # -- Server Callbacks --

  @impl true
  def init(_opts) do
    {:ok, %{session: nil}}
  end

  @impl true
  def handle_call(:start_browser, _from, %{session: nil} = state) do
    case ensure_wallaby_started() do
      :ok ->
        case Wallaby.start_session() do
          {:ok, session} ->
            Logger.info("BrowserServer: Chrome session started")
            session = navigate_to_landing(session)
            {:reply, {:ok, :started}, %{state | session: session}}

          {:error, reason} ->
            Logger.error("BrowserServer: Failed to start session: #{inspect(reason)}")
            {:reply, {:error, reason}, state}
        end

      {:error, reason} ->
        {:reply, {:error, reason}, state}
    end
  end

  def handle_call(:start_browser, _from, state) do
    {:reply, {:ok, :already_started}, state}
  end

  def handle_call(:stop_browser, _from, %{session: nil} = state) do
    {:reply, {:ok, :not_running}, state}
  end

  def handle_call(:stop_browser, _from, %{session: session} = state) do
    Wallaby.end_session(session)
    Logger.info("BrowserServer: Chrome session ended")
    {:reply, {:ok, :stopped}, %{state | session: nil}}
  end

  def handle_call({:take_screenshot, _opts}, _from, %{session: nil} = state) do
    {:reply, {:error, :no_session}, state}
  end

  def handle_call({:take_screenshot, opts}, _from, %{session: session} = state) do
    try do
      name = Keyword.get(opts, :name, timestamp_name())
      session = Wallaby.Browser.take_screenshot(session, name: name)
      path = List.last(session.screenshots)
      {:reply, {:ok, path}, %{state | session: session}}
    rescue
      e ->
        Logger.error("BrowserServer: Screenshot failed: #{Exception.message(e)}")
        {:reply, {:error, Exception.message(e)}, state}
    end
  end

  def handle_call(:status, _from, %{session: nil} = state) do
    {:reply, %{active: false, current_url: nil}, state}
  end

  def handle_call(:status, _from, %{session: session} = state) do
    url =
      try do
        Wallaby.Browser.current_url(session)
      rescue
        _ -> "unknown"
      end

    {:reply, %{active: true, current_url: url}, state}
  end

  @impl true
  def terminate(_reason, %{session: nil}), do: :ok

  def terminate(_reason, %{session: session}) do
    Wallaby.end_session(session)
    :ok
  end

  # -- Private --

  defp ensure_wallaby_started do
    case Application.ensure_all_started(:wallaby) do
      {:ok, _} -> :ok
      {:error, {app, reason}} -> {:error, "Failed to start #{app}: #{inspect(reason)}"}
    end
  end

  @landing_url "http://localhost:4000/browser-landing"

  defp navigate_to_landing(session) do
    try do
      Wallaby.Browser.visit(session, @landing_url)
    rescue
      _ ->
        Logger.warning("BrowserServer: Could not navigate to landing page")
        session
    end
  end

  defp timestamp_name do
    DateTime.utc_now()
    |> DateTime.to_iso8601(:basic)
    |> String.replace(~r/[^0-9T]/, "")
  end
end
