# Multi-select parent compatibility probe

These eight native consumers emit the same **15 independent payloads**: five multi-select families under Actions, Section, and Input. They do not send anything to Slack. `C0123456` is a placeholder; supply a real authorized test destination to your transport when testing live. Keep the native payload unchanged when comparing parent types.

Install/build the corresponding implementation using its README. Run from the repository root, capturing stdout to `probe.jsonl`; compiler diagnostics belong on stderr. Python, TypeScript, Ruby and PHP can instead consume an installed package.

```sh
python docs/examples/multiselect-compat/python.py > probe.jsonl
node docs/examples/multiselect-compat/node.mjs > probe.jsonl
ruby docs/examples/multiselect-compat/ruby.rb > probe.jsonl
php docs/examples/multiselect-compat/php.php vendor/autoload.php > probe.jsonl
(cd docs/examples/multiselect-compat/go && go run .) > probe.jsonl
cargo run --quiet --manifest-path docs/examples/multiselect-compat/rust/Cargo.toml > probe.jsonl
dotnet build docs/examples/multiselect-compat/csharp/Multiselect.csproj
dotnet run --no-build --project docs/examples/multiselect-compat/csharp/Multiselect.csproj > probe.jsonl
```

For a source checkout, use `PYTHONPATH=python`, Ruby `-I ruby/lib`, PHP's Composer-generated autoloader, or `SLACKBLOCKS_MODULE=file:///path/to/typescript/dist/index.mjs`. The Go, Rust and C# projects explicitly reference this checkout, not a registry fallback. Put build caches and temporary output on your chosen build volume.

For Java, build `java/` first and set `JAVA_CP` to the candidate JAR plus its dependency classpath (Maven's `dependency:build-classpath` produces the latter). Use a disposable `EXAMPLE_CLASSES` directory:

```sh
javac -cp "$JAVA_CP" -d "$EXAMPLE_CLASSES" docs/examples/multiselect-compat/java/Multiselect.java
java -cp "$EXAMPLE_CLASSES:$JAVA_CP" Multiselect > probe.jsonl
```

After **each** language run:

```sh
python docs/examples/multiselect-compat/check.py probe.jsonl
```

The check asserts family/type, parent/surface, native static option content, IDs, and omission of unrelated modal/external-menu flags. A passing local check is not Slack API or client-rendering acceptance. See the [compatibility recipe](../../docs/recipes/multiselect-compatibility.mdx) for live comparison and reporting.
