using System;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
using Slackblocks.Internal;

namespace Slackblocks;

/// <summary>A value that can be written as Slack Block Kit JSON.</summary>
/// <remarks>
/// Every slackblocks type implements this interface, including the role interfaces such as
/// <see cref="Blocks.IBlock"/>, so a collection of blocks can be serialized without knowing each
/// concrete type.
/// </remarks>
[JsonConverter(typeof(SlackObjectJsonConverterFactory))]
public interface ISlackObject
{
    /// <summary>Returns this value as compact Slack JSON.</summary>
    /// <returns>Compact JSON text.</returns>
    string ToJson();

    /// <summary>Returns a mutable copy of this value's Slack JSON.</summary>
    /// <remarks>Changing the returned node does not change this value.</remarks>
    /// <returns>A new JSON object.</returns>
    JsonObject ToJsonNode();
}

/// <summary>
/// Base class for every immutable slackblocks value. A value is validated once, when it is
/// constructed, and cannot change afterwards.
/// </summary>
/// <remarks>
/// Two values are equal when they are the same type and produce the same Slack JSON, ignoring
/// object key order. <see cref="object.ToString"/> returns the compact JSON.
/// </remarks>
[JsonConverter(typeof(SlackObjectJsonConverterFactory))]
public abstract class SlackObject : ISlackObject, IEquatable<SlackObject>
{
    private JsonObject wire = new();
    private string json = "{}";
    private string? canonical;

    /// <summary>Initializes the base state; derived constructors call <see cref="Initialize"/>.</summary>
    private protected SlackObject()
    {
    }

    /// <summary>The validated wire object. Callers inside the library must not mutate it.</summary>
    internal JsonObject Wire => wire;

    /// <inheritdoc/>
    public string ToJson() => json;

    /// <inheritdoc/>
    public JsonObject ToJsonNode() => (JsonObject)wire.DeepClone();

    /// <inheritdoc/>
    public bool Equals(SlackObject? other) =>
        other is not null
        && (ReferenceEquals(this, other)
            || (other.GetType() == GetType() && Canonical() == other.Canonical()));

    /// <inheritdoc/>
    public override bool Equals(object? obj) => Equals(obj as SlackObject);

    /// <inheritdoc/>
    public override int GetHashCode() => HashCode.Combine(GetType(), Canonical());

    /// <summary>Returns this value's compact Slack JSON.</summary>
    /// <returns>Compact JSON text.</returns>
    public override string ToString() => json;

    /// <summary>Stores the validated wire object built by a derived constructor.</summary>
    private protected void Initialize(JsonObject validated)
    {
        wire = validated;
        json = SlackJson.Write(validated);
    }

    private string Canonical() => canonical ??= SlackJson.Canonical(wire);
}
