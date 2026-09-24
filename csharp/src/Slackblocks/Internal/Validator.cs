using System;
using System.Collections.Frozen;
using System.Collections.Generic;
using System.Text.Json.Nodes;
using System.Text.RegularExpressions;

namespace Slackblocks.Internal;

/// <summary>
/// Central Slack wire validation, applied once to each constructed value. The rules mirror the
/// Java validator line for line so both implementations reject the shared invalid corpus with the
/// same categories.
/// </summary>
internal static partial class Validator
{
    private static readonly FrozenDictionary<string, string[]> RequiredFields = new Dictionary<string, string[]>
    {
        ["actions"] = ["elements"],
        ["alert"] = ["text"],
        ["area"] = ["series", "axis_config"],
        ["bar"] = ["series", "axis_config"],
        ["button"] = ["text", "action_id"],
        ["carousel"] = ["elements"],
        ["channel"] = ["channel_id"],
        ["channels_select"] = ["action_id"],
        ["checkboxes"] = ["action_id", "options"],
        ["container"] = ["child_blocks"],
        ["context"] = ["elements"],
        ["context_actions"] = ["elements"],
        ["conversations_select"] = ["action_id"],
        ["data_table"] = ["rows", "caption"],
        ["data_visualization"] = ["title", "chart"],
        ["datepicker"] = ["action_id"],
        ["datetimepicker"] = ["action_id"],
        ["email_text_input"] = ["action_id"],
        ["emoji"] = ["name"],
        ["external_select"] = ["action_id"],
        ["feedback_buttons"] = ["positive_button", "negative_button"],
        ["file"] = ["external_id"],
        ["file_input"] = ["action_id"],
        ["header"] = ["text"],
        ["home"] = ["blocks"],
        ["icon_button"] = ["text"],
        ["image"] = ["alt_text"],
        ["input"] = ["label", "element"],
        ["line"] = ["series", "axis_config"],
        ["link"] = ["url"],
        ["markdown"] = ["text"],
        ["modal"] = ["title", "blocks"],
        ["multi_channels_select"] = ["action_id"],
        ["multi_conversations_select"] = ["action_id"],
        ["multi_external_select"] = ["action_id"],
        ["multi_static_select"] = ["action_id"],
        ["multi_users_select"] = ["action_id"],
        ["number_input"] = ["action_id"],
        ["overflow"] = ["action_id", "options"],
        ["pie"] = ["segments"],
        ["plain_text_input"] = ["action_id"],
        ["plan"] = ["title"],
        ["radio_buttons"] = ["action_id", "options"],
        ["raw_number"] = ["value", "text"],
        ["raw_text"] = ["text"],
        ["rich_text"] = ["elements"],
        ["rich_text_input"] = ["action_id"],
        ["rich_text_list"] = ["style", "elements"],
        ["rich_text_preformatted"] = ["elements"],
        ["rich_text_quote"] = ["elements"],
        ["rich_text_section"] = ["elements"],
        ["static_select"] = ["action_id"],
        ["table"] = ["rows"],
        ["task_card"] = ["task_id", "title"],
        ["text"] = ["text"],
        ["timepicker"] = ["action_id"],
        ["url"] = ["url", "text"],
        ["url_text_input"] = ["action_id"],
        ["user"] = ["user_id"],
        ["usergroup"] = ["usergroup_id"],
        ["users_select"] = ["action_id"],
        ["video"] = ["alt_text", "thumbnail_url", "title", "video_url"],
        ["workflow_button"] = ["text", "workflow"],
    }.ToFrozenDictionary(StringComparer.Ordinal);

    private static readonly FrozenSet<string> InputElementTypes = Set(
        "plain_text_input",
        "number_input",
        "checkboxes",
        "radio_buttons",
        "datepicker",
        "datetimepicker",
        "timepicker",
        "channels_select",
        "multi_channels_select",
        "conversations_select",
        "multi_conversations_select",
        "external_select",
        "multi_external_select",
        "static_select",
        "multi_static_select",
        "users_select",
        "multi_users_select",
        "rich_text_input",
        "email_text_input",
        "url_text_input",
        "file_input");

    private static readonly FrozenSet<string> ConfirmTypes = Set(
        "button",
        "channels_select",
        "checkboxes",
        "conversations_select",
        "datepicker",
        "datetimepicker",
        "external_select",
        "icon_button",
        "multi_channels_select",
        "multi_conversations_select",
        "multi_external_select",
        "multi_static_select",
        "multi_users_select",
        "overflow",
        "radio_buttons",
        "static_select",
        "timepicker",
        "users_select",
        "workflow_button");

