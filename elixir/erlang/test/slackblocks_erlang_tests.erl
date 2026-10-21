-module(slackblocks_erlang_tests).
-include_lib("eunit/include/eunit.hrl").

consumer_test() ->
    {ok, _} = application:ensure_all_started(slackblocks),
    ?assertEqual(ok, slackblocks_erlang:verify()).
