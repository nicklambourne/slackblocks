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
        ["button"] = ["text"],
        ["carousel"] = ["elements"],
        ["channel"] = ["channel_id"],
        ["checkboxes"] = ["options"],
        ["container"] = ["child_blocks"],
        ["context"] = ["elements"],
        ["context_actions"] = ["elements"],
        ["data_table"] = ["rows", "caption"],
        ["data_visualization"] = ["title", "chart"],
        ["emoji"] = ["name"],
        ["feedback_buttons"] = ["positive_button", "negative_button"],
        ["file"] = ["external_id", "source"],
        ["header"] = ["text"],
        ["home"] = ["blocks"],
        ["icon"] = ["name"],
        ["icon_button"] = ["text", "icon"],
        ["image"] = ["alt_text"],
        ["input"] = ["label", "element"],
        ["line"] = ["series", "axis_config"],
        ["link"] = ["url"],
        ["markdown"] = ["text"],
        ["modal"] = ["title", "blocks"],
        ["number_input"] = ["is_decimal_allowed"],
        ["overflow"] = ["options"],
        ["pie"] = ["segments"],
        ["plan"] = ["title", "tasks"],
        ["radio_buttons"] = ["options"],
        ["raw_number"] = ["value", "text"],
        ["raw_text"] = ["text"],
        ["rich_text"] = ["elements"],
        ["rich_text_input"] = ["action_id"],
        ["rich_text_list"] = ["style", "elements"],
        ["rich_text_preformatted"] = ["elements"],
        ["rich_text_quote"] = ["elements"],
        ["rich_text_section"] = ["elements"],
        ["table"] = ["rows"],
        ["task_card"] = ["task_id", "title", "status"],
        ["text"] = ["text"],
        ["url"] = ["url", "text"],
        ["user"] = ["user_id"],
        ["usergroup"] = ["usergroup_id"],
        ["video"] = ["alt_text", "thumbnail_url", "title", "video_url"],
        ["workflow_button"] = ["text", "workflow", "action_id"],
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

    private static readonly FrozenSet<string> MultiSelectTypes = Set(
        "multi_channels_select",
        "multi_conversations_select",
        "multi_external_select",
        "multi_static_select",
        "multi_users_select");

    private static readonly FrozenSet<string> ConversationFilterIncludes = Set("im", "mpim", "private", "public");
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

    [GeneratedRegex("^F[A-Z0-9]{8,}$", RegexOptions.CultureInvariant)]
    private static partial Regex SlackFileId();

    /// <summary>Matches a path ending in a block list item, where an <c>image</c> is an image block.</summary>
    [GeneratedRegex(@"(^|\.)(child_)?blocks\[\d+]$", RegexOptions.CultureInvariant)]
    private static partial Regex BlockListItem();

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
                ValidateConversationFilter(value, name);
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
                ValidateSlackFile(value, name);
                break;
            case "ChartSegment":
                ValidateLabelValue(value, name, SlackLimits.DataVisualizationSegmentLabelMaxLength, positive: true);
                break;
            case "DataPoint":
                ValidateLabelValue(value, name, SlackLimits.DataVisualizationPointLabelMaxLength, positive: false);
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
                Require(value, "blocks", name);
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

                StringLength(channel, Child(name, "channel"), SlackLimits.MessageChannelMinLength, 0);
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
            StringLength(blockId, Child(path, "block_id"), 0, SlackLimits.BlockIdMaxLength);
        }

        if (JsonValues.IsString(value["action_id"], out var actionId))
        {
            StringLength(actionId, Child(path, "action_id"), 0, SlackLimits.ActionIdMaxLength);
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

        if (MultiSelectTypes.Contains(type))
        {
            CheckRange(value, "max_selected_items", path, SlackLimits.MultiSelectMaxSelectedItemsMin, int.MaxValue);
        }

        if ((type is "conversations_select" or "multi_conversations_select") && value.ContainsKey("filter"))
        {
            ValidateConversationFilter(ObjectAt(value["filter"], Child(path, "filter")), Child(path, "filter"));
        }

        switch (type)
        {
            case "plain_text":
            case "mrkdwn":
                TextLength(value, Child(path, "text"), SlackLimits.TextMinLength, SlackLimits.TextMaxLength);
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
                TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.HeaderTextMaxLength);
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
                if (JsonValues.Number(value["max_files"]) is double maxFiles
                    && (maxFiles < SlackLimits.FileInputMaxFilesMin || maxFiles > SlackLimits.FileInputMaxFilesMax))
                {
                    Fail(
                        ErrorCategory.OutOfRange,
                        Child(path, "max_files"),
                        $"expected a value between {SlackLimits.FileInputMaxFilesMin} and {SlackLimits.FileInputMaxFilesMax}");
                }

                break;
            case "plain_text_input":
                CheckRange(value, "min_length", path, SlackLimits.PlainTextInputMinLengthMin, SlackLimits.PlainTextInputMinLengthMax);
                CheckRange(value, "max_length", path, SlackLimits.PlainTextInputMaxLengthMin, SlackLimits.PlainTextInputMaxLengthMax);
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
                CheckRange(value, "min_lines", path, SlackLimits.RichTextInputMinLinesMin, SlackLimits.RichTextInputMinLinesMax);
                CheckRange(value, "max_lines", path, SlackLimits.RichTextInputMaxLinesMin, SlackLimits.RichTextInputMaxLinesMax);
                if (value.ContainsKey("initial_value")
                    && ObjectType(ObjectAt(value["initial_value"], Child(path, "initial_value"))) != "rich_text")
                {
                    Fail(ErrorCategory.TypeMismatch, Child(path, "initial_value"), "expected a rich_text block");
                }

                break;
            case "rich_text_list":
                CheckRange(value, "indent", path, SlackLimits.RichTextListIndentMin, SlackLimits.RichTextListIndentMax);
                CheckRange(value, "offset", path, SlackLimits.RichTextListOffsetMin, int.MaxValue);
                CheckRange(value, "border", path, SlackLimits.RichTextListBorderMin, SlackLimits.RichTextListBorderMax);
                break;
            case "rich_text_quote":
                CheckRange(value, "border", path, SlackLimits.RichTextQuoteBorderMin, SlackLimits.RichTextQuoteBorderMax);
                break;
            case "rich_text_preformatted":
                CheckRange(value, "border", path, SlackLimits.RichTextPreformattedBorderMin, SlackLimits.RichTextPreformattedBorderMax);
                break;
            case "overflow":
            case "checkboxes":
            case "radio_buttons":
                var options = ListAt(value["options"], Child(path, "options"));
                SliceLength(
                    options,
                    Child(path, "options"),
                    type switch
                    {
                        "overflow" => SlackLimits.OverflowOptionsMinItems,
                        "checkboxes" => SlackLimits.CheckboxesOptionsMinItems,
                        _ => SlackLimits.RadioButtonsOptionsMinItems,
                    },
                    type switch
                    {
                        "overflow" => SlackLimits.OverflowOptionsMaxItems,
                        "checkboxes" => SlackLimits.CheckboxesOptionsMaxItems,
                        _ => SlackLimits.RadioButtonsOptionsMaxItems,
                    });
                ValidateOptions(options, Child(path, "options"));
                break;
            case "url":
                StringAt(value["url"], Child(path, "url"));
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
                SliceLength(ListAt(value["elements"], Child(path, "elements")), Child(path, "elements"), 0, SlackLimits.ActionsElementsMaxItems);
                break;
            case "alert":
                TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.AlertTextMaxLength);
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
                SliceLength(ListAt(value["elements"], Child(path, "elements")), Child(path, "elements"), 1, SlackLimits.ContextActionsElementsMaxItems);
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
                    StringLength(title, Child(path, "title"), 0, SlackLimits.DataVisualizationTitleMaxLength);
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
                ValidateTaskStatus(value, path);

                // A TaskCardBlock is constructed before it is placed, and plan tasks (which carry no
                // type on the wire) may be pending, so the constructor itself accepts pending.
                // Anywhere a task card appears as a block, it is standalone and cannot be pending.
                if (JsonValues.IsString(value["status"], out var status) && status == "pending" && path != "TaskCardBlock")
                {
                    Fail(ErrorCategory.TypeMismatch, Child(path, "status"), "a standalone task card cannot be pending");
                }

                break;
            case "plan":
                ValidatePlan(value, path);
                break;
            case "input":
                ValidateInput(value, path);
                break;
            case "markdown":
                StringLength(StringAt(value["text"], Child(path, "text")), Child(path, "text"), 0, SlackLimits.MarkdownTextMaxLength);
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
            TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.SectionTextMaxLength);
        }

        if (value.ContainsKey("fields"))
        {
            SliceLength(fields, Child(path, "fields"), 0, SlackLimits.SectionFieldsMaxItems);
            for (var index = 0; index < fields.Count; index++)
            {
                TextLength(fields[index], $"{Child(path, "fields")}[{index}].text", 0, SlackLimits.SectionFieldsItemMaxLength);
            }
        }
    }

    private static void ValidateButton(JsonObject value, string path)
    {
        TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.ButtonTextMaxLength);
        CheckOptionalString(value, "url", path, SlackLimits.ButtonUrlMaxLength);
        CheckOptionalString(value, "value", path, SlackLimits.ButtonValueMaxLength);
        CheckOptionalString(value, "accessibility_label", path, SlackLimits.ButtonAccessibilityLabelMaxLength);
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
            SliceLength(users, Child(path, "visible_to_user_ids"), 0, SlackLimits.IconButtonVisibleToUserIdsMaxItems);
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
            SliceLength(options, Child(path, "options"), 0, SlackLimits.SelectOptionsMaxItems);
            ValidateOptions(options, Child(path, "options"));
        }

        if (hasGroups)
        {
            var groups = ListAt(value["option_groups"], Child(path, "option_groups"));
            SliceLength(groups, Child(path, "option_groups"), 0, SlackLimits.SelectOptionGroupsMaxItems);
            for (var index = 0; index < groups.Count; index++)
            {
                var groupPath = $"{Child(path, "option_groups")}[{index}]";
                ValidateOptionGroup(ObjectAt(groups[index], groupPath), groupPath);
            }
        }

        if (value.ContainsKey("placeholder"))
        {
            TextLength(value["placeholder"], Child(path, "placeholder.text"), 0, SlackLimits.SelectPlaceholderMaxLength);
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

        if (hasSlackFile)
        {
            ValidateSlackFile(ObjectAt(value["slack_file"], Child(path, "slack_file")), Child(path, "slack_file"));
        }

        var block = path == "ImageBlock" || BlockListItem().IsMatch(path);
        CheckOptionalString(value, "image_url", path, block ? SlackLimits.ImageImageUrlMaxLength : SlackLimits.ImageElementImageUrlMaxLength);
        CheckOptionalString(value, "alt_text", path, block ? SlackLimits.ImageAltTextMaxLength : SlackLimits.ImageElementAltTextMaxLength);
        CheckOptionalText(value, "title", path, SlackLimits.ImageTitleMaxLength);
    }

    private static void ValidateSlackFile(JsonObject value, string path)
    {
        if (value.ContainsKey("id") == value.ContainsKey("url"))
        {
            Fail(ErrorCategory.MutuallyExclusive, path, "expected exactly one of id or url");
        }

        if (value.ContainsKey("id") && !SlackFileId().IsMatch(StringAt(value["id"], Child(path, "id"))))
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "id"), "expected an ID matching ^F[A-Z0-9]{8,}$");
        }
    }

    private static void ValidateConversationFilter(JsonObject value, string path)
    {
        if (!value.ContainsKey("include")
            && !value.ContainsKey("exclude_external_shared_channels")
            && !value.ContainsKey("exclude_bot_users"))
        {
            Fail(ErrorCategory.MissingRequired, path, "expected at least one filter field");
        }

        if (value.ContainsKey("include"))
        {
            var include = ListAt(value["include"], Child(path, "include"));
            SliceLength(include, Child(path, "include"), SlackLimits.ConversationFilterIncludeMinItems, 0);
            for (var index = 0; index < include.Count; index++)
            {
                var itemPath = $"{Child(path, "include")}[{index}]";
                if (!ConversationFilterIncludes.Contains(StringAt(include[index], itemPath)))
                {
                    Fail(ErrorCategory.TypeMismatch, itemPath, "expected im, mpim, private, or public");
                }
            }
        }
    }

    private static void ValidateTaskStatus(JsonObject value, string path)
    {
        if (JsonValues.IsString(value["status"], out var status) && !TaskStatuses.Contains(status))
        {
            Fail(ErrorCategory.TypeMismatch, Child(path, "status"), "unknown task status");
        }
    }

    private static void ValidatePlan(JsonObject value, string path)
    {
        var tasks = ListAt(value["tasks"], Child(path, "tasks"));
        SliceLength(tasks, Child(path, "tasks"), 0, SlackLimits.PlanTasksMaxItems);
        var seen = new HashSet<string>(StringComparer.Ordinal);
        for (var index = 0; index < tasks.Count; index++)
        {
            var taskPath = $"{Child(path, "tasks")}[{index}]";
            var task = ObjectAt(tasks[index], taskPath);

            // Plan tasks carry no type on the wire, so the task card rules are applied here.
            foreach (var field in RequiredFields["task_card"])
            {
                Require(task, field, taskPath);
            }

            ValidateTaskStatus(task, taskPath);
            if (!seen.Add(StringAt(task["task_id"], Child(taskPath, "task_id"))))
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "tasks"), "expected unique task IDs");
            }
        }
    }

    private static void ValidateContext(JsonObject value, string path)
    {
        var elements = ListAt(value["elements"], Child(path, "elements"));
        SliceLength(elements, Child(path, "elements"), 0, SlackLimits.ContextElementsMaxItems);
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

        CheckOptionalText(value, "title", path, SlackLimits.CardTitleMaxLength);
        CheckOptionalText(value, "subtitle", path, SlackLimits.CardSubtitleMaxLength);
        CheckOptionalText(value, "body", path, SlackLimits.CardBodyMaxLength);
        CheckOptionalText(value, "subtext", path, SlackLimits.CardSubtextMaxLength);
        if (value["actions"] is JsonArray actions)
        {
            SliceLength(actions, Child(path, "actions"), 0, SlackLimits.CardActionsMaxItems);
        }
    }

    private static void ValidateCarousel(JsonObject value, string path)
    {
        var cards = ListAt(value["elements"], Child(path, "elements"));
        SliceLength(cards, Child(path, "elements"), SlackLimits.CarouselElementsMinItems, SlackLimits.CarouselElementsMaxItems);
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

        CheckOptionalText(value, "title", path, SlackLimits.ContainerTitleMaxLength);
        CheckOptionalText(value, "subtitle", path, SlackLimits.ContainerSubtitleMaxLength);
        var blocks = ListAt(value["child_blocks"], Child(path, "child_blocks"));
        SliceLength(blocks, Child(path, "child_blocks"), SlackLimits.ContainerChildBlocksMinItems, SlackLimits.ContainerChildBlocksMaxItems);
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
        SliceLength(segments, Child(path, "segments"), SlackLimits.DataVisualizationSegmentsMinItems, SlackLimits.DataVisualizationSegmentsMaxItems);
        for (var index = 0; index < segments.Count; index++)
        {
            var segmentPath = $"{Child(path, "segments")}[{index}]";
            ValidateLabelValue(
                ObjectAt(segments[index], segmentPath),
                segmentPath,
                SlackLimits.DataVisualizationSegmentLabelMaxLength,
                positive: true);
        }
    }

    private static void ValidateInput(JsonObject value, string path)
    {
        TextLength(value["label"], Child(path, "label.text"), 0, SlackLimits.InputLabelMaxLength);
        if (value.ContainsKey("hint"))
        {
            TextLength(value["hint"], Child(path, "hint.text"), 0, SlackLimits.InputHintMaxLength);
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
            StringLength(altText, Child(path, "alt_text"), 0, SlackLimits.VideoAltTextMaxLength);
        }

        TextLength(value["title"], Child(path, "title.text"), 0, SlackLimits.VideoTitleMaxLength);
        CheckOptionalString(value, "author_name", path, SlackLimits.VideoAuthorNameMaxLength);
        CheckOptionalString(value, "provider_name", path, SlackLimits.VideoProviderNameMaxLength);
        CheckOptionalText(value, "description", path, SlackLimits.VideoDescriptionMaxLength);
        CheckOptionalString(value, "thumbnail_url", path, SlackLimits.VideoThumbnailUrlMaxLength);
        CheckOptionalString(value, "video_url", path, SlackLimits.VideoVideoUrlMaxLength);
        CheckOptionalString(value, "title_url", path, SlackLimits.VideoTitleUrlMaxLength);
        CheckOptionalString(value, "provider_icon_url", path, SlackLimits.VideoProviderIconUrlMaxLength);
    }

    private static void ValidateView(JsonObject value, string path, string type)
    {
        var blocks = ListAt(value["blocks"], Child(path, "blocks"));
        SliceLength(blocks, Child(path, "blocks"), 0, SlackLimits.ViewBlocksMaxItems);
        ValidateSurface(blocks, type, Child(path, "blocks"));
        CheckOptionalString(value, "private_metadata", path, SlackLimits.ViewPrivateMetadataMaxLength);
        CheckOptionalString(value, "callback_id", path, SlackLimits.ViewCallbackIdMaxLength);
        CheckOptionalString(value, "external_id", path, SlackLimits.ViewExternalIdMaxLength);
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

            CheckOptionalText(value, "title", path, SlackLimits.ViewTitleMaxLength);
            CheckOptionalText(value, "close", path, SlackLimits.ViewCloseMaxLength);
            CheckOptionalText(value, "submit", path, SlackLimits.ViewSubmitMaxLength);
        }
    }

    private static void ValidateOptionGroup(JsonObject value, string path)
    {
        Require(value, "label", path);
        TextLength(value["label"], Child(path, "label.text"), 0, SlackLimits.OptionGroupLabelMaxLength);
        var options = ListAt(value["options"], Child(path, "options"));
        SliceLength(options, Child(path, "options"), SlackLimits.OptionGroupOptionsMinItems, SlackLimits.OptionGroupOptionsMaxItems);
        ValidateOptions(options, Child(path, "options"));
    }

    private static void ValidateFeedbackButton(JsonObject value, string path)
    {
        Require(value, "text", path);
        Require(value, "value", path);
        TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.FeedbackButtonTextMaxLength);
        CheckOptionalString(value, "value", path, SlackLimits.FeedbackButtonValueMaxLength);
        CheckOptionalString(value, "accessibility_label", path, SlackLimits.FeedbackButtonAccessibilityLabelMaxLength);
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
        StringLength(name, Child(path, "name"), 0, SlackLimits.DataVisualizationSeriesNameMaxLength);
        var data = ListAt(value["data"], Child(path, "data"));
        SliceLength(data, Child(path, "data"), SlackLimits.DataVisualizationDataMinItems, SlackLimits.DataVisualizationDataMaxItems);
    }

    private static void ValidateAxisConfig(JsonObject value, string path)
    {
        var categories = ListAt(value["categories"], Child(path, "categories"));
        SliceLength(
            categories,
            Child(path, "categories"),
            SlackLimits.DataVisualizationCategoriesMinItems,
            SlackLimits.DataVisualizationCategoriesMaxItems);
        var seen = new HashSet<string>(StringComparer.Ordinal);
        for (var index = 0; index < categories.Count; index++)
        {
            var itemPath = $"{Child(path, "categories")}[{index}]";
            var category = StringAt(categories[index], itemPath);
            StringLength(category, itemPath, 0, SlackLimits.DataVisualizationCategoryLabelMaxLength);
            if (!seen.Add(category))
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "categories"), "expected unique labels");
            }
        }

        CheckOptionalString(value, "x_label", path, SlackLimits.DataVisualizationAxisLabelMaxLength);
        CheckOptionalString(value, "y_label", path, SlackLimits.DataVisualizationAxisLabelMaxLength);
    }

    private static void ValidateConfirmation(JsonObject value, string path)
    {
        foreach (var (field, maximum) in new[]
        {
            ("title", SlackLimits.ConfirmationTitleMaxLength),
            ("text", SlackLimits.ConfirmationTextMaxLength),
            ("confirm", SlackLimits.ConfirmationConfirmMaxLength),
            ("deny", SlackLimits.ConfirmationDenyMaxLength)
        })
        {
            Require(value, field, path);
            TextLength(value[field], Child(path, field + ".text"), 0, maximum);
        }
    }

    private static void ValidateOption(JsonObject value, string path)
    {
        Require(value, "text", path);
        Require(value, "value", path);
        TextLength(value["text"], Child(path, "text.text"), 0, SlackLimits.OptionTextMaxLength);
        CheckOptionalString(value, "value", path, SlackLimits.OptionValueMaxLength);
        CheckOptionalText(value, "description", path, SlackLimits.OptionDescriptionMaxLength);
        CheckOptionalString(value, "url", path, SlackLimits.OptionUrlMaxLength);
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
        SliceLength(
            rows,
            Child(path, "rows"),
            dataTable ? SlackLimits.DataTableRowsMinItems : 1,
            dataTable ? SlackLimits.DataTableRowsMaxItems : SlackLimits.TableRowsMaxItems);
        var columns = -1;
        var contentLength = 0;
        var allowed = dataTable ? DataTableCellTypes : TableCellTypes;
        for (var rowIndex = 0; rowIndex < rows.Count; rowIndex++)
        {
            var rowPath = $"{Child(path, "rows")}[{rowIndex}]";
            var row = ListAt(rows[rowIndex], rowPath);
            SliceLength(
                row,
                rowPath,
                dataTable ? SlackLimits.DataTableColumnsMinItems : 0,
                dataTable ? SlackLimits.DataTableColumnsMaxItems : SlackLimits.TableColumnsMaxItems);
            if (columns < 0)
            {
                columns = row.Count;
            }
            else if (dataTable && row.Count != columns)
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
                    StringLength(StringAt(cell["text"], Child(cellPath, "text")), Child(cellPath, "text"), SlackLimits.DataTableCellTextMinLength, 0);
                }
            }
        }

        if (value.ContainsKey("column_settings"))
        {
            if (dataTable)
            {
                Fail(ErrorCategory.InvalidUsage, Child(path, "column_settings"), "data tables do not support column_settings");
            }

            SliceLength(
                ListAt(value["column_settings"], Child(path, "column_settings")),
                Child(path, "column_settings"),
                0,
                SlackLimits.TableColumnSettingsMaxItems);
        }

        if (dataTable)
        {
            if (JsonValues.Number(value["page_size"]) is double pageSize
                && (pageSize < SlackLimits.DataTablePageSizeMin || pageSize > SlackLimits.DataTablePageSizeMax))
            {
                Fail(
                    ErrorCategory.OutOfRange,
                    Child(path, "page_size"),
                    $"expected a value between {SlackLimits.DataTablePageSizeMin} and {SlackLimits.DataTablePageSizeMax}");
            }

            CheckRange(value, "row_header_column_index", path, SlackLimits.DataTableRowHeaderColumnIndexMin, int.MaxValue);
            var caption = StringAt(value["caption"], Child(path, "caption"));
            StringLength(caption, Child(path, "caption"), 1, 0);
            if (contentLength > SlackLimits.DataTableContentMaxLength)
            {
                Fail(ErrorCategory.LengthExceeded, Child(path, "rows"), $"content exceeds maximum {SlackLimits.DataTableContentMaxLength}");
            }
        }
    }

    private static void ValidateSeriesChart(JsonObject value, string path)
    {
        var series = ListAt(value["series"], Child(path, "series"));
        SliceLength(series, Child(path, "series"), SlackLimits.DataVisualizationSeriesMinItems, SlackLimits.DataVisualizationSeriesMaxItems);
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
                ValidateLabelValue(point, pointPath, SlackLimits.DataVisualizationPointLabelMaxLength, positive: false);
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

        if (positive && number <= SlackLimits.DataVisualizationSegmentValueExclusiveMin)
        {
            Fail(ErrorCategory.OutOfRange, Child(path, "value"), $"expected a value greater than {SlackLimits.DataVisualizationSegmentValueExclusiveMin}");
        }
    }

    private static void ValidateMessageCollections(JsonObject value, string path)
    {
        if (value.ContainsKey("blocks"))
        {
            var blocks = ListAt(value["blocks"], Child(path, "blocks"));
            SliceLength(blocks, Child(path, "blocks"), 0, SlackLimits.MessageBlocksMaxItems);
            ValidateSurface(blocks, "message", Child(path, "blocks"));
        }

        if (value.ContainsKey("attachments"))
        {
            var attachments = ListAt(value["attachments"], Child(path, "attachments"));
            SliceLength(attachments, Child(path, "attachments"), 0, SlackLimits.MessageAttachmentsMaxItems);
            for (var index = 0; index < attachments.Count; index++)
            {
                var attachmentPath = $"{Child(path, "attachments")}[{index}]";
                Require(ObjectAt(attachments[index], attachmentPath), "blocks", attachmentPath);
            }
        }

        // Slack limits markdown block text and data table cell text across the whole message.
        var markdownTotal = 0;
        var dataTableTotal = 0;
        AddMessageTotals(value, ref markdownTotal, ref dataTableTotal);
        if (markdownTotal > SlackLimits.MarkdownTotalTextMaxLength)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"markdown text totals {markdownTotal}, exceeding maximum {SlackLimits.MarkdownTotalTextMaxLength}");
        }

        if (dataTableTotal > SlackLimits.DataTableTotalContentMaxLength)
        {
            Fail(ErrorCategory.LengthExceeded, path, $"data table content totals {dataTableTotal}, exceeding maximum {SlackLimits.DataTableTotalContentMaxLength}");
        }
    }

    private static void AddMessageTotals(JsonNode? value, ref int markdownTotal, ref int dataTableTotal)
    {
        switch (value)
        {
            case JsonObject obj when ObjectType(obj) == "markdown" && JsonValues.IsString(obj["text"], out var text):
                markdownTotal += JsonValues.CodePoints(text);
                break;
            case JsonObject obj when ObjectType(obj) == "data_table":
                dataTableTotal += TextCharacterCount(obj["rows"]);
                break;
            case JsonObject obj:
                foreach (var (_, nested) in obj)
                {
                    AddMessageTotals(nested, ref markdownTotal, ref dataTableTotal);
                }

                break;
            case JsonArray array:
                foreach (var item in array)
                {
                    AddMessageTotals(item, ref markdownTotal, ref dataTableTotal);
                }

                break;
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

    private static void CheckRange(JsonObject value, string field, string path, int minimum, int maximum)
    {
        if (JsonValues.Number(value[field]) is double number && (number < minimum || number > maximum))
        {
            Fail(
                ErrorCategory.OutOfRange,
                Child(path, field),
                maximum == int.MaxValue ? $"expected a value of at least {minimum}" : $"expected a value between {minimum} and {maximum}");
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