    private static readonly FrozenSet<string> ContextElementTypes = Set("plain_text", "mrkdwn", "image");
    private static readonly FrozenSet<string> AlertLevels = Set("default", "info", "warning", "error", "success");
    private static readonly FrozenSet<string> ContainerWidths = Set("narrow", "standard", "wide", "full");
    private static readonly FrozenSet<string> TaskStatuses = Set("pending", "in_progress", "complete", "error");
    private static readonly FrozenSet<string> TableCellTypes = Set("raw_text", "rich_text");
    private static readonly FrozenSet<string> DataTableCellTypes = Set("raw_text", "rich_text", "raw_number");
    private static readonly FrozenSet<string> AttachmentColorAliases = Set("good", "warning", "danger");

    /// <summary>Validates one fully assembled Slack object.</summary>
    /// <param name="name">Public type name used as the root of error paths.</param>
    /// <param name="value">The wire object.</param>
    public static void Validate(string name, JsonObject value)
    {
        if (value.Count == 0)
        {
            Fail(ErrorCategory.MissingRequired, name, "expected at least one field");
        }

        ValidateType(name, value);
        ValidateObject(value, name);
    }

    [GeneratedRegex("^#[0-9a-fA-F]{6}$", RegexOptions.CultureInvariant)]
    private static partial Regex AttachmentColor();

    private static FrozenSet<string> Set(params string[] values) => values.ToFrozenSet(StringComparer.Ordinal);

    private static void ValidateType(string name, JsonObject value)
    {
        switch (name)
        {
            case "Confirmation":
                ValidateConfirmation(value, name);
                break;
            case "Option":
                ValidateOption(value, name);
                break;
            case "OptionGroup":
                ValidateOptionGroup(value, name);
                break;
            case "ConversationFilter":
                if (!value.ContainsKey("include")
                    && !value.ContainsKey("exclude_external_shared_channels")
                    && !value.ContainsKey("exclude_bot_users"))
                {
                    Fail(ErrorCategory.MissingRequired, name, "expected at least one filter field");
                }

                break;
            case "InputParameter":
                Require(value, "name", name);
                Require(value, "value", name);
                break;
            case "Trigger":
                Require(value, "url", name);
                break;
            case "Workflow":
                Require(value, "trigger", name);
                break;
            case "SlackFile":
                if (value.ContainsKey("id") == value.ContainsKey("url"))
                {
                    Fail(ErrorCategory.MutuallyExclusive, name, "expected exactly one of id or url");
                }

                break;
            case "ChartSegment":
                ValidateLabelValue(value, name, 20, positive: true);
                break;
            case "DataPoint":
                ValidateLabelValue(value, name, 20, positive: false);
                break;
            case "DataSeries":
                ValidateDataSeries(value, name);
                break;
            case "AxisConfig":
                ValidateAxisConfig(value, name);
                break;
            case "FeedbackButton":
                ValidateFeedbackButton(value, name);
                break;
            case "DispatchActionConfiguration":
                ValidateDispatchActionConfiguration(value, name);
                break;
            case "Attachment":
                if (JsonValues.IsString(value["color"], out var color)
                    && !AttachmentColorAliases.Contains(color)
                    && !AttachmentColor().IsMatch(color))
                {
                    Fail(ErrorCategory.TypeMismatch, Child(name, "color"), "expected a six-digit hex color or Slack alias");
                }

                break;
            case "MessagePayload":
                Require(value, "channel", name);
                if (!JsonValues.IsString(value["channel"], out var channel))
                {
                    Fail(ErrorCategory.TypeMismatch, Child(name, "channel"), "expected a string");
                }

                StringLength(channel, Child(name, "channel"), 1, 0);
                ValidateMessageCollections(value, name);
                break;
            case "MessageResponse":
            case "WebhookMessage":
                ValidateMessageCollections(value, name);
                break;
        }
    }

