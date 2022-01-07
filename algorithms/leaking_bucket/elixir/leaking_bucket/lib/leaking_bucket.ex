defmodule LeakingBucket do
  use GenServer

  require Logger

  @bucket_size 10
  @processing_rate_in_ms 1_000

  alias Zipper.ListZipper

  defguard is_full(bucket) when length(elem(bucket, 0)) + length(elem(bucket, 1)) == @bucket_size
  defguard is_empty(bucket) when length(elem(bucket, 0)) + length(elem(bucket, 1)) == 0

  def start_link do
    GenServer.start_link(__MODULE__, {ListZipper.new(), self()}, name: __MODULE__)
  end

  def fill_bucket(request) do
    GenServer.call(__MODULE__, {:fill_bucket, request})
  end

  def init({bucket, from}) do
    :timer.send_interval(@processing_rate_in_ms, __MODULE__, {:leak, from})
    {:ok, bucket}
  end

  def handle_call({:fill_bucket, _request}, _from, bucket) when is_full(bucket) do
    {:reply, :too_many_requests, bucket}
  end

  def handle_call({:fill_bucket, request}, _from, bucket) do
    bucket = ListZipper.put(bucket, request)
    {:reply, bucket, bucket}
  end

  def handle_info({:leak, _from}, bucket) when is_empty(bucket) do
    {:noreply, bucket}
  end

  def handle_info({:leak, from}, bucket) do
    {request, bucket} = ListZipper.pop(bucket)
    send(from, request)

    Logger.info("Sent #{inspect(request)} to #{inspect(from)}")

    {:noreply, bucket}
  end
end
