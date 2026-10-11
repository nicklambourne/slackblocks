%% Executable Erlang example. Called by EUnit and from the self-contained release.
-module(slackblocks_erlang).
-export([message/0, verify/0]).

-spec message() -> map().
message() ->
    {ok, Button} = 'Elixir.Slackblocks.Elements.Button':new([
        {text, <<"View build">>}, {action_id, <<"view_build">>}, {style, primary}
    ]),
    {ok, Section} = 'Elixir.Slackblocks.Blocks.Section':new([
        {text, <<"*Build complete*">>}, {accessory, Button}
    ]),
    {ok, Message} = 'Elixir.Slackblocks.Message':new([
        {channel, <<"C123">>}, {text, <<"Build complete">>}, {blocks, [Section]}
    ]),
    Message.

-spec verify() -> ok.
verify() ->
    Message = message(),
    {ok, Wire} = 'Elixir.Slackblocks':to_map(Message),
    <<"C123">> = maps:get(<<"channel">>, Wire),
    {ok, Json} = 'Elixir.Slackblocks':encode(Message),
    {ok, Wire} = 'Elixir.Jason':decode(Json),
    {ok, Message} = 'Elixir.Slackblocks':decode('Elixir.Slackblocks.Message', Json),
    {ok, ProtocolJson} = 'Elixir.Jason':encode(Message),
    {ok, Wire} = 'Elixir.Jason':decode(ProtocolJson),
    {ok, Edited} = 'Elixir.Slackblocks.Message':change(Message, [{text, <<"Updated">>}]),
    <<"Updated">> = maps:get(text, Edited),
    <<"Build complete">> = maps:get(text, Message),
    {error, #{category := missing_required, path := [<<"channel">>]}} =
        'Elixir.Slackblocks.Message':change(Message, [{channel, nil}]),
    {error, #{category := type_mismatch}} =
        'Elixir.Slackblocks':to_map(Message#{channel => "charlist"}),
    {ok, Task} = 'Elixir.Slackblocks.Blocks.TaskCard':new([
        {task_id, <<"queued">>}, {title, <<"Queued">>}, {status, pending}
    ]),
    {error, #{category := invalid_usage}} = 'Elixir.Slackblocks':to_map(Task),
    {ok, Plan} = 'Elixir.Slackblocks.Blocks.Plan':new([{title, <<"Plan">>}, {tasks, [Task]}]),
    {ok, #{<<"tasks">> := [TaskWire]}} = 'Elixir.Slackblocks':to_map(Plan),
    false = maps:is_key(<<"type">>, TaskWire),
    ok.
