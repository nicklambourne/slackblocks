# slackblocks implementation principles

1. Idiomatic APIs take priority over uniform APIs. Names, casing, and construction patterns may differ by language.
2. Serialized output must be semantically identical for every shared valid fixture.
3. Validation outcomes must agree on rejection and error category; concrete error types and messages are language-specific.
4. Feature gaps must be explicit in a checked-in conformance skip list.
5. Every implementation must declare the spec version it conforms to.
