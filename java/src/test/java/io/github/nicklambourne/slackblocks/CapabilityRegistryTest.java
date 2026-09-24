package io.github.nicklambourne.slackblocks;

import static java.util.stream.Collectors.toCollection;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.JsonParser;
import io.github.nicklambourne.slackblocks.block.ActionsBlock;
import io.github.nicklambourne.slackblocks.block.AlertBlock;
import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.CardBlock;
import io.github.nicklambourne.slackblocks.block.CarouselBlock;
import io.github.nicklambourne.slackblocks.block.ContainerBlock;
import io.github.nicklambourne.slackblocks.block.ContextActionsBlock;
import io.github.nicklambourne.slackblocks.block.ContextActionsElement;
import io.github.nicklambourne.slackblocks.block.ContextBlock;
import io.github.nicklambourne.slackblocks.block.ContextElement;
import io.github.nicklambourne.slackblocks.block.DataTableBlock;
import io.github.nicklambourne.slackblocks.block.DataTableCell;
import io.github.nicklambourne.slackblocks.block.DataVisualizationBlock;
import io.github.nicklambourne.slackblocks.block.DividerBlock;
import io.github.nicklambourne.slackblocks.block.FileBlock;
import io.github.nicklambourne.slackblocks.block.HeaderBlock;
import io.github.nicklambourne.slackblocks.block.ImageBlock;
import io.github.nicklambourne.slackblocks.block.InputBlock;
import io.github.nicklambourne.slackblocks.block.MarkdownBlock;
import io.github.nicklambourne.slackblocks.block.PlanBlock;
import io.github.nicklambourne.slackblocks.block.RichTextBlock;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.block.TableBlock;
import io.github.nicklambourne.slackblocks.block.TableCell;
import io.github.nicklambourne.slackblocks.block.TaskCardBlock;
import io.github.nicklambourne.slackblocks.block.VideoBlock;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.element.ChannelMultiSelectElement;
import io.github.nicklambourne.slackblocks.element.ChannelSelectElement;
import io.github.nicklambourne.slackblocks.element.CheckboxesElement;
import io.github.nicklambourne.slackblocks.element.ConversationMultiSelectElement;
import io.github.nicklambourne.slackblocks.element.ConversationSelectElement;
import io.github.nicklambourne.slackblocks.element.DatePickerElement;
import io.github.nicklambourne.slackblocks.element.DateTimePickerElement;
import io.github.nicklambourne.slackblocks.element.Element;
import io.github.nicklambourne.slackblocks.element.EmailInputElement;
import io.github.nicklambourne.slackblocks.element.ExternalMultiSelectElement;
import io.github.nicklambourne.slackblocks.element.ExternalSelectElement;
import io.github.nicklambourne.slackblocks.element.FeedbackButtonsElement;
import io.github.nicklambourne.slackblocks.element.FileInputElement;
import io.github.nicklambourne.slackblocks.element.IconButtonElement;
import io.github.nicklambourne.slackblocks.element.ImageElement;
import io.github.nicklambourne.slackblocks.element.InputElement;
import io.github.nicklambourne.slackblocks.element.NumberInputElement;
import io.github.nicklambourne.slackblocks.element.OverflowElement;
import io.github.nicklambourne.slackblocks.element.PlainTextInputElement;
import io.github.nicklambourne.slackblocks.element.RadioButtonsElement;
import io.github.nicklambourne.slackblocks.element.RichTextInputElement;
import io.github.nicklambourne.slackblocks.element.StaticMultiSelectElement;
import io.github.nicklambourne.slackblocks.element.StaticSelectElement;
import io.github.nicklambourne.slackblocks.element.TimePickerElement;
import io.github.nicklambourne.slackblocks.element.UrlInputElement;
import io.github.nicklambourne.slackblocks.element.UserMultiSelectElement;
import io.github.nicklambourne.slackblocks.element.UserSelectElement;
import io.github.nicklambourne.slackblocks.element.WorkflowButtonElement;
import io.github.nicklambourne.slackblocks.object.AreaChart;
import io.github.nicklambourne.slackblocks.object.AxisConfig;
import io.github.nicklambourne.slackblocks.object.BarChart;
import io.github.nicklambourne.slackblocks.object.Chart;
import io.github.nicklambourne.slackblocks.object.ChartSegment;
import io.github.nicklambourne.slackblocks.object.ColumnSettings;
import io.github.nicklambourne.slackblocks.object.Confirmation;
import io.github.nicklambourne.slackblocks.object.ConversationFilter;
import io.github.nicklambourne.slackblocks.object.DataPoint;
import io.github.nicklambourne.slackblocks.object.DataSeries;
import io.github.nicklambourne.slackblocks.object.DispatchActionConfiguration;
import io.github.nicklambourne.slackblocks.object.FeedbackButton;
import io.github.nicklambourne.slackblocks.object.InputParameter;
import io.github.nicklambourne.slackblocks.object.LineChart;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import io.github.nicklambourne.slackblocks.object.Option;
import io.github.nicklambourne.slackblocks.object.OptionGroup;
import io.github.nicklambourne.slackblocks.object.PieChart;
import io.github.nicklambourne.slackblocks.object.PlainText;
import io.github.nicklambourne.slackblocks.object.RawNumber;
import io.github.nicklambourne.slackblocks.object.RawText;
import io.github.nicklambourne.slackblocks.object.RichTextBlockElement;
import io.github.nicklambourne.slackblocks.object.RichTextChannel;
import io.github.nicklambourne.slackblocks.object.RichTextCodeBlock;
import io.github.nicklambourne.slackblocks.object.RichTextEmoji;
import io.github.nicklambourne.slackblocks.object.RichTextLink;
import io.github.nicklambourne.slackblocks.object.RichTextList;
import io.github.nicklambourne.slackblocks.object.RichTextQuote;
import io.github.nicklambourne.slackblocks.object.RichTextSection;
import io.github.nicklambourne.slackblocks.object.RichTextSectionElement;
import io.github.nicklambourne.slackblocks.object.RichTextStyle;
import io.github.nicklambourne.slackblocks.object.RichTextText;
import io.github.nicklambourne.slackblocks.object.RichTextUser;
import io.github.nicklambourne.slackblocks.object.RichTextUserGroup;
import io.github.nicklambourne.slackblocks.object.SlackFile;
import io.github.nicklambourne.slackblocks.object.SlackIcon;
import io.github.nicklambourne.slackblocks.object.Text;
import io.github.nicklambourne.slackblocks.object.Trigger;
import io.github.nicklambourne.slackblocks.object.UrlSource;
import io.github.nicklambourne.slackblocks.object.Workflow;
import io.github.nicklambourne.slackblocks.payload.Attachment;
import io.github.nicklambourne.slackblocks.payload.HomeTabView;
import io.github.nicklambourne.slackblocks.payload.MessagePayload;
import io.github.nicklambourne.slackblocks.payload.MessageResponse;
import io.github.nicklambourne.slackblocks.payload.ModalView;
import io.github.nicklambourne.slackblocks.payload.WebhookMessage;
import java.io.File;
import java.lang.reflect.Modifier;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;
import java.util.Objects;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

