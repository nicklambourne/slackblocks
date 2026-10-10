# PHP implementation decisions

- PHP 8.2+ and 64-bit integers. CI tests 8.2–8.5; existing languages retain their support ranges.
- PSR-4 classes in `Slackblocks`, camelCase named constructor parameters, readonly public properties, native role interfaces and backed enums. Collection contracts use PHPDoc and runtime validation. Role interfaces describe library values; external implementations are not accepted as validated models.
- Optional null means omission; required fields fail. No implicit IDs. Message defaults follow the shared model (`text: ''`, `mrkdwn: true`); response defaults additionally use `in_channel` and `replace_original: false`.
- Strings supplied to text fields become the documented PlainText or MarkdownText class. Limits count Unicode code points. Invalid UTF-8 and nonfinite numbers fail.
- PHP double values retain native double semantics. Checked JSON rejects signed-integer overflow, nonfinite numbers and nonzero literals that underflow to zero. It does not promise arbitrary decimal precision. Opaque JSON uses the same numeric policy.
- JsonObject preserves JSON objects independently of PHP lists and copies mutable objects. Metadata and extension fields are opaque; extensions cannot shadow modeled fields or discriminators.
- A pending TaskCardBlock can be built for a PlanBlock. Plan serialization omits each task's type without changing the task; standalone pending serialization fails.
- `with()` applies named constructor arguments to a new value and validates it. Unknown fields fail; the original remains unchanged after either success or failure.
- Checked ingress uses Slack wire names, resolves role discriminators and emits categorized errors. Native constructors deliberately use PHP's native TypeError/ArgumentCountError for incompatible signatures; weak callers still follow PHP's scalar coercion rules.
- Model generation reads only the shared model/limits/vocabulary. Handwritten contextual rules follow modeled graph edges. Conformance constructions must stay independent of expected JSON.
- Composer distribution must contain only PHP package files. First publication is disabled pending the complete train and verified distribution repository/Packagist setup.

The implementation audit and full delivery plan are intentionally kept in task scratch at the owner's request.
