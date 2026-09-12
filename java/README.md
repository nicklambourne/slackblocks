# slackblocks for Java

[![Maven Central](https://img.shields.io/maven-central/v/io.github.nicklambourne/slackblocks?logo=apachemaven)](https://central.sonatype.com/artifact/io.github.nicklambourne/slackblocks)
[![Java CI](https://github.com/nicklambourne/slackblocks/actions/workflows/java.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions/workflows/java.yml)

Validated, fluent Slack Block Kit construction for Java 17 and newer.

The Java artifact uses the same version as the Python, TypeScript, and Go packages and is released only as part of the coordinated slackblocks release train.

## Installation

Maven:

```xml
<dependency>
  <groupId>io.github.nicklambourne</groupId>
  <artifactId>slackblocks</artifactId>
  <version>2.2.0</version>
</dependency>
```

Gradle:

```kotlin
implementation("io.github.nicklambourne:slackblocks:2.2.0")
```

## Build a block

Every public value is immutable. Start from the concrete type's `builder()`, use named fluent methods, and call `build()` to validate and materialise it:

```java
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.element.ButtonElement;

SectionBlock block = SectionBlock.builder()
    .markdownText("A deployment is ready for review.")
    .accessory(ButtonElement.builder()
        .text("Review")
        .actionId("review_deployment")
        .value("deploy-482")
        .build())
    .build();
```

Known Slack limits, required fields, mutually exclusive fields, and composition restrictions are checked at `build()`. A failure throws `ValidationException` with a stable `ErrorCategory` and field path.

## Send with the official Slack Java SDK

Built blocks implement `com.slack.api.model.block.LayoutBlock` directly, so no adapter or transport wrapper is needed. Keep delivery concerns on the Slack SDK request:

```java
import com.slack.api.Slack;
import com.slack.api.methods.request.chat.ChatPostMessageRequest;
import java.util.List;

var client = Slack.getInstance().methods(System.getenv("SLACK_API_TOKEN"));
var response = client.chatPostMessage(ChatPostMessageRequest.builder()
    .channel("C0123456")
    .text("A deployment is ready for review")
    .blocks(List.of(block))
    .build());
```

Add `com.slack.api:slack-api-client` to your application when using `Slack` or `MethodsClient`. slackblocks itself depends only on the SDK model artifact for native type compatibility.

## JSON and complete payloads

Every completed value implements `SlackObject`:

```java
String json = block.toJson();
```

Use `MessagePayload`, `WebhookMessage`, `MessageResponse`, `ModalView`, and `HomeTabView` when you need a complete JSON payload rather than an SDK request. `SlackblocksJson.write(value)` is also available explicitly.

## Higher-level components

`Paginator` and `Accordion` use concrete builders and expand to ordinary `List<Block>` values. They introduce no separate runtime or rendering model.

## Compatibility and conformance

- Java 17 or newer.
- Versioned in lockstep with every supported slackblocks language.
- All shared valid fixtures and invalid-case categories are mandatory; the Java skip list is empty.
- Public builders are generated from the shared model metadata, while Java naming and SDK integration remain language-native.

## Development

```bash
cd java
./mvnw clean verify
```

The build compiles with `-Xlint:all -Werror`, runs the complete conformance suite, applies Javadoc doclint, and creates the binary, source, and Javadoc JARs required by Maven Central.

See the [Java documentation](https://nicklambourne.github.io/slackblocks/?language=java), [API reference](https://nicklambourne.github.io/slackblocks/reference/java), and repository-level [contributing guide](https://nicklambourne.github.io/slackblocks/contributing).