/**
 * Enforces the shared capability registry ({@code spec/coverage.json}) against this package's
 * public types: every public {@link SlackObject} type maps to a shared capability or is explicitly
 * excluded, and every shared capability has a Java type.
 */
final class CapabilityRegistryTest {
  private static final Path COVERAGE = Path.of("..", "spec", "coverage.json");

  private static final Map<Class<?>, String> CAPABILITY_BY_TYPE =
      Map.ofEntries(
          Map.entry(ActionsBlock.class, "blocks.actions"),
          Map.entry(AlertBlock.class, "blocks.alert"),
          Map.entry(AreaChart.class, "blocks.data_visualization"),
          Map.entry(Attachment.class, "messages.attachment"),
          Map.entry(AxisConfig.class, "objects.axis_config"),
          Map.entry(BarChart.class, "blocks.data_visualization"),
          Map.entry(ButtonElement.class, "elements.button"),
          Map.entry(CardBlock.class, "blocks.card"),
          Map.entry(CarouselBlock.class, "blocks.carousel"),
          Map.entry(ChannelMultiSelectElement.class, "elements.multi_select_channels"),
          Map.entry(ChannelSelectElement.class, "elements.select_channels"),
          Map.entry(ChartSegment.class, "objects.chart_segment"),
          Map.entry(CheckboxesElement.class, "elements.checkboxes"),
          Map.entry(ColumnSettings.class, "objects.column_settings"),
          Map.entry(Confirmation.class, "objects.confirmation"),
          Map.entry(ContainerBlock.class, "blocks.container"),
          Map.entry(ContextActionsBlock.class, "blocks.context_actions"),
          Map.entry(ContextBlock.class, "blocks.context"),
          Map.entry(ConversationFilter.class, "objects.conversation_filter"),
          Map.entry(ConversationMultiSelectElement.class, "elements.multi_select_conversations"),
          Map.entry(ConversationSelectElement.class, "elements.select_conversations"),
          Map.entry(DataPoint.class, "objects.data_point"),
          Map.entry(DataSeries.class, "objects.data_series"),
          Map.entry(DataTableBlock.class, "blocks.data_table"),
          Map.entry(DataVisualizationBlock.class, "blocks.data_visualization"),
          Map.entry(DatePickerElement.class, "elements.date_picker"),
          Map.entry(DateTimePickerElement.class, "elements.datetime_picker"),
          Map.entry(DispatchActionConfiguration.class, "objects.dispatch_action_configuration"),
          Map.entry(DividerBlock.class, "blocks.divider"),
          Map.entry(EmailInputElement.class, "elements.email_input"),
          Map.entry(ExternalMultiSelectElement.class, "elements.multi_select_external"),
          Map.entry(ExternalSelectElement.class, "elements.select_external"),
          Map.entry(FeedbackButton.class, "objects.feedback_button"),
          Map.entry(FeedbackButtonsElement.class, "elements.feedback_buttons"),
          Map.entry(FileBlock.class, "blocks.file"),
          Map.entry(FileInputElement.class, "elements.file_input"),
          Map.entry(HeaderBlock.class, "blocks.header"),
          Map.entry(HomeTabView.class, "views.home_tab"),
          Map.entry(IconButtonElement.class, "elements.icon_button"),
          Map.entry(ImageBlock.class, "blocks.image"),
          Map.entry(ImageElement.class, "elements.image"),
          Map.entry(InputBlock.class, "blocks.input"),
          Map.entry(InputParameter.class, "objects.input_parameter"),
          Map.entry(LineChart.class, "blocks.data_visualization"),
          Map.entry(MarkdownBlock.class, "blocks.markdown"),
          Map.entry(MarkdownText.class, "objects.markdown_text"),
          Map.entry(MessagePayload.class, "messages.message"),
          Map.entry(MessageResponse.class, "messages.message_response"),
          Map.entry(ModalView.class, "views.modal"),
          Map.entry(NumberInputElement.class, "elements.number_input"),
          Map.entry(Option.class, "objects.option"),
          Map.entry(OptionGroup.class, "objects.option_group"),
          Map.entry(OverflowElement.class, "elements.overflow"),
          Map.entry(PieChart.class, "blocks.data_visualization"),
          Map.entry(PlainText.class, "objects.plain_text"),
          Map.entry(PlainTextInputElement.class, "elements.plain_text_input"),
          Map.entry(PlanBlock.class, "blocks.plan"),
          Map.entry(RadioButtonsElement.class, "elements.radio_buttons"),
          Map.entry(RawNumber.class, "objects.raw_number"),
          Map.entry(RawText.class, "objects.raw_text"),
          Map.entry(RichTextBlock.class, "blocks.rich_text"),
          Map.entry(RichTextChannel.class, "rich_text.channel"),
          Map.entry(RichTextCodeBlock.class, "rich_text.code_block"),
          Map.entry(RichTextEmoji.class, "rich_text.emoji"),
          Map.entry(RichTextInputElement.class, "elements.rich_text_input"),
          Map.entry(RichTextLink.class, "rich_text.link"),
          Map.entry(RichTextList.class, "rich_text.list"),
          Map.entry(RichTextQuote.class, "rich_text.quote"),
          Map.entry(RichTextSection.class, "rich_text.section"),
          Map.entry(RichTextText.class, "rich_text.text"),
          Map.entry(RichTextUser.class, "rich_text.user"),
          Map.entry(RichTextUserGroup.class, "rich_text.user_group"),
          Map.entry(SectionBlock.class, "blocks.section"),
          Map.entry(SlackFile.class, "objects.slack_file"),
          Map.entry(SlackIcon.class, "objects.slack_icon"),
          Map.entry(StaticMultiSelectElement.class, "elements.multi_select_static"),
          Map.entry(StaticSelectElement.class, "elements.select_static"),
          Map.entry(TableBlock.class, "blocks.table"),
          Map.entry(TaskCardBlock.class, "blocks.task_card"),
          Map.entry(TimePickerElement.class, "elements.time_picker"),
          Map.entry(Trigger.class, "objects.trigger"),
          Map.entry(UrlInputElement.class, "elements.url_input"),
          Map.entry(UrlSource.class, "elements.url_source"),
          Map.entry(UserMultiSelectElement.class, "elements.multi_select_users"),
          Map.entry(UserSelectElement.class, "elements.select_users"),
          Map.entry(VideoBlock.class, "blocks.video"),
          Map.entry(WebhookMessage.class, "messages.webhook_message"),
          Map.entry(Workflow.class, "objects.workflow"),
          Map.entry(WorkflowButtonElement.class, "elements.workflow_button"));