    private static void ValidateObject(JsonObject value, string path)
    {
        var type = ObjectType(value);
        if (RequiredFields.TryGetValue(type, out var required))
        {
            foreach (var field in required)
            {
                Require(value, field, path);
            }
        }

        if (JsonValues.IsString(value["block_id"], out var blockId))
        {
            StringLength(blockId, Child(path, "block_id"), 0, 255);
        }

        if (JsonValues.IsString(value["action_id"], out var actionId))
        {
            StringLength(actionId, Child(path, "action_id"), 0, 255);
        }

        if (ConfirmTypes.Contains(type) && value.ContainsKey("confirm"))
        {
            ValidateConfirmation(ObjectAt(value["confirm"], Child(path, "confirm")), Child(path, "confirm"));
        }

        if (value.ContainsKey("dispatch_action_config"))
        {
            ValidateDispatchActionConfiguration(
                ObjectAt(value["dispatch_action_config"], Child(path, "dispatch_action_config")),
                Child(path, "dispatch_action_config"));
        }

        switch (type)
        {
            case "plain_text":
            case "mrkdwn":
                TextLength(value, Child(path, "text"), 1, 3000);
                break;
            case "icon":
                if (!JsonValues.IsString(value["name"], out var iconName) || !SlackVocabulary.SlackIconNames.Contains(iconName))
                {
                    Fail(ErrorCategory.TypeMismatch, Child(path, "name"), "unknown Slack icon");
                }

                break;
            case "section":
                ValidateSection(value, path);
                break;
            case "header":
                TextLength(value["text"], Child(path, "text.text"), 0, 150);
                break;
            case "button":
                ValidateButton(value, path);
                break;
            case "workflow_button":
                ValidateWorkflowButton(value, path);
                break;
            case "icon_button":
                ValidateIconButton(value, path);
                break;
            case "feedback_buttons":
                foreach (var field in new[] { "positive_button", "negative_button" })
                {
                    ValidateFeedbackButton(ObjectAt(value[field], Child(path, field)), Child(path, field));
                }

                break;
            case "file_input":
                if (JsonValues.Number(value["max_files"]) is double maxFiles && (maxFiles < 1 || maxFiles > 10))
                {
                    Fail(ErrorCategory.OutOfRange, Child(path, "max_files"), "expected a value between 1 and 10");
                }

                break;
            case "plain_text_input":
                if (JsonValues.Number(value["max_length"]) is double maxLength && maxLength > 3000)
                {
                    Fail(ErrorCategory.OutOfRange, Child(path, "max_length"), "exceeds maximum 3000");
                }

                if (value.ContainsKey("placeholder"))
                {
                    TextLength(value["placeholder"], Child(path, "placeholder.text"), 0, SlackLimits.PlainTextInputPlaceholderMaxLength);
                }

                break;
            case "email_text_input":
                CheckOptionalText(value, "placeholder", path, SlackLimits.EmailInputPlaceholderMaxLength);
                break;
            case "url_text_input":
                CheckOptionalText(value, "placeholder", path, SlackLimits.UrlInputPlaceholderMaxLength);
                break;
            case "datepicker":
                CheckOptionalText(value, "placeholder", path, SlackLimits.DatePickerPlaceholderMaxLength);
                break;
            case "timepicker":
                CheckOptionalText(value, "placeholder", path, SlackLimits.TimePickerPlaceholderMaxLength);
                break;
            case "rich_text_input":
                CheckOptionalText(value, "placeholder", path, SlackLimits.RichTextInputPlaceholderMaxLength);
                break;
            case "overflow":
            case "checkboxes":
            case "radio_buttons":
                var options = ListAt(value["options"], Child(path, "options"));
                SliceLength(options, Child(path, "options"), 1, type == "overflow" ? 5 : 10);
                ValidateOptions(options, Child(path, "options"));
                break;
            case "url":
                StringLength(StringAt(value["url"], Child(path, "url")), Child(path, "url"), 1, 3000);
                break;
            case "static_select":
            case "multi_static_select":
                ValidateStaticSelect(value, path);
                break;
            case "number_input":
                if (JsonValues.Number(value["min_value"]) is double minimum
                    && JsonValues.Number(value["max_value"]) is double maximum
                    && minimum > maximum)
                {
                    Fail(ErrorCategory.OutOfRange, path, "min_value cannot exceed max_value");
                }

                CheckOptionalText(value, "placeholder", path, SlackLimits.NumberInputPlaceholderMaxLength);
                break;
            case "image":
                ValidateImage(value, path);
                break;
            case "context":
                ValidateContext(value, path);
                break;
            case "actions":
                SliceLength(ListAt(value["elements"], Child(path, "elements")), Child(path, "elements"), 0, 25);
                break;
            case "alert":
                TextLength(value["text"], Child(path, "text.text"), 0, 200);
                if (JsonValues.IsString(value["level"], out var level) && !AlertLevels.Contains(level))
                {
                    Fail(ErrorCategory.TypeMismatch, Child(path, "level"), "unknown alert level");
                }

                break;
            case "card":
                ValidateCard(value, path);
                break;
            case "carousel":
                ValidateCarousel(value, path);
                break;
            case "container":
                ValidateContainer(value, path);
                break;
            case "context_actions":
                SliceLength(ListAt(value["elements"], Child(path, "elements")), Child(path, "elements"), 1, 5);
                break;
            case "data_table":
                ValidateTable(value, path, dataTable: true);
                break;
            case "table":
                ValidateTable(value, path, dataTable: false);
                break;
            case "data_visualization":
                if (JsonValues.IsString(value["title"], out var title))
                {
                    StringLength(title, Child(path, "title"), 0, 50);
                }

                ObjectAt(value["chart"], Child(path, "chart"));
                break;
            case "pie":
                ValidatePie(value, path);
                break;
            case "bar":
            case "area":
            case "line":
                ValidateSeriesChart(value, path);
                break;
            case "task_card":
                if (JsonValues.IsString(value["status"], out var status) && !TaskStatuses.Contains(status))
                {
                    Fail(ErrorCategory.TypeMismatch, Child(path, "status"), "unknown task status");
                }

                break;
            case "input":
                ValidateInput(value, path);
                break;
            case "markdown":
                StringLength(StringAt(value["text"], Child(path, "text")), Child(path, "text"), 1, 12000);
                break;
            case "video":
                ValidateVideo(value, path);
                break;
            case "modal":
            case "home":
                ValidateView(value, path, type);
                break;
        }

        foreach (var (key, nested) in value)
        {
            if (key != "type" && key != "event_payload")
            {
                ValidateNested(nested, Child(path, key));
            }
        }
    }

