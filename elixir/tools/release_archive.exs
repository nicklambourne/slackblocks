# Hex 2.5.1 expands directory entries in filesystem order. Sort new artifacts,
# and replay an existing artifact's order when reproducing immutable releases.
[output | reference] = System.argv()
Hex.start()
build = Mix.Tasks.Hex.Build.prepare_package()

files =
  case reference do
    [] ->
      Enum.sort(build.meta.files)

    [metadata_path] ->
      {:ok, terms} = :file.consult(String.to_charlist(metadata_path))
      {"files", recorded} = List.keyfind(terms, "files", 0)

      unless Enum.sort(recorded) == Enum.sort(build.meta.files) and
               length(Enum.uniq(recorded)) == length(recorded),
             do: Mix.raise("Tested Hex file inventory differs from source")

      recorded
  end

meta = Map.put(build.meta, :files, files)

Mix.Tasks.Hex.Build.print_info(
  meta,
  build.organization,
  build.exclude_deps,
  build.package[:files]
)

%{outer_checksum: checksum} = Hex.Tar.create!(meta, files, output)
IO.puts("Hex archive checksum: " <> Base.encode16(checksum, case: :lower))
