defmodule Agov.CliSafety do
  @moduledoc false

  @allow_root_env "AGOV_ALLOW_ROOT"
  @allow_values ~w(1 true yes on)

  def ensure_not_root! do
    if blocked_root?(effective_uid(), System.get_env(@allow_root_env)) do
      Mix.shell().error("""
      ERR  AGF tooling refuses to run as root.

      Run the command as a normal project user. Root-owned build products,
      telemetry logs, and generated files can break later developer workflows.

      For a disposable CI/container job where UID 0 is intentional, set
      #{@allow_root_env}=1 for that invocation.
      """)

      exit({:shutdown, 126})
    end
  end

  def blocked_root?(0, allow_root_env), do: not allow_root?(allow_root_env)
  def blocked_root?(_uid, _allow_root_env), do: false

  def allow_root?(value) when is_binary(value) do
    value
    |> String.trim()
    |> String.downcase()
    |> then(&(&1 in @allow_values))
  end

  def allow_root?(_value), do: false

  def effective_uid do
    case :os.type() do
      {:unix, _} -> unix_uid()
      _ -> nil
    end
  end

  defp unix_uid do
    case System.cmd("id", ["-u"], stderr_to_stdout: true) do
      {output, 0} ->
        output
        |> String.trim()
        |> Integer.parse()
        |> case do
          {uid, ""} -> uid
          _ -> nil
        end

      _ ->
        nil
    end
  rescue
    _ -> nil
  end
end
