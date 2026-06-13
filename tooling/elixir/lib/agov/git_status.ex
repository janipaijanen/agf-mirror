defmodule Agov.GitStatus do
  @moduledoc false

  @callback current_branch() :: {:ok, String.t()} | {:error, term()}
  @callback dirty_files() :: {:ok, [String.t()]} | {:error, term()}
  @callback branch_local_files(String.t(), [String.t()]) ::
              {:ok, [String.t()]} | {:skip, term()} | {:error, term()}

  def current_branch do
    case git(["branch", "--show-current"]) do
      {:ok, ""} -> {:ok, "detached HEAD"}
      {:ok, branch} -> {:ok, branch}
      {:error, _} = error -> error
    end
  end

  def dirty_files do
    case git_raw(["status", "--porcelain", "--untracked-files=all"]) do
      {:ok, output} ->
        files =
          output
          |> String.split("\n", trim: true)
          |> Enum.map(&String.slice(&1, 3..-1//1))
          |> Enum.reject(&(&1 == ""))

        {:ok, files}

      {:error, _} = error ->
        error
    end
  end

  def branch_local_files(base_branch, paths) when is_binary(base_branch) and base_branch != "" do
    with {:ok, root} <- git(["rev-parse", "--show-toplevel"]),
         :ok <- verify_ref(base_branch),
         {:ok, base_files} <- ls_tree(root, base_branch, paths),
         {:ok, head_files} <- ls_tree(root, "HEAD", paths) do
      base_set = MapSet.new(base_files)

      local =
        head_files
        |> Enum.reject(&MapSet.member?(base_set, &1))
        |> Enum.sort()

      {:ok, local}
    else
      {:skip, _} = skip -> skip
      {:error, _} = error -> error
    end
  end

  def branch_local_files(_base_branch, _paths), do: {:skip, :base_branch_not_configured}

  defp verify_ref(ref) do
    case System.cmd("git", ["rev-parse", "--verify", "#{ref}^{commit}"], stderr_to_stdout: true) do
      {_output, 0} -> :ok
      {_output, _} -> {:skip, :base_branch_unavailable}
    end
  end

  defp ls_tree(root, ref, paths) do
    args = ["ls-tree", "-r", "--name-only", ref, "--" | paths]

    case System.cmd("git", args, cd: root, stderr_to_stdout: true) do
      {output, 0} ->
        files =
          output
          |> String.split("\n", trim: true)
          |> Enum.sort()

        {:ok, files}

      {output, status} ->
        {:error, {:git_failed, status, String.trim(output)}}
    end
  end

  defp git(args) do
    case System.cmd("git", args, stderr_to_stdout: true) do
      {output, 0} -> {:ok, String.trim(output)}
      {output, status} -> {:error, {:git_failed, status, String.trim(output)}}
    end
  end

  defp git_raw(args) do
    case System.cmd("git", args, stderr_to_stdout: true) do
      {output, 0} -> {:ok, output}
      {output, status} -> {:error, {:git_failed, status, String.trim(output)}}
    end
  end
end