  /** Public {@link SlackObject} types that are not shared capabilities in their own right. */
  private static final Set<Class<?>> EXCLUDED =
      Set.of(
          // The serializable root every value implements.
          SlackObject.class,
          // Abstract bases: interfaces naming the values accepted in a position. Their concrete
          // implementations are mapped above.
          Block.class,
          Element.class,
          InputElement.class,
          ContextElement.class,
          ContextActionsElement.class,
          TableCell.class,
          DataTableCell.class,
          Chart.class,
          Text.class,
          RichTextBlockElement.class,
          RichTextSectionElement.class,
          // Style flags nested inside rich text elements; covered through the rich_text.*
          // capabilities that carry them, as in the other implementations.
          RichTextStyle.class);

  @Test
  void everyPublicSlackObjectTypeIsMappedOrExcluded() throws Exception {
    Set<String> exported = publicSlackObjectTypes();
    Set<String> unmapped = new TreeSet<>(exported);
    unmapped.removeAll(names(CAPABILITY_BY_TYPE.keySet()));
    unmapped.removeAll(names(EXCLUDED));
    assertTrue(
        unmapped.isEmpty(),
        () ->
            "public SlackObject types with no capability mapping or exclusion in "
                + "CapabilityRegistryTest: "
                + unmapped);
  }

