using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;

namespace Slackblocks.Tests;

/// <summary>
/// Rejects every shared invalid case through the public constructors. The constructions mirror
/// the Java harness case for case, so both implementations are checked against the same inputs.
/// </summary>
public sealed class InvalidConformanceTests
{
    private static readonly Dictionary<string, string> CSharpTypes = new(StringComparer.Ordinal)
    {
        ["Button"] = "ButtonElement",
        ["Checkboxes"] = "CheckboxesElement",
        ["FeedbackButtons"] = "FeedbackButtonsElement",
        ["FileInput"] = "FileInputElement",
        ["HomeTab"] = "HomeTabView",
        ["IconButton"] = "IconButtonElement",
        ["Message"] = "MessagePayload",
        ["Modal"] = "ModalView",
        ["NumberInput"] = "NumberInputElement",
        ["Overflow"] = "OverflowElement",
        ["PlainTextInput"] = "PlainTextInputElement",
        ["RadioButtons"] = "RadioButtonsElement",
        ["StaticSelect"] = "StaticSelectElement",
        ["URLSource"] = "UrlSource",
    };

    public static TheoryData<string, string> InvalidCases()
    {
        var data = new TheoryData<string, string>();
        foreach (var entry in Manifest()["cases"]!.AsArray())
        {
            data.Add(entry!["id"]!.GetValue<string>(), entry["category"]!.GetValue<string>());
        }

        return data;
    }

    [Fact]
    public void ImplementsTheSharedSpecificationVersion() =>
        Assert.Equal(SlackblocksInfo.SpecVersion, Manifest()["spec_version"]!.GetValue<string>());

    [Theory]
    [MemberData(nameof(InvalidCases))]
    public void EverySharedInvalidCaseIsRejectedByTheConstructors(string id, string category)
    {
        var invalid = InvalidValue(id);
        var driver = new FixtureDriver();
        var type = driver.Type(CSharpTypes.GetValueOrDefault(invalid.Name, invalid.Name));

        var error = Assert.Throws<ValidationException>(() => driver.Build(ToNode(invalid.Value), type, invalid.Name));

        Assert.Equal(category, error.Category.ToWireValue());
        Assert.False(string.IsNullOrWhiteSpace(error.Path), "validation errors must carry a path");
    }

    private static JsonObject Manifest() =>
        JsonNode.Parse(File.ReadAllText(Repository.PathTo("spec", "fixtures", "invalid", "manifest.json")))!.AsObject();

