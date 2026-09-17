using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;

namespace Slackblocks.Tests;

/// <summary>Enforces the shared capability registry against this package's public types.</summary>
public sealed class CapabilityTests
{
    private static readonly Dictionary<string, string> CapabilityByType = new(StringComparer.Ordinal)
    {
        ["ActionsBlock"] = "blocks.actions",
        ["AlertBlock"] = "blocks.alert",
        ["AreaChart"] = "blocks.data_visualization",
        ["Attachment"] = "messages.attachment",
        ["AxisConfig"] = "objects.axis_config",
        ["BarChart"] = "blocks.data_visualization",
        ["ButtonElement"] = "elements.button",
        ["CardBlock"] = "blocks.card",
        ["CarouselBlock"] = "blocks.carousel",
        ["ChannelMultiSelectElement"] = "elements.multi_select_channels",
        ["ChannelSelectElement"] = "elements.select_channels",
        ["ChartSegment"] = "objects.chart_segment",
        ["CheckboxesElement"] = "elements.checkboxes",
        ["ColumnSettings"] = "objects.column_settings",
        ["Confirmation"] = "objects.confirmation",
        ["ContainerBlock"] = "blocks.container",
        ["ContextActionsBlock"] = "blocks.context_actions",
        ["ContextBlock"] = "blocks.context",
        ["ConversationFilter"] = "objects.conversation_filter",
        ["ConversationMultiSelectElement"] = "elements.multi_select_conversations",
        ["ConversationSelectElement"] = "elements.select_conversations",
        ["DataPoint"] = "objects.data_point",
        ["DataSeries"] = "objects.data_series",
        ["DataTableBlock"] = "blocks.data_table",
        ["DataVisualizationBlock"] = "blocks.data_visualization",
        ["DatePickerElement"] = "elements.date_picker",
        ["DateTimePickerElement"] = "elements.datetime_picker",
        ["DispatchActionConfiguration"] = "objects.dispatch_action_configuration",
        ["DividerBlock"] = "blocks.divider",
        ["EmailInputElement"] = "elements.email_input",
        ["ExternalMultiSelectElement"] = "elements.multi_select_external",
        ["ExternalSelectElement"] = "elements.select_external",
        ["FeedbackButton"] = "objects.feedback_button",
        ["FeedbackButtonsElement"] = "elements.feedback_buttons",
        ["FileBlock"] = "blocks.file",
        ["FileInputElement"] = "elements.file_input",
        ["HeaderBlock"] = "blocks.header",
        ["HomeTabView"] = "views.home_tab",
        ["IconButtonElement"] = "elements.icon_button",
        ["ImageBlock"] = "blocks.image",
        ["ImageElement"] = "elements.image",
        ["InputBlock"] = "blocks.input",
        ["InputParameter"] = "objects.input_parameter",
        ["LineChart"] = "blocks.data_visualization",
        ["MarkdownBlock"] = "blocks.markdown",
        ["MarkdownText"] = "objects.markdown_text",
        ["MessagePayload"] = "messages.message",
        ["MessageResponse"] = "messages.message_response",
        ["ModalView"] = "views.modal",
        ["NumberInputElement"] = "elements.number_input",
        ["Option"] = "objects.option",
        ["OptionGroup"] = "objects.option_group",
        ["OverflowElement"] = "elements.overflow",
        ["PieChart"] = "blocks.data_visualization",
        ["PlainText"] = "objects.plain_text",
        ["PlainTextInputElement"] = "elements.plain_text_input",
        ["PlanBlock"] = "blocks.plan",
        ["RadioButtonsElement"] = "elements.radio_buttons",
        ["RawNumber"] = "objects.raw_number",
        ["RawText"] = "objects.raw_text",
        ["RichTextBlock"] = "blocks.rich_text",
        ["RichTextChannel"] = "rich_text.channel",
        ["RichTextCodeBlock"] = "rich_text.code_block",
        ["RichTextEmoji"] = "rich_text.emoji",
        ["RichTextInputElement"] = "elements.rich_text_input",
        ["RichTextLink"] = "rich_text.link",
        ["RichTextList"] = "rich_text.list",
        ["RichTextQuote"] = "rich_text.quote",
        ["RichTextSection"] = "rich_text.section",
        ["RichTextText"] = "rich_text.text",
        ["RichTextUser"] = "rich_text.user",
        ["RichTextUserGroup"] = "rich_text.user_group",
        ["SectionBlock"] = "blocks.section",
        ["SlackFile"] = "objects.slack_file",
        ["SlackIcon"] = "objects.slack_icon",
        ["StaticMultiSelectElement"] = "elements.multi_select_static",
        ["StaticSelectElement"] = "elements.select_static",
        ["TableBlock"] = "blocks.table",
        ["TaskCardBlock"] = "blocks.task_card",
        ["TimePickerElement"] = "elements.time_picker",
        ["Trigger"] = "objects.trigger",
        ["UrlInputElement"] = "elements.url_input",
        ["UrlSource"] = "elements.url_source",
        ["UserMultiSelectElement"] = "elements.multi_select_users",
        ["UserSelectElement"] = "elements.select_users",
        ["VideoBlock"] = "blocks.video",
        ["WebhookMessage"] = "messages.webhook_message",
        ["Workflow"] = "objects.workflow",
        ["WorkflowButtonElement"] = "elements.workflow_button",
    };

    // Public JSON-producing types that are not shared capabilities: the text base class, the
    // style value nested inside rich text, and the abstract base.
    private static readonly HashSet<string> Excluded = new(StringComparer.Ordinal) { "Text", "RichTextStyle", "SlackObject" };

    [Fact]
    public void EveryPublicValueTypeMapsToASharedCapability()
    {
        var exported = typeof(SlackObject).Assembly.GetExportedTypes()
            .Where(type => typeof(SlackObject).IsAssignableFrom(type) && !Excluded.Contains(type.Name))
            .Select(type => type.Name)
            .Order(StringComparer.Ordinal)
            .ToList();

        Assert.Equal(CapabilityByType.Keys.Order(StringComparer.Ordinal).ToList(), exported);
    }

    [Fact]
    public void EverySharedCapabilityHasACSharpImplementation()
    {
        var coverage = JsonNode.Parse(File.ReadAllText(Repository.PathTo("spec", "coverage.json")))!["capabilities"]!.AsObject();
        var implemented = CapabilityByType.Values.ToHashSet(StringComparer.Ordinal);

        Assert.Equal(coverage.Select(entry => entry.Key).Order(StringComparer.Ordinal), implemented.Order(StringComparer.Ordinal));
    }
}