  @Test
  void everyMappedOrExcludedTypeIsAPublicSlackObjectType() throws Exception {
    Set<String> stale = names(CAPABILITY_BY_TYPE.keySet());
    stale.addAll(names(EXCLUDED));
    stale.removeAll(publicSlackObjectTypes());
    assertTrue(
        stale.isEmpty(),
        () -> "mapped or excluded types that are not public SlackObjects: " + stale);
  }

  @Test
  void noTypeIsBothMappedAndExcluded() {
    Set<String> both = names(CAPABILITY_BY_TYPE.keySet());
    both.retainAll(names(EXCLUDED));
    assertTrue(both.isEmpty(), () -> "types both mapped and excluded: " + both);
  }

  @Test
  void everyMappedCapabilityIsInTheSharedRegistry() throws Exception {
    Set<String> registered = registeredCapabilities();
    Set<String> unknown = new TreeSet<>();
    CAPABILITY_BY_TYPE.forEach(
        (type, capability) -> {
          if (!registered.contains(capability)) {
            unknown.add(type.getSimpleName() + " -> " + capability);
          }
        });
    assertTrue(unknown.isEmpty(), () -> "capabilities missing from coverage.json: " + unknown);
  }

  @Test
  void everySharedCapabilityHasAJavaType() throws Exception {
    Set<String> missing = registeredCapabilities();
    missing.removeAll(CAPABILITY_BY_TYPE.values());
    assertTrue(missing.isEmpty(), () -> "coverage.json capabilities with no Java type: " + missing);
  }

  private static Set<String> registeredCapabilities() throws Exception {
    return new TreeSet<>(
        JsonParser.parseString(Files.readString(COVERAGE))
            .getAsJsonObject()
            .getAsJsonObject("capabilities")
            .keySet());
  }

  /** Enumerates the public top-level {@link SlackObject} types compiled into the main classes. */
  private static Set<String> publicSlackObjectTypes() throws Exception {
    Path classes =
        Path.of(
            Objects.requireNonNull(SlackObject.class.getProtectionDomain().getCodeSource())
                .getLocation()
                .toURI());
    assertTrue(Files.isDirectory(classes), () -> "expected a class directory: " + classes);
    try (Stream<Path> files = Files.walk(classes)) {
      return files
          .map(file -> classes.relativize(file).toString())
          .filter(file -> file.endsWith(".class") && !file.contains("$"))
          .filter(file -> !file.endsWith("package-info.class"))
          .map(file -> file.substring(0, file.length() - ".class".length()))
          .map(file -> load(file.replace(File.separatorChar, '.')))
          .filter(type -> Modifier.isPublic(type.getModifiers()))
          .filter(SlackObject.class::isAssignableFrom)
          .map(Class::getName)
          .collect(toCollection(TreeSet::new));
    }
  }

  private static Class<?> load(String name) {
    try {
      return Class.forName(name, false, CapabilityRegistryTest.class.getClassLoader());
    } catch (ClassNotFoundException e) {
      throw new IllegalStateException(name, e);
    }
  }

  private static Set<String> names(Set<Class<?>> types) {
    return types.stream().map(Class::getName).collect(toCollection(TreeSet::new));
  }
}
