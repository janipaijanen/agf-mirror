defmodule Agov.ValidateReporter do
  @moduledoc false

  def report_file_result(path, {:ok, warnings}) when is_list(warnings) do
    Enum.each(warnings, &warn(path, &1))
    Mix.shell().info("OK #{path}")
    :ok
  end

  def report_file_result(path, {:ok, _data}) do
    Mix.shell().info("OK #{path}")
    :ok
  end

  def report_file_result(path, {:error, errors}) when is_list(errors) do
    Enum.each(errors, &err(path, &1))
    :error
  end

  def report_collection_result(label, {:ok, files}) when is_list(files) do
    Enum.each(files, &Mix.shell().info("OK #{&1}"))
    Mix.shell().info("OK #{label}")
    :ok
  end

  def report_collection_result(label, {:error, file_errors}) do
    Enum.each(file_errors, &report_file_errors/1)
    Mix.shell().error("ERR #{label}")
    :error
  end

  def report_warning_collection(label, {:ok, warnings}) do
    Enum.each(warnings, fn warning ->
      warn(Map.get(warning, :file, label), warning)
    end)

    Mix.shell().info("OK #{label}")
    :ok
  end

  def report_single_result(label, {:ok, :skipped}) do
    Mix.shell().info("OK #{label}: skipped")
    :ok
  end

  def report_single_result(label, {:ok, warnings}) when is_list(warnings) do
    Enum.each(warnings, &warn(label, &1))
    Mix.shell().info("OK #{label}")
    :ok
  end

  def report_single_result(label, {:ok, _data}) do
    Mix.shell().info("OK #{label}")
    :ok
  end

  def report_single_result(label, {:error, errors}) when is_list(errors) do
    Enum.each(errors, &err(label, &1))
    :error
  end

  defp report_file_errors(%{file: file, errors: errors}) do
    Enum.each(errors, &err(file, &1))
  end

  defp report_file_errors(%{file: file, findings: findings}) do
    Enum.each(findings, &err(file, &1))
  end

  defp err(file, %{message: message, location: location}) do
    Mix.shell().error("ERR #{file}: #{location}: #{message}")
  end

  defp err(file, message), do: Mix.shell().error("ERR #{file}: #{inspect(message)}")

  defp warn(file, %{message: message, location: location}) do
    Mix.shell().info("WARN #{file}: #{location}: #{message}")
  end

  defp warn(file, message), do: Mix.shell().info("WARN #{file}: #{inspect(message)}")
end