    private static void ValidateSection(JsonObject value, string path)
    {
        var hasText = value.ContainsKey("text");
        var fields = value.ContainsKey("fields") ? ListAt(value["fields"], Child(path, "fields")) : new JsonArray();
        if (!hasText && fields.Count == 0)
        {
            Fail(ErrorCategory.MissingRequired, path, "expected text, fields, or both");
        }

        if (hasText)
        {
            TextLength(value["text"], Child(path, "text.text"), 0, 3000);
        }

        if (value.ContainsKey("fields"))
        {
            SliceLength(fields, Child(path, "fields"), 0, 10);
            for (var index = 0; index < fields.Count; index++)
            {
                TextLength(fields[index], $"{Child(path, "fields")}[{index}].text", 0, 2000);
            }
        }
    }

    private static void ValidateButton(JsonObject value, string path)
    {
        TextLength(value["text"], Child(path, "text.text"), 0, 75);
        CheckOptionalString(value, "url", path, 3000);
        CheckOptionalString(value, "value", path, 2000);
        CheckOptionalString(value, "accessibility_label", path, 75);
    }

    private static void ValidateWorkflowButton(JsonObject value, string path)
    {
        TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.WorkflowButtonTextMaxLength);
        CheckOptionalString(value, "accessibility_label", path, SlackLimits.WorkflowButtonAccessibilityLabelMaxLength);
    }

    private static void ValidateIconButton(JsonObject value, string path)
    {
        if (!JsonValues.IsString(value["icon"], out var icon) || icon != "trash")
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "icon"), "expected trash");
        }

        CheckOptionalString(value, "value", path, SlackLimits.IconButtonValueMaxLength);
        CheckOptionalString(value, "accessibility_label", path, SlackLimits.IconButtonAccessibilityLabelMaxLength);
        if (value["visible_to_user_ids"] is JsonArray users)
        {
            SliceLength(users, Child(path, "visible_to_user_ids"), 0, 10);
        }
    }

    private static void ValidateStaticSelect(JsonObject value, string path)
    {
        var hasOptions = value.ContainsKey("options");
        var hasGroups = value.ContainsKey("option_groups");
        if (hasOptions && hasGroups)
        {
            Fail(ErrorCategory.MutuallyExclusive, path, "options and option_groups cannot be provided together");
        }

        if (hasOptions)
        {
            var options = ListAt(value["options"], Child(path, "options"));
            SliceLength(options, Child(path, "options"), 0, 100);
            ValidateOptions(options, Child(path, "options"));
        }

        if (hasGroups)
        {
            var groups = ListAt(value["option_groups"], Child(path, "option_groups"));
            SliceLength(groups, Child(path, "option_groups"), 0, 100);
            for (var index = 0; index < groups.Count; index++)
            {
                var groupPath = $"{Child(path, "option_groups")}[{index}]";
                ValidateOptionGroup(ObjectAt(groups[index], groupPath), groupPath);
            }
        }

        if (value.ContainsKey("placeholder"))
        {
            TextLength(value["placeholder"], Child(path, "placeholder.text"), 0, 150);
        }
    }

    private static void ValidateImage(JsonObject value, string path)
    {
        var hasUrl = value.ContainsKey("image_url");
        var hasSlackFile = value.ContainsKey("slack_file");
        if (!hasUrl && !hasSlackFile)
        {
            Fail(ErrorCategory.MissingRequired, path, "expected image_url or slack_file");
        }

        if (hasUrl && hasSlackFile)
        {
            Fail(ErrorCategory.MutuallyExclusive, path, "image_url and slack_file cannot be provided together");
        }

        CheckOptionalString(value, "image_url", path, 3000);
        CheckOptionalString(value, "alt_text", path, 2000);
    }

    private static void ValidateContext(JsonObject value, string path)
    {
        var elements = ListAt(value["elements"], Child(path, "elements"));
        SliceLength(elements, Child(path, "elements"), 0, 10);
        for (var index = 0; index < elements.Count; index++)
        {
            var elementPath = $"{Child(path, "elements")}[{index}]";
            if (!ContextElementTypes.Contains(ObjectType(ObjectAt(elements[index], elementPath))))
            {
                Fail(ErrorCategory.TypeMismatch, elementPath, "expected text or image element");
            }
        }
    }

    private static void ValidateCard(JsonObject value, string path)
    {
        if (!value.ContainsKey("hero_image")
            && !value.ContainsKey("title")
            && !value.ContainsKey("actions")
            && !value.ContainsKey("body"))
        {
            Fail(ErrorCategory.MissingRequired, path, "expected hero_image, title, actions, or body");
        }

        if (value.ContainsKey("icon") && value.ContainsKey("slack_icon"))
        {
            Fail(ErrorCategory.MutuallyExclusive, path, "icon and slack_icon cannot be provided together");
        }

        CheckOptionalText(value, "title", path, 150);
        CheckOptionalText(value, "subtitle", path, 150);
        CheckOptionalText(value, "body", path, 200);
        CheckOptionalText(value, "subtext", path, 200);
        if (value["actions"] is JsonArray actions)
        {
            SliceLength(actions, Child(path, "actions"), 0, 3);
        }
    }

    private static void ValidateCarousel(JsonObject value, string path)
    {
        var cards = ListAt(value["elements"], Child(path, "elements"));
        SliceLength(cards, Child(path, "elements"), 1, 10);
        for (var index = 0; index < cards.Count; index++)
        {
            var cardPath = $"{Child(path, "elements")}[{index}]";
            if (ObjectType(ObjectAt(cards[index], cardPath)) != "card")
            {
                Fail(ErrorCategory.TypeMismatch, cardPath, "expected a card");
            }
        }
    }

    private static void ValidateContainer(JsonObject value, string path)
    {
        if (!value.ContainsKey("title") && !value.ContainsKey("rich_text_title"))
        {
            Fail(ErrorCategory.MissingRequired, path, "expected title or rich_text_title");
        }

        CheckOptionalText(value, "title", path, 150);
        CheckOptionalText(value, "subtitle", path, 150);
        var blocks = ListAt(value["child_blocks"], Child(path, "child_blocks"));
        SliceLength(blocks, Child(path, "child_blocks"), 1, 10);
        if (JsonValues.IsString(value["width"], out var width) && !ContainerWidths.Contains(width))
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "width"), "unknown container width");
        }

        if (JsonValues.IsTrue(value["default_collapsed"]) && !JsonValues.IsTrue(value["is_collapsible"]))
        {
            Fail(ErrorCategory.InvalidUsage, Child(path, "default_collapsed"), "requires is_collapsible");
        }

        if (JsonValues.IsTrue(value["has_header_divider"]) && JsonValues.IsTrue(value["is_collapsible"]))
        {
            Fail(ErrorCategory.InvalidUsage, Child(path, "has_header_divider"), "requires a non-collapsible container");
        }
    }

    private static void ValidatePie(JsonObject value, string path)
    {
        var segments = ListAt(value["segments"], Child(path, "segments"));
        SliceLength(segments, Child(path, "segments"), 1, 12);
        for (var index = 0; index < segments.Count; index++)
        {
            var segmentPath = $"{Child(path, "segments")}[{index}]";
            ValidateLabelValue(ObjectAt(segments[index], segmentPath), segmentPath, 20, positive: true);
        }
    }

    private static void ValidateInput(JsonObject value, string path)
    {
        TextLength(value["label"], Child(path, "label.text"), 0, 2000);
        if (value.ContainsKey("hint"))
        {
            TextLength(value["hint"], Child(path, "hint.text"), 0, 2000);
        }

        var element = ObjectAt(value["element"], Child(path, "element"));
        if (!InputElementTypes.Contains(ObjectType(element)))
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "element"), "expected an input-compatible element");
        }
    }

    private static void ValidateVideo(JsonObject value, string path)
    {
        if (JsonValues.IsString(value["alt_text"], out var altText))
        {
            StringLength(altText, Child(path, "alt_text"), 1, 200);
        }

        TextLength(value["title"], Child(path, "title.text"), 0, 200);
        CheckOptionalString(value, "author_name", path, 50);
        CheckOptionalString(value, "provider_name", path, 50);
        CheckOptionalText(value, "description", path, 200);
    }

    private static void ValidateView(JsonObject value, string path, string type)
    {
        var blocks = ListAt(value["blocks"], Child(path, "blocks"));
        SliceLength(blocks, Child(path, "blocks"), 1, 100);
        ValidateSurface(blocks, type, Child(path, "blocks"));
        CheckOptionalString(value, "private_metadata", path, 3000);
        CheckOptionalString(value, "callback_id", path, 255);
        if (type == "modal")
        {
            if (!value.ContainsKey("submit"))
            {
                for (var index = 0; index < blocks.Count; index++)
                {
                    var blockPath = $"{Child(path, "blocks")}[{index}]";
                    if (ObjectType(ObjectAt(blocks[index], blockPath)) == "input")
                    {
                        Fail(
                            ErrorCategory.MissingRequired,
                            Child(path, "submit"),
                            $"required when the modal contains an input block ({blockPath})");
                    }
                }
            }

            CheckOptionalText(value, "title", path, 24);
            CheckOptionalText(value, "close", path, 24);
            CheckOptionalText(value, "submit", path, 24);
        }
    }

    private static void ValidateOptionGroup(JsonObject value, string path)
    {
        Require(value, "label", path);
        TextLength(value["label"], Child(path, "label.text"), 0, 75);
        var options = ListAt(value["options"], Child(path, "options"));
        SliceLength(options, Child(path, "options"), 1, 100);
        ValidateOptions(options, Child(path, "options"));
    }

    private static void ValidateFeedbackButton(JsonObject value, string path)
    {
        Require(value, "text", path);
        Require(value, "value", path);
        TextLength(value["text"], Child(path, "text.text"), 0, 75);
        CheckOptionalString(value, "value", path, 2000);
        CheckOptionalString(value, "accessibility_label", path, 75);
    }

    private static void ValidateDispatchActionConfiguration(JsonObject value, string path)
    {
        Require(value, "trigger_actions_on", path);
        SliceLength(
            ListAt(value["trigger_actions_on"], Child(path, "trigger_actions_on")),
            Child(path, "trigger_actions_on"),
            SlackLimits.DispatchActionConfigurationTriggerActionsOnMinItems,
            SlackLimits.DispatchActionConfigurationTriggerActionsOnMaxItems);
    }

    private static void ValidateDataSeries(JsonObject value, string path)
    {
        var name = StringAt(value["name"], Child(path, "name"));
        StringLength(name, Child(path, "name"), 0, 20);
        var data = ListAt(value["data"], Child(path, "data"));
        SliceLength(data, Child(path, "data"), 1, 20);
    }

    private static void ValidateAxisConfig(JsonObject value, string path)
    {
        var categories = ListAt(value["categories"], Child(path, "categories"));
        SliceLength(categories, Child(path, "categories"), 1, 20);
        var seen = new HashSet<string>(StringComparer.Ordinal);
        for (var index = 0; index < categories.Count; index++)
        {
            var itemPath = $"{Child(path, "categories")}[{index}]";
            var category = StringAt(categories[index], itemPath);
            StringLength(category, itemPath, 0, 20);
            if (!seen.Add(category))
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "categories"), "expected unique labels");
            }
        }

        CheckOptionalString(value, "x_label", path, 50);
        CheckOptionalString(value, "y_label", path, 50);
    }

    private static void ValidateConfirmation(JsonObject value, string path)
    {
        foreach (var (field, maximum) in new[] { ("title", 100), ("text", 300), ("confirm", 30), ("deny", 30) })
        {
            Require(value, field, path);
            TextLength(value[field], Child(path, field + ".text"), 0, maximum);
        }
    }

    private static void ValidateOption(JsonObject value, string path)
    {
        Require(value, "text", path);
        Require(value, "value", path);
        TextLength(value["text"], Child(path, "text.text"), 0, 75);
        CheckOptionalString(value, "value", path, 150);
        CheckOptionalText(value, "description", path, 75);
        CheckOptionalString(value, "url", path, 3000);
    }

    private static void ValidateOptions(JsonArray values, string path)
    {
        for (var index = 0; index < values.Count; index++)
        {
            var optionPath = $"{path}[{index}]";
            ValidateOption(ObjectAt(values[index], optionPath), optionPath);
        }
    }

    private static void ValidateTable(JsonObject value, string path, bool dataTable)
    {
        var rows = ListAt(value["rows"], Child(path, "rows"));
        SliceLength(rows, Child(path, "rows"), dataTable ? 2 : 1, dataTable ? 201 : SlackLimits.TableRowsMaxItems);
        var columns = -1;
        var contentLength = 0;
        var allowed = dataTable ? DataTableCellTypes : TableCellTypes;
        for (var rowIndex = 0; rowIndex < rows.Count; rowIndex++)
        {
            var rowPath = $"{Child(path, "rows")}[{rowIndex}]";
            var row = ListAt(rows[rowIndex], rowPath);
            SliceLength(row, rowPath, dataTable ? 1 : 0, dataTable ? 20 : SlackLimits.TableColumnsMaxItems);
            if (columns < 0)
            {
                columns = row.Count;
            }
            else if (row.Count != columns)
            {
                Fail(ErrorCategory.InvalidUsage, rowPath, "column count differs");
            }

            for (var cellIndex = 0; cellIndex < row.Count; cellIndex++)
            {
                var cellPath = $"{rowPath}[{cellIndex}]";
                var cell = ObjectAt(row[cellIndex], cellPath);
                var cellType = ObjectType(cell);
                if (!allowed.Contains(cellType))
                {
                    Fail(ErrorCategory.TypeMismatch, cellPath, "unsupported table cell");
                }

                if (dataTable && rowIndex == 0 && cellType == "rich_text")
                {
                    Fail(ErrorCategory.TypeMismatch, cellPath, "header cells cannot contain rich text");
                }

                contentLength += TextCharacterCount(cell);
                if (dataTable && cellType is "raw_text" or "raw_number")
                {
                    StringLength(StringAt(cell["text"], Child(cellPath, "text")), Child(cellPath, "text"), 1, 0);
                }
            }
        }

        if (value.ContainsKey("column_settings"))
        {
            var settings = ListAt(value["column_settings"], Child(path, "column_settings"));
            SliceLength(settings, Child(path, "column_settings"), 0, 20);
            if (settings.Count != columns)
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "column_settings"), "expected one entry for every column");
            }
        }

        if (dataTable)
        {
            if (JsonValues.Number(value["page_size"]) is double pageSize && (pageSize < 1 || pageSize > 100))
            {
                Fail(ErrorCategory.OutOfRange, Child(path, "page_size"), "expected a value between 1 and 100");
            }

            var caption = StringAt(value["caption"], Child(path, "caption"));
            StringLength(caption, Child(path, "caption"), 1, 0);
            if (contentLength > 20000)
            {
                Fail(ErrorCategory.LengthExceeded, Child(path, "rows"), "content exceeds maximum 20000");
            }
        }
    }

    private static void ValidateSeriesChart(JsonObject value, string path)
    {
        var series = ListAt(value["series"], Child(path, "series"));
        SliceLength(series, Child(path, "series"), 1, 12);
        var axis = ObjectAt(value["axis_config"], Child(path, "axis_config"));
        ValidateAxisConfig(axis, Child(path, "axis_config"));
        var categories = ListAt(axis["categories"], Child(path, "axis_config.categories"));
        var categorySet = new HashSet<string>(StringComparer.Ordinal);
        foreach (var raw in categories)
        {
            categorySet.Add(StringAt(raw, Child(path, "axis_config.categories")));
        }

        var names = new HashSet<string>(StringComparer.Ordinal);
        for (var index = 0; index < series.Count; index++)
        {
            var itemPath = $"{Child(path, "series")}[{index}]";
            var item = ObjectAt(series[index], itemPath);
            ValidateDataSeries(item, itemPath);
            var name = StringAt(item["name"], Child(itemPath, "name"));
            if (!names.Add(name))
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "series"), "series names must be unique");
            }

            var points = ListAt(item["data"], Child(itemPath, "data"));
            var seen = new HashSet<string>(StringComparer.Ordinal);
            for (var pointIndex = 0; pointIndex < points.Count; pointIndex++)
            {
                var pointPath = $"{Child(itemPath, "data")}[{pointIndex}]";
                var point = ObjectAt(points[pointIndex], pointPath);
                ValidateLabelValue(point, pointPath, 20, positive: false);
                seen.Add(StringAt(point["label"], Child(pointPath, "label")));
            }

            if (points.Count != categorySet.Count || !seen.SetEquals(categorySet))
            {
                Fail(ErrorCategory.InvalidUsage, Child(itemPath, "data"), "expected exactly one point for every axis category");
            }
        }
    }

    private static void ValidateLabelValue(JsonObject value, string path, int labelMaximum, bool positive)
    {
        Require(value, "label", path);
        var label = StringAt(value["label"], Child(path, "label"));
        StringLength(label, Child(path, "label"), 0, labelMaximum);
        if (JsonValues.Number(value["value"]) is not double number || !double.IsFinite(number))
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "value"), "expected a finite number");
            return;
        }

        if (positive && number <= 0)
        {
            Fail(ErrorCategory.OutOfRange, Child(path, "value"), "expected a value greater than 0");
        }
    }

    private static void ValidateMessageCollections(JsonObject value, string path)
    {
        if (value.ContainsKey("blocks"))
        {
            var blocks = ListAt(value["blocks"], Child(path, "blocks"));
            SliceLength(blocks, Child(path, "blocks"), 0, 50);
            ValidateSurface(blocks, "message", Child(path, "blocks"));
        }

        if (value.ContainsKey("attachments"))
        {
            SliceLength(ListAt(value["attachments"], Child(path, "attachments")), Child(path, "attachments"), 0, 100);
        }
    }

    private static void ValidateSurface(JsonArray blocks, string surface, string path)
    {
        var allowed = SlackVocabulary.SurfaceBlockTypes[surface];
        for (var index = 0; index < blocks.Count; index++)
        {
            var blockPath = $"{path}[{index}]";
            var type = ObjectType(ObjectAt(blocks[index], blockPath));
            if (!allowed.Contains(type))
            {
                Fail(ErrorCategory.TypeMismatch, blockPath + ".type", $"block type {type} is not supported on {surface} surfaces");
            }
        }
    }

    private static void ValidateNested(JsonNode? value, string path)
    {
        switch (value)
        {
            case null:
                Fail(ErrorCategory.TypeMismatch, path, "expected a value, not null");
                break;
            case JsonObject obj:
                ValidateObject(obj, path);
                break;
            case JsonArray array:
                for (var index = 0; index < array.Count; index++)
                {
                    ValidateNested(array[index], $"{path}[{index}]");
                }

                break;
        }
    }

    private static JsonObject ObjectAt(JsonNode? value, string path)
    {
        if (value is not JsonObject obj)
        {
            Fail(ErrorCategory.TypeMismatch, path, "expected an object");
            throw new InvalidOperationException("unreachable");
        }

        return obj;
    }

    private static JsonArray ListAt(JsonNode? value, string path)
    {
        if (value is not JsonArray array)
        {
            Fail(ErrorCategory.TypeMismatch, path, "expected an array");
            throw new InvalidOperationException("unreachable");
        }

        return array;
    }

    private static string StringAt(JsonNode? value, string path)
    {
        if (!JsonValues.IsString(value, out var text))
        {
            Fail(ErrorCategory.TypeMismatch, path, "expected a string");
        }

        return text;
    }

    private static string ObjectType(JsonObject value) =>
        JsonValues.IsString(value["type"], out var type) ? type : string.Empty;

    private static void TextLength(JsonNode? value, string path, int minimum, int maximum)
    {
        string text;
        if (JsonValues.IsString(value, out var direct))
        {
            text = direct;
        }
        else if (value is JsonObject obj)
        {
            text = StringAt(obj["text"], path);
        }
        else
        {
            Fail(ErrorCategory.TypeMismatch, path, "expected text");
            return;
        }

        StringLength(text, path, minimum, maximum);
    }

    private static void StringLength(string value, string path, int minimum, int maximum)
    {
        var size = JsonValues.CodePoints(value);
        if (minimum > 0 && size < minimum)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"{size} is less than minimum {minimum}");
        }

        if (maximum > 0 && size > maximum)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"{size} exceeds maximum {maximum}");
        }
    }

    private static void SliceLength(JsonArray value, string path, int minimum, int maximum)
    {
        if (minimum > 0 && value.Count < minimum)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"{value.Count} is less than minimum {minimum}");
        }

        if (maximum > 0 && value.Count > maximum)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"{value.Count} exceeds maximum {maximum}");
        }
    }

    private static int TextCharacterCount(JsonNode? value)
    {
        var total = 0;
        switch (value)
        {
            case JsonObject obj:
                foreach (var (key, nested) in obj)
                {
                    total += key == "text" && JsonValues.IsString(nested, out var text)
                        ? JsonValues.CodePoints(text)
                        : TextCharacterCount(nested);
                }

                break;
            case JsonArray array:
                foreach (var item in array)
                {
                    total += TextCharacterCount(item);
                }

                break;
        }

        return total;
    }

    private static void CheckOptionalString(JsonObject value, string field, string path, int maximum)
    {
        if (JsonValues.IsString(value[field], out var text))
        {
            StringLength(text, Child(path, field), 0, maximum);
        }
    }

    private static void CheckOptionalText(JsonObject value, string field, string path, int maximum)
    {
        if (value.ContainsKey(field))
        {
            TextLength(value[field], Child(path, field + ".text"), 0, maximum);
        }
    }

    private static void Require(JsonObject value, string field, string path)
    {
        if (!value.ContainsKey(field))
        {
            Fail(ErrorCategory.MissingRequired, path, "expected " + field);
        }
    }

    private static string Child(string path, string field) => path.Length == 0 ? field : path + "." + field;

    private static void Fail(ErrorCategory category, string path, string reason) =>
        throw new ValidationException(category, path, reason);
}
