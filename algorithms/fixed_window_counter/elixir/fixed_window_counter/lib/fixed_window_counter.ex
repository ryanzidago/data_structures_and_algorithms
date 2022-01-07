defmodule FixedWindowCounter do
  use GenServer

  @interval_in_ms 100
  @window_size_in_ms 1_000
  @max_requests_per_window_size 5

  defguard is_expired(pointer) when pointer >= @window_size_in_ms

  def start_link, do: GenServer.start_link(__MODULE__, %{counter: 0, t: 0}, name: __MODULE__)
  def allow_request, do: GenServer.call(__MODULE__, :allow_request)

  def init(state) do
    :timer.send_interval(@interval_in_ms, __MODULE__, :window)
    {:ok, state}
  end

  def handle_info(:window, state) when is_expired(state.t) do
    state = %{counter: 0, t: 0}
    {:noreply, state}
  end

  def handle_info(:window, state) do
    state = %{state | t: state.t + @interval_in_ms}
    {:noreply, state}
  end

  def handle_call(:allow_request, _from, state)
      when state.counter == @max_requests_per_window_size do
    {:reply, :too_many_request, state}
  end

  def handle_call(:allow_request, _from, state) do
    state = %{state | counter: state.counter + 1}
    {:reply, state, state}
  end
end