    private static Invalid InvalidValue(string id) =>
        id switch
        {
            "text-empty" => Case("PlainText", TextObject("plain_text", "")),
            "text-too-long" => Case("PlainText", TextObject("plain_text", Repeated("x", 3001))),
            "button-action-id-too-long" => Typed("Button", "button", "text", Plain("A"), "action_id", Repeated("x", 256)),
            "button-text-too-long" => Typed("Button", "button", "text", Plain(Repeated("x", 76)), "action_id", "a"),
            "button-url-too-long" => Typed("Button", "button", "text", Plain("A"), "action_id", "a", "url", Repeated("x", 3001)),
            "button-value-too-long" => Typed("Button", "button", "text", Plain("A"), "action_id", "a", "value", Repeated("x", 2001)),
            "confirmation-title-too-long" => Confirmation(Repeated("x", 101), "Text", "Yes", "No"),
            "confirmation-text-too-long" => Confirmation("Title", Repeated("x", 301), "Yes", "No"),
            "confirmation-confirm-too-long" => Confirmation("Title", "Text", Repeated("x", 31), "No"),
            "confirmation-deny-too-long" => Confirmation("Title", "Text", "Yes", Repeated("x", 31)),
            "option-text-too-long" => Case("Option", Option(Repeated("x", 76), "a")),
            "option-value-too-long" => Case("Option", Option("A", Repeated("x", 151))),
            "option-url-too-long" => Case("Option", Map("text", Plain("A"), "value", "a", "url", Repeated("🙂", 3001))),
            "option-description-too-long" => Case("Option", Map("text", Plain("A"), "value", "a", "description", Plain(Repeated("x", 76)))),
            "option-group-label-too-long" => OptionGroup(Repeated("x", 76), Items(Option("A", "a"))),
            "option-group-empty" => OptionGroup("Group", Items()),
            "option-group-too-many-options" => OptionGroup("Group", Copies(101, index => Option("A", "a"))),
            "select-placeholder-too-long" => Typed("StaticSelect", "static_select", "action_id", "a", "options", Items(Option("A", "a")), "placeholder", Plain(Repeated("x", 151))),
            "plain-text-input-placeholder-too-long" => Typed("PlainTextInput", "plain_text_input", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "email-input-placeholder-too-long" => Typed("EmailInputElement", "email_text_input", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "url-input-placeholder-too-long" => Typed("UrlInputElement", "url_text_input", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "number-input-placeholder-too-long" => Typed("NumberInput", "number_input", "action_id", "a", "is_decimal_allowed", false, "placeholder", Plain(Repeated("x", 151))),
            "date-picker-placeholder-too-long" => Typed("DatePickerElement", "datepicker", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "time-picker-placeholder-too-long" => Typed("TimePickerElement", "timepicker", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "rich-text-input-placeholder-too-long" => Typed("RichTextInputElement", "rich_text_input", "action_id", "a", "placeholder", Plain(Repeated("x", 151))),
            "select-too-many-options" => Typed("StaticSelect", "static_select", "action_id", "a", "options", Copies(101, index => Option("A", "a"))),
            "select-too-many-option-groups" => Typed("StaticSelect", "static_select", "action_id", "a", "option_groups", Copies(101, index => Map("label", Plain("Group"), "options", Items(Option("A", "a"))))),
            "overflow-empty" => Typed("Overflow", "overflow", "action_id", "a", "options", Items()),
            "overflow-too-many-options" => Typed("Overflow", "overflow", "action_id", "a", "options", Copies(6, index => Option("A", "a"))),
            "checkboxes-empty" => Typed("Checkboxes", "checkboxes", "action_id", "a", "options", Items()),
            "checkboxes-too-many-options" => Typed("Checkboxes", "checkboxes", "action_id", "a", "options", Copies(11, index => Option("A", "a"))),
            "radio-buttons-empty" => Typed("RadioButtons", "radio_buttons", "action_id", "a", "options", Items()),
            "radio-buttons-too-many-options" => Typed("RadioButtons", "radio_buttons", "action_id", "a", "options", Copies(11, index => Option("A", "a"))),
            "url-source-url-empty" => Typed("URLSource", "url", "url", "", "text", "text"),
            "url-source-url-too-long" => Typed("URLSource", "url", "url", Repeated("x", 3001), "text", "text"),
            "table-too-many-rows" => Typed("TableBlock", "table", "rows", Copies(101, index => Items(RawText("A")))),
            "table-too-many-columns" => Typed("TableBlock", "table", "rows", Items(Copies(21, index => RawText("A")))),
            "table-ragged-rows" => Typed("TableBlock", "table", "rows", Items(Items(RawText("A"), RawText("B")), Items(RawText("C")))),
            "table-column-settings-mismatch" => Typed("TableBlock", "table", "rows", Items(Items(RawText("A"), RawText("B"))), "column_settings", Items(Map("is_wrapped", true))),
            "file-input-max-files-too-small" => Typed("FileInput", "file_input", "action_id", "a", "max_files", 0),
            "file-input-max-files-too-large" => Typed("FileInput", "file_input", "action_id", "a", "max_files", 11),
            "dispatch-action-no-triggers" => Case("DispatchActionConfiguration", Map("trigger_actions_on", Items())),
            "dispatch-action-too-many-triggers" => Case("DispatchActionConfiguration", Map("trigger_actions_on", Items("on_enter_pressed", "on_character_entered", "on_enter_pressed"))),
            "plain-text-input-max-length-too-large" => Typed("PlainTextInput", "plain_text_input", "action_id", "a", "max_length", 3001),
            "actions-too-many-elements" => Typed("ActionsBlock", "actions", "elements", Copies(26, index => Button())),
            "context-too-many-elements" => Typed("ContextBlock", "context", "elements", Copies(11, index => Mrkdwn("A"))),
            "header-text-too-long" => Typed("HeaderBlock", "header", "text", Plain(Repeated("x", 151))),
            "image-url-too-long" => Typed("ImageBlock", "image", "image_url", Repeated("x", 3001), "alt_text", "Alt"),
            "image-alt-text-too-long" => Typed("ImageBlock", "image", "image_url", "https://example.com/image.png", "alt_text", Repeated("x", 2001)),
            "input-label-too-long" => Typed("InputBlock", "input", "label", Plain(Repeated("x", 2001)), "element", InputElement()),
            "input-hint-too-long" => Typed("InputBlock", "input", "label", Plain("Label"), "hint", Plain(Repeated("x", 2001)), "element", InputElement()),
            "markdown-empty" => Typed("MarkdownBlock", "markdown", "text", ""),
            "markdown-too-long" => Typed("MarkdownBlock", "markdown", "text", Repeated("x", 12001)),
            "section-text-too-long" => Typed("SectionBlock", "section", "text", Mrkdwn(Repeated("x", 3001))),
            "section-too-many-fields" => Typed("SectionBlock", "section", "fields", Copies(11, index => Mrkdwn("x"))),
            "section-field-too-long" => Typed("SectionBlock", "section", "fields", Items(Mrkdwn(Repeated("x", 2001)))),
            "video-alt-text-empty" => InvalidVideo("alt_text", ""),
            "video-alt-text-too-long" => InvalidVideo("alt_text", Repeated("x", 201)),
            "video-title-too-long" => InvalidVideo("title", Plain(Repeated("x", 201))),
            "video-author-name-too-long" => InvalidVideo("author_name", Repeated("x", 51)),
            "video-description-too-long" => InvalidVideo("description", Plain(Repeated("x", 201))),
            "video-provider-name-too-long" => InvalidVideo("provider_name", Repeated("x", 51)),
            "message-channel-empty" => Case("Message", Map("channel", "")),
            "message-too-many-blocks" => Case("Message", Map("channel", "C123", "blocks", Copies(51, index => Divider()))),
            "message-too-many-attachments" => Case("Message", Map("channel", "C123", "attachments", Copies(101, index => Map("blocks", Items(Divider()))))),
            "message-invalid-block-surface" => Case("Message", Map("channel", "C123", "blocks", Items(Obj("alert", "text", Plain("Modal only"))))),
            "modal-invalid-block-surface" => Typed("Modal", "modal", "title", Plain("Invalid"), "blocks", Items(Obj("markdown", "text", "Message only"))),
            "home-invalid-block-surface" => Typed("HomeTab", "home", "blocks", Items(Obj("alert", "text", Plain("Modal only")))),
            "modal-input-requires-submit" => Typed("Modal", "modal", "title", Plain("Missing submit"), "blocks", Items(InputBlock())),
            "view-missing-blocks" => Typed("HomeTab", "home", "blocks", Items()),
            "view-too-many-blocks" => Typed("HomeTab", "home", "blocks", Copies(101, index => Divider())),
            "view-private-metadata-too-long" => Typed("HomeTab", "home", "blocks", Items(Divider()), "private_metadata", Repeated("x", 3001)),
            "view-callback-id-too-long" => Typed("HomeTab", "home", "blocks", Items(Divider()), "callback_id", Repeated("x", 256)),
            "view-title-too-long" => Typed("Modal", "modal", "title", Plain(Repeated("x", 25)), "blocks", Items(Divider())),
            "view-close-too-long" => Typed("Modal", "modal", "title", Plain("Title"), "close", Plain(Repeated("x", 25)), "blocks", Items(Divider())),
            "view-submit-too-long" => Typed("Modal", "modal", "title", Plain("Title"), "submit", Plain(Repeated("x", 25)), "blocks", Items(Divider())),
            "section-missing-content" => Typed("SectionBlock", "section"),
            "section-empty-fields" => Typed("SectionBlock", "section", "fields", Items()),
            "static-select-options-and-groups" => Typed("StaticSelect", "static_select", "action_id", "a", "options", Items(Option("A", "a")), "option_groups", Items(Map("label", Plain("A"), "options", Items(Option("B", "b"))))),
            "image-url-and-slack-file" => Typed("ImageElement", "image", "alt_text", "image", "image_url", "https://example.com/image.png", "slack_file", Map("id", "F123")),
            "number-input-inverted-range" => Typed("NumberInput", "number_input", "action_id", "a", "is_decimal_allowed", true, "min_value", 2, "max_value", 1),
            "context-invalid-element" => Typed("ContextBlock", "context", "elements", Items(Divider())),
            "input-invalid-element" => Typed("InputBlock", "input", "label", Plain("Label"), "element", Button()),
            "block-id-too-long" => Typed("DividerBlock", "divider", "block_id", Repeated("x", 256)),
            "button-accessibility-label-too-long" => Typed("Button", "button", "text", Plain("A"), "action_id", "a", "accessibility_label", Repeated("x", 76)),
            "workflow-button-text-too-long" => Typed("WorkflowButtonElement", "workflow_button", "text", Plain(Repeated("x", 76)), "workflow", Workflow()),
            "workflow-button-accessibility-label-too-long" => Typed("WorkflowButtonElement", "workflow_button", "text", Plain("Run"), "workflow", Workflow(), "accessibility_label", Repeated("x", 76)),
            "alert-text-too-long" => Typed("AlertBlock", "alert", "text", Plain(Repeated("x", 201))),
            "card-title-too-long" => Typed("CardBlock", "card", "title", Plain(Repeated("x", 151))),
            "card-subtitle-too-long" => Typed("CardBlock", "card", "title", Plain("Card"), "subtitle", Plain(Repeated("x", 151))),
            "card-body-too-long" => Typed("CardBlock", "card", "body", Plain(Repeated("x", 201))),
            "card-too-many-actions" => Typed("CardBlock", "card", "title", Plain("Card"), "actions", Copies(4, index => Button())),
            "card-subtext-too-long" => Typed("CardBlock", "card", "title", Plain("Card"), "subtext", Plain(Repeated("x", 201))),
            "carousel-empty" => Typed("CarouselBlock", "carousel", "elements", Items()),
            "carousel-too-many-cards" => Typed("CarouselBlock", "carousel", "elements", Copies(11, index => Card())),
            "container-title-too-long" => Typed("ContainerBlock", "container", "title", Plain(Repeated("x", 151)), "child_blocks", Items(Divider())),
            "container-subtitle-too-long" => Typed("ContainerBlock", "container", "title", Plain("Container"), "subtitle", Plain(Repeated("x", 151)), "child_blocks", Items(Divider())),
            "container-too-many-child-blocks" => Typed("ContainerBlock", "container", "title", Plain("Container"), "child_blocks", Copies(11, index => Divider())),
            "context-actions-too-many-elements" => Typed("ContextActionsBlock", "context_actions", "elements", Copies(6, index => IconButton())),
            "feedback-button-text-too-long" => Typed("FeedbackButtons", "feedback_buttons", "positive_button", Feedback(Repeated("x", 76), "good"), "negative_button", Feedback("Bad", "bad")),
            "feedback-button-value-too-long" => Typed("FeedbackButtons", "feedback_buttons", "positive_button", Feedback("Good", Repeated("x", 2001)), "negative_button", Feedback("Bad", "bad")),
            "feedback-button-accessibility-label-too-long" => Typed("FeedbackButtons", "feedback_buttons", "positive_button", Map("text", Plain("Good"), "value", "good", "accessibility_label", Repeated("x", 76)), "negative_button", Feedback("Bad", "bad")),
            "icon-button-too-many-visible-users" => Typed("IconButton", "icon_button", "text", Plain("Delete"), "icon", "trash", "visible_to_user_ids", Copies(11, index => "U" + index)),
            "icon-button-value-too-long" => Typed("IconButton", "icon_button", "text", Plain("Delete"), "icon", "trash", "value", Repeated("x", 2001)),
            "icon-button-accessibility-label-too-long" => Typed("IconButton", "icon_button", "text", Plain("Delete"), "icon", "trash", "accessibility_label", Repeated("x", 76)),
            "data-table-too-few-rows" => DataTable(Items(Items(RawText("Name"))), "Names"),
            "data-table-too-many-rows" => DataTable(Copies(202, index => Items(RawText("A"))), "Names"),
            "data-table-too-few-columns" => DataTable(Items(Items(), Items()), "Empty"),
            "data-table-too-many-columns" => DataTable(Copies(2, row => Copies(21, column => RawText("A"))), "Wide"),
            "data-table-page-size-too-small" => DataTable(ValidRows(), "Names", "page_size", 0),
            "data-table-page-size-too-large" => DataTable(ValidRows(), "Names", "page_size", 101),
            "data-table-cell-text-empty" => DataTable(Items(Items(RawText("Name")), Items(RawText(""))), "Names"),
            "data-table-content-too-long" => DataTable(Items(Items(RawText("Name")), Items(RawText(Repeated("x", 20000)))), "Names"),
            "data-visualization-title-too-long" => Typed("DataVisualizationBlock", "data_visualization", "title", Repeated("x", 51), "chart", Obj("pie", "segments", Items(Segment("A", 1)))),
            "pie-chart-empty" => Typed("PieChart", "pie", "segments", Items()),
            "pie-chart-too-many-segments" => Typed("PieChart", "pie", "segments", Copies(13, index => Segment("S", 1))),
            "chart-segment-label-too-long" => Typed("ChartSegment", "", "label", Repeated("x", 21), "value", 1),
            "chart-segment-value-not-positive" => Typed("ChartSegment", "", "label", "A", "value", 0),
            "chart-series-empty" => Typed("LineChart", "line", "axis_config", Axis("A"), "series", Items()),
            "chart-duplicate-point-labels" => Typed("LineChart", "line", "axis_config", Axis("A", "B"), "series", Items(Series("Series", Point("A", 1), Point("A", 2)))),
            "chart-too-many-series" => Typed("LineChart", "line", "series", Copies(13, index => Series("S" + index, Point("A", 1))), "axis_config", Axis("A")),
            "data-series-name-too-long" => Case("DataSeries", Series(Repeated("x", 21), Point("A", 1))),
            "data-series-empty" => Case("DataSeries", Map("name", "Series", "data", Items())),
            "data-series-too-many-points" => Case("DataSeries", Series("Series", Copies(21, index => Point("P" + index, index)).ToArray())),
            "data-point-label-too-long" => Case("DataPoint", Point(Repeated("x", 21), 1)),
            "axis-categories-empty" => Case("AxisConfig", Map("categories", Items())),
            "axis-too-many-categories" => Case("AxisConfig", Map("categories", Copies(21, index => "X" + index))),
            "axis-category-label-too-long" => Case("AxisConfig", Axis(Repeated("x", 21))),
            "axis-label-too-long" => Case("AxisConfig", Map("categories", Items("A"), "x_label", Repeated("x", 51))),
            _ => throw new InvalidOperationException("No C# invalid construction registered for " + id),
        };

    private static JsonNode? ToNode(object? value) => value switch
    {
        null => null,
        string text => JsonValue.Create(text),
        bool flag => JsonValue.Create(flag),
        int number => JsonValue.Create(number),
        double number => JsonValue.Create(number),
        Dictionary<string, object?> map => new JsonObject(map.Select(entry => KeyValuePair.Create(entry.Key, ToNode(entry.Value)))),
        IEnumerable<object?> items => new JsonArray(items.Select(ToNode).ToArray()),
        _ => throw new InvalidOperationException("Unsupported test value " + value.GetType()),
    };

    private static Invalid Case(string name, Dictionary<string, object?> value) => new(name, value);

    private static Invalid Typed(string name, string type, params object?[] fields) => Case(name, Obj(type, fields));

    private static Invalid Confirmation(string title, string text, string confirm, string deny) =>
        Case("Confirmation", Map("title", Plain(title), "text", Mrkdwn(text), "confirm", Plain(confirm), "deny", Plain(deny)));

    private static Invalid OptionGroup(string label, List<object?> options) =>
        Case("OptionGroup", Map("label", Plain(label), "options", options));

    private static Invalid InvalidVideo(string field, object value)
    {
        var video = ValidVideo();
        video[field] = value;
        return Case("VideoBlock", video);
    }

    private static Invalid DataTable(List<object?> rows, string caption, params object?[] fields)
    {
        var value = Obj("data_table", "rows", rows, "caption", caption);
        Put(value, fields);
        return Case("DataTableBlock", value);
    }

    private static Dictionary<string, object?> ValidVideo() =>
        Obj("video", "alt_text", "Video", "thumbnail_url", "https://example.com/thumbnail.png", "title", Plain("Title"), "video_url", "https://example.com/video.mp4");

    private static Dictionary<string, object?> Option(string text, string value) => Map("text", Plain(text), "value", value);

    private static Dictionary<string, object?> Feedback(string text, string value) => Map("text", Plain(text), "value", value);

    private static Dictionary<string, object?> Button() => Obj("button", "text", Plain("A"), "action_id", "a");

    private static Dictionary<string, object?> IconButton() => Obj("icon_button", "text", Plain("Delete"), "icon", "trash");

    private static Dictionary<string, object?> Workflow() => Map("trigger", Map("url", "https://slack.com/shortcuts/Ft0/abc"));

    private static Dictionary<string, object?> InputElement() => Obj("plain_text_input", "action_id", "a");

    private static Dictionary<string, object?> InputBlock() => Obj("input", "label", Plain("Name"), "element", InputElement());

    private static Dictionary<string, object?> Divider() => Obj("divider");

    private static Dictionary<string, object?> Card() => Obj("card", "title", Plain("Card"));

    private static Dictionary<string, object?> RawText(string value) => Obj("raw_text", "text", value);

    private static List<object?> ValidRows() => Items(Items(RawText("Name")), Items(RawText("Alice")));

    private static Dictionary<string, object?> Segment(string label, double value) => Map("label", label, "value", value);

    private static Dictionary<string, object?> Point(string label, double value) => Map("label", label, "value", value);

    private static Dictionary<string, object?> Series(string name, params object?[] points) => Map("name", name, "data", Items(points));

    private static Dictionary<string, object?> Axis(params string[] categories) => Map("categories", Items(categories));

    private static Dictionary<string, object?> Plain(string value) => TextObject("plain_text", value);

    private static Dictionary<string, object?> Mrkdwn(string value) => TextObject("mrkdwn", value);

    private static Dictionary<string, object?> TextObject(string type, string value) => Obj(type, "text", value);

    private static Dictionary<string, object?> Obj(string type, params object?[] fields)
    {
        var result = new Dictionary<string, object?>(StringComparer.Ordinal);
        if (type.Length > 0)
        {
            result["type"] = type;
        }

        Put(result, fields);
        return result;
    }

    private static Dictionary<string, object?> Map(params object?[] fields) => Obj(string.Empty, fields);

    private static void Put(Dictionary<string, object?> target, object?[] fields)
    {
        if (fields.Length % 2 != 0)
        {
            throw new ArgumentException("fields must be key/value pairs");
        }

        for (var index = 0; index < fields.Length; index += 2)
        {
            target[(string)fields[index]!] = fields[index + 1];
        }
    }

    private static List<object?> Items(params object?[] items) => [.. items];

    private static List<object?> Copies(int count, Func<int, object?> factory) => Enumerable.Range(0, count).Select(factory).ToList();

    private static string Repeated(string value, int count) => string.Concat(Enumerable.Repeat(value, count));

    private sealed record Invalid(string Name, Dictionary<string, object?> Value);
}
