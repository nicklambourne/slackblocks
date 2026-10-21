# Erlang consumer

This example calls the Elixir implementation directly, using quoted module atoms,
UTF-8 binaries, keyword tuples and `{ok, Value}` / `{error, Reason}` results.
No additional Slackblocks package or compatibility wrapper is needed.

Build prerequisites: a compatible Elixir/Mix and Erlang/OTP pair, Hex installed
with `mix local.hex`, and Rebar3. The example pins `rebar_mix` 0.5.1 and declares
the application dependency so releases include Elixir, Jason and Decimal.

The package is not published yet. `tools/erlang_check.py` builds a Hex archive,
serves that exact tarball on localhost using the plugin's Hex fetch resource, executes
EUnit, consolidates protocols and builds/runs an OTP release. The build uses the distributable, never the monorepo sources. The local fetch
resource avoids a Rebar3 3.27.1 bug discovering unnamed Mix checkout applications.
After publication,
ordinary consumers use the released Hex version in `rebar.config` without the local fetch resource.

The released `rebar_mix` 0.5.1 plugin predates current Mix lock tuples. This
example therefore declares Jason and Decimal explicitly in Rebar3 as well.
Keep these versions compatible with the package requirements; the consumer tests
exercise both supported Decimal majors.

The release test starts with build tools removed from PATH. Runtime libraries and
ERTS are bundled, so deployment does not require a separate Elixir installation.
See `src/slackblocks_erlang.erl` for the runnable message example and tagged errors.
