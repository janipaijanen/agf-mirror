defmodule Agov.ClaimFSM do
  @moduledoc """
  Pure state machine for one coordination artifact claim.

  States:

  * `:unclaimed`
  * `{:active, entry}`
  * `{:stale, entry}`
  * `{:released, entry}`

  Events:

  * `{:claim, from, seq, claimed_at, expires_on}`
  * `{:complete, from}`
  * `{:tick, now}`
  """

  def transition(:unclaimed, {:claim, from, seq, claimed_at, expires_on}, now) do
    classify(claim_entry(from, seq, claimed_at, expires_on), now)
  end

  def transition({:active, _entry}, {:claim, from, seq, claimed_at, expires_on}, now) do
    classify(claim_entry(from, seq, claimed_at, expires_on), now)
  end

  def transition({:stale, _entry}, {:claim, from, seq, claimed_at, expires_on}, now) do
    classify(claim_entry(from, seq, claimed_at, expires_on), now)
  end

  def transition({:active, %{from: from} = entry}, {:complete, from}, _now) do
    {:released, entry}
  end

  def transition({:stale, %{from: from} = entry}, {:complete, from}, _now) do
    {:released, entry}
  end

  def transition({:active, %{from: from}} = state, {:complete, other}, _now)
      when other != from do
    state
  end

  def transition({:stale, %{from: from}} = state, {:complete, other}, _now)
      when other != from do
    state
  end

  def transition(:unclaimed, {:complete, _from}, _now) do
    :unclaimed
  end

  def transition({:active, %{expires_on: nil}} = state, {:tick, _event_now}, _now) do
    state
  end

  def transition({:active, %{expires_on: expires_on} = entry} = state, {:tick, event_now}, _now) do
    case DateTime.compare(expires_on, event_now) do
      :lt -> {:stale, entry}
      _ -> state
    end
  end

  def transition({:stale, _entry} = state, {:tick, _event_now}, _now) do
    state
  end

  def transition(
        {:released, _entry} = state,
        {:claim, _from, _seq, _claimed_at, _expires_on},
        _now
      ) do
    state
  end

  def transition({:released, _entry} = state, {:complete, _from}, _now) do
    state
  end

  def transition({:released, _entry} = state, {:tick, _event_now}, _now) do
    state
  end

  defp claim_entry(from, seq, claimed_at, expires_on) do
    %{from: from, seq: seq, claimed_at: claimed_at, expires_on: expires_on}
  end

  defp classify(%{expires_on: nil} = entry, _now), do: {:active, entry}

  defp classify(%{expires_on: expires_on} = entry, now) do
    case DateTime.compare(expires_on, now) do
      :lt -> {:stale, entry}
      _ -> {:active, entry}
    end
  end
end
