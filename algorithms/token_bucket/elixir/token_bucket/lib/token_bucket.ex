defmodule TokenBucket do
  use GenServer

  @default_state %{tokens: 10, size: 10, refill_rate_in_ms: 1_000}

  defguard is_full(state) when state.tokens == state.size
  defguard is_empty(state) when state.tokens == 0

  def start_link(state \\ @default_state) do
    GenServer.start_link(__MODULE__, state, name: __MODULE__)
  end

  def allow_request? do
    GenServer.call(__MODULE__, :maybe_allow_request)
  end

  def init(state) do
    :timer.send_interval(state.refill_rate_in_ms, :refill)

    {:ok, state}
  end

  def handle_info(:refill, state) when is_full(state) do
    {:noreply, state}
  end

  def handle_info(:refill, state) when not is_full(state) do
    {:noreply, %{state | tokens: state.tokens + 1}}
  end

  def handle_call(:maybe_allow_request, _from, state) when is_empty(state) do
    {:reply, {false, state}, state}
  end

  def handle_call(:maybe_allow_request, _from, state) do
    state = %{state | tokens: state.tokens - 1}
    {:reply, {true, state}, state}
  end
end
