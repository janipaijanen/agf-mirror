defmodule Agov.ValidateInfosec do
  @moduledoc false

  alias Agov.ValidatorUtil

  @patterns [
    {"VCS token", ~r/(glpat-|ghp_|gho_|github_pat_|glcbt-|glrt-|xoxb-|xoxp-)[^\s"]{4,}/},
    {"database connection string", ~r/(postgres|mysql|redis|mongodb|amqp):\/\/[^\s"]*[:@]/i},
    {"API key or password",
     ~r/(api[_-]?key|secret[_-]?key|SECRET_KEY|password)\s*[:=]\s*[^\s"]{8,}/i},
    {"private key", ~r/BEGIN\s+(RSA |EC |DSA |OPENSSH )?PRIVATE KEY/},
    {"absolute filesystem path", ~r/\/(home|var|opt|srv|etc)\/[a-zA-Z0-9_\/.-]+/}
  ]

  def validate_file(path) do
    case File.read(path) do
      {:ok, content} ->
        findings =
          @patterns
          |> Enum.filter(fn {_label, regex} -> Regex.match?(regex, content) end)
          |> Enum.map(fn {label, _regex} -> %{message: "#{label} detected", location: path} end)

        if findings == [], do: {:ok, []}, else: {:error, findings}

      {:error, reason} ->
        {:error, [%{message: "file read error: #{inspect(reason)}", location: path}]}
    end
  end

  def validate_all(dir \\ nil) do
    target = dir || ValidatorUtil.learnings_dir()

    target
    |> ValidatorUtil.yaml_files()
    |> Enum.map(fn file -> {file, validate_file(file)} end)
    |> split_results()
  end

  defp split_results(results) do
    errors =
      results
      |> Enum.flat_map(fn
        {file, {:error, findings}} -> [%{file: file, findings: findings}]
        {_file, {:ok, []}} -> []
      end)

    if errors == [], do: {:ok, Enum.map(results, &elem(&1, 0))}, else: {:error, errors}
  end
end
