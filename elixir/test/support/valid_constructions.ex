defmodule Slackblocks.Test.ValidConstructions do
  @moduledoc false
  alias Slackblocks.{Blocks, Elements, Objects, RichText, Views}
  # Explicit native attempts, maintained independently of the fixture JSON and API generator.
  def all do
    [
      {"attachments/attachment_multi_block",
       fn ->
         Slackblocks.Attachment.new!(
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "I like pretty colours"),
               block_id: "fake_block_id_0"
             ),
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "I don't like pretty colours"),
               block_id: "fake_block_id_1"
             )
           ],
           color: "#8800ff"
         )
       end},
      {"attachments/attachment_simple",
       fn ->
         Slackblocks.Attachment.new!(
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "I like pretty colours"),
               block_id: "fake_block_id"
             )
           ],
           color: "#000000",
           fallback: "Colours preference"
         )
       end},
      {"blocks/actions_block_checkboxes",
       fn ->
         Blocks.Actions.new!(
           elements: [
             Elements.Checkboxes.new!(
               action_id: "actionId-0",
               options: [
                 Objects.Option.new!(
                   text: Objects.MarkdownText.new!(text: "*a*"),
                   value: "a",
                   description: Objects.PlainText.new!(text: "*a*")
                 ),
                 Objects.Option.new!(
                   text: Objects.MarkdownText.new!(text: "*b*"),
                   value: "b",
                   description: Objects.PlainText.new!(text: "*b*")
                 ),
                 Objects.Option.new!(
                   text: Objects.MarkdownText.new!(text: "*c*"),
                   value: "c",
                   description: Objects.PlainText.new!(text: "*c*")
                 )
               ]
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/alert_block",
       fn ->
         Blocks.Alert.new!(
           text: Objects.MarkdownText.new!(text: "The work is mysterious and important."),
           level: :info,
           block_id: "fake_block_id"
         )
       end},
      {"blocks/card_block",
       fn ->
         Blocks.Card.new!(
           hero_image:
             Elements.Image.new!(
               alt_text: "Sample hero image",
               image_url: "https://picsum.photos/400/300"
             ),
           title: Objects.MarkdownText.new!(text: "Lumon Industries"),
           subtitle: Objects.MarkdownText.new!(text: "Committed to work-life balance"),
           body: Objects.MarkdownText.new!(text: "Please enjoy each card equally."),
           actions: [
             Elements.Button.new!(
               text: Objects.PlainText.new!(text: "Action Button"),
               action_id: "button_action"
             )
           ],
           slack_icon: Objects.SlackIcon.new!(name: "bot"),
           subtext: Objects.MarkdownText.new!(text: "A card assembled by slackblocks."),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/carousel_block",
       fn ->
         Blocks.Carousel.new!(
           elements: [
             Blocks.Card.new!(
               title: Objects.MarkdownText.new!(text: "First result"),
               block_id: "card_1"
             ),
             Blocks.Card.new!(
               title: Objects.MarkdownText.new!(text: "Second result"),
               block_id: "card_2"
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/container_block",
       fn ->
         Blocks.Container.new!(
           child_blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "All systems operational."),
               block_id: "child_1"
             )
           ],
           title: Objects.PlainText.new!(text: "Deployment summary"),
           subtitle: Objects.MarkdownText.new!(text: "Production is healthy"),
           width: :standard,
           is_collapsible: false,
           default_collapsed: false,
           has_header_divider: true,
           block_id: "fake_block_id"
         )
       end},
      {"blocks/context_actions_feedback_buttons",
       fn ->
         Blocks.ContextActions.new!(
           elements: [
             Elements.FeedbackButtons.new!(
               positive_button:
                 Objects.FeedbackButton.new!(
                   text: Objects.PlainText.new!(text: "Good"),
                   value: "positive_feedback",
                   accessibility_label: "Mark this response as good"
                 ),
               negative_button:
                 Objects.FeedbackButton.new!(
                   text: Objects.PlainText.new!(text: "Bad"),
                   value: "negative_feedback",
                   accessibility_label: "Mark this response as bad"
                 ),
               action_id: "feedback_buttons_1"
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/context_actions_icon_button",
       fn ->
         Blocks.ContextActions.new!(
           elements: [
             Elements.IconButton.new!(
               text: Objects.PlainText.new!(text: "Delete"),
               icon: :trash,
               action_id: "delete_button",
               value: "delete_item"
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/context_block_text_only",
       fn ->
         Blocks.Context.new!(
           elements: [Objects.MarkdownText.new!(text: "Hello, world!")],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/data_table_block",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [
               Objects.RawText.new!(text: "Name"),
               Objects.RawText.new!(text: "Score")
             ],
             [
               Objects.RawText.new!(text: "Alice"),
               Objects.RawNumber.new!(value: 42, text: "42")
             ]
           ],
           caption: "Team scores",
           page_size: 5,
           row_header_column_index: 0,
           block_id: "fake_block_id"
         )
       end},
      {"blocks/data_visualization_area",
       fn ->
         Blocks.DataVisualization.new!(
           title: "Daily Active Users",
           chart:
             Objects.AreaChart.new!(
               series: [
                 Objects.DataSeries.new!(
                   name: "Free Tier",
                   data: [
                     Objects.DataPoint.new!(label: "Mon", value: 12_000),
                     Objects.DataPoint.new!(label: "Tue", value: 13_500)
                   ]
                 )
               ],
               axis_config:
                 Objects.AxisConfig.new!(
                   categories: ["Mon", "Tue"],
                   x_label: "Day",
                   y_label: "Users"
                 )
             ),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/data_visualization_bar",
       fn ->
         Blocks.DataVisualization.new!(
           title: "Pies by Tastiness",
           chart:
             Objects.BarChart.new!(
               series: [
                 Objects.DataSeries.new!(
                   name: "Pies",
                   data: [
                     Objects.DataPoint.new!(label: "Pumpkin", value: 70),
                     Objects.DataPoint.new!(label: "Blueberry", value: 90)
                   ]
                 )
               ],
               axis_config:
                 Objects.AxisConfig.new!(
                   categories: ["Pumpkin", "Blueberry"],
                   x_label: "Pies",
                   y_label: "Tastiness"
                 )
             ),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/data_visualization_line",
       fn ->
         Blocks.DataVisualization.new!(
           title: "Weekly Paper Sales",
           chart:
             Objects.LineChart.new!(
               series: [
                 Objects.DataSeries.new!(
                   name: "Website",
                   data: [
                     Objects.DataPoint.new!(label: "Week 1", value: 32_000),
                     Objects.DataPoint.new!(label: "Week 2", value: 35_000)
                   ]
                 ),
                 Objects.DataSeries.new!(
                   name: "In-store",
                   data: [
                     Objects.DataPoint.new!(label: "Week 1", value: 28_000),
                     Objects.DataPoint.new!(label: "Week 2", value: 31_000)
                   ]
                 )
               ],
               axis_config:
                 Objects.AxisConfig.new!(
                   categories: ["Week 1", "Week 2"],
                   x_label: "Week",
                   y_label: "Paper Sales (USD)"
                 )
             ),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/data_visualization_pie",
       fn ->
         Blocks.DataVisualization.new!(
           title: "My Favorite Candy Bars",
           chart:
             Objects.PieChart.new!(
               segments: [
                 Objects.ChartSegment.new!(label: "Kit Kat", value: 45),
                 Objects.ChartSegment.new!(label: "Twix", value: 28),
                 Objects.ChartSegment.new!(label: "Crunch", value: 18),
                 Objects.ChartSegment.new!(label: "Milky Way", value: 9)
               ]
             ),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/divider_block_only", fn -> Blocks.Divider.new!(block_id: "fake_block_id") end},
      {"blocks/file_block_only",
       fn ->
         Blocks.File.new!(
           external_id: "external_id",
           source: "remote",
           block_id: "fake_block_id"
         )
       end},
      {"blocks/header_block_emoji_at_limit",
       fn ->
         Blocks.Header.new!(
           text: Objects.PlainText.new!(text: String.duplicate("😀", 150)),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/header_block_only",
       fn ->
         Blocks.Header.new!(
           text: Objects.PlainText.new!(text: "AloHa!"),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/header_block_text_at_limit",
       fn ->
         Blocks.Header.new!(
           text: Objects.PlainText.new!(text: String.duplicate("x", 150)),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/image_block_only",
       fn ->
         Blocks.Image.new!(
           image_url: "https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
           alt_text: "image1",
           title: Objects.PlainText.new!(text: "image1"),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/image_block_slack_file",
       fn ->
         Blocks.Image.new!(
           slack_file:
             Objects.SlackFile.new!(
               url: "https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png"
             ),
           alt_text: "An incredibly cute kitten.",
           block_id: "fake_block_id"
         )
       end},
      {"blocks/input_block_only",
       fn ->
         Blocks.Input.new!(
           label: Objects.PlainText.new!(text: "Label", emoji: true),
           element: Elements.PlainTextInput.new!(action_id: "action"),
           block_id: "fake_block_id",
           hint: Objects.PlainText.new!(text: "Hint", emoji: true),
           optional: true
         )
       end},
      {"blocks/markdown_block_basic",
       fn ->
         Blocks.Markdown.new!(text: "**Hello**, _world_!", block_id: "fake_block_id")
       end},
      {"blocks/markdown_empty",
       fn -> Blocks.Markdown.new!(text: "", block_id: "fake_block_id") end},
      {"blocks/plan_block",
       fn ->
         Blocks.Plan.new!(
           title: "Thinking completed",
           tasks: [
             Blocks.TaskCard.new!(
               task_id: "call_001",
               title: "Fetched user profile information",
               output:
                 Blocks.RichText.new!(
                   elements: [
                     RichText.Section.new!(
                       elements: [RichText.Text.new!(text: "Profile data loaded")]
                     )
                   ],
                   block_id: "plan_output"
                 ),
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "call_002",
               title: "Checked user permissions",
               status: :pending
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/rich_text_block_basic",
       fn ->
         Blocks.RichText.new!(
           elements: [
             RichText.Section.new!(
               elements: [
                 RichText.Text.new!(
                   text: "You 'bout to witness hip-hop in its most purest",
                   style: RichText.Style.new!(bold: true)
                 ),
                 RichText.Text.new!(
                   text: "Most rawest form, flow almost flawless",
                   style: RichText.Style.new!(strike: true)
                 ),
                 RichText.Text.new!(
                   text: "Most hardest, most honest known artist",
                   style: RichText.Style.new!(italic: true)
                 )
               ]
             )
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/section_block_both_text_and_fields",
       fn ->
         Blocks.Section.new!(
           text: Objects.MarkdownText.new!(text: "Hello"),
           fields: [
             Objects.MarkdownText.new!(text: "Are you"),
             Objects.PlainText.new!(text: "There?", emoji: true)
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/section_block_empty_text_field_value",
       fn ->
         Blocks.Section.new!(
           fields: [
             Objects.MarkdownText.new!(text: "Highly"),
             Objects.PlainText.new!(text: "Strung", emoji: true)
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/section_block_fields",
       fn ->
         Blocks.Section.new!(
           text: Objects.MarkdownText.new!(text: "Test:"),
           fields: [
             Objects.PlainText.new!(text: "foo"),
             Objects.MarkdownText.new!(text: "bar")
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/section_block_single_field_value_coercion",
       fn ->
         Blocks.Section.new!(
           fields: [Objects.MarkdownText.new!(text: "Lowly")],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/section_block_text_only",
       fn ->
         Blocks.Section.new!(
           text: Objects.MarkdownText.new!(text: "Hello, world!"),
           block_id: "fake_block_id"
         )
       end},
      {"blocks/table_block",
       fn ->
         Blocks.Table.new!(
           rows: [
             [
               Objects.RawText.new!(text: "Header A"),
               Objects.RawText.new!(text: "Header B")
             ],
             [
               Objects.RawText.new!(text: "Data 1A"),
               Blocks.RichText.new!(
                 elements: [
                   RichText.Section.new!(
                     elements: [
                       RichText.Link.new!(url: "https://slack.com", text: "Data 1B")
                     ]
                   )
                 ]
               )
             ],
             [
               Objects.RawText.new!(text: "Data 2A"),
               Blocks.RichText.new!(
                 elements: [
                   RichText.Section.new!(
                     elements: [
                       RichText.Link.new!(url: "https://slack.com", text: "Data 2B")
                     ]
                   )
                 ]
               )
             ]
           ],
           column_settings: [
             Objects.ColumnSettings.new!(is_wrapped: true),
             Objects.ColumnSettings.new!(align: :right)
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/table_ragged_rows",
       fn ->
         Blocks.Table.new!(
           rows: [
             [
               Objects.RawText.new!(text: "Header A"),
               Objects.RawText.new!(text: "Header B")
             ],
             [Objects.RawText.new!(text: "Only one cell")]
           ],
           block_id: "fake_block_id"
         )
       end},
      {"blocks/task_card_block",
       fn ->
         Blocks.TaskCard.new!(
           task_id: "task_1",
           title: "Fetching weather data",
           output:
             Blocks.RichText.new!(
               elements: [
                 RichText.Section.new!(
                   elements: [
                     RichText.Text.new!(text: "Found weather data for Chicago from 2 sources")
                   ]
                 )
               ],
               block_id: "task_output"
             ),
           sources: [
             Objects.UrlSource.new!(url: "https://weather.com/", text: "weather.com"),
             Objects.UrlSource.new!(
               url: "https://www.accuweather.com/",
               text: "accuweather.com"
             )
           ],
           status: :in_progress,
           block_id: "fake_block_id"
         )
       end},
      {"blocks/video_block_basic",
       fn ->
         Blocks.Video.new!(
           alt_text: "alt",
           thumbnail_url: "https://example.com/t.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/v.mp4",
           block_id: "b1"
         )
       end},
      {"blocks/video_block_full",
       fn ->
         Blocks.Video.new!(
           alt_text: "How to use Slack",
           thumbnail_url: "https://example.com/thumb.png",
           title: Objects.PlainText.new!(text: "Getting Started"),
           video_url: "https://example.com/video.mp4",
           block_id: "video_1",
           author_name: "Slack",
           description: Objects.PlainText.new!(text: "A short intro"),
           provider_icon_url: "https://example.com/icon.png",
           provider_name: "YouTube",
           title_url: "https://example.com"
         )
       end},
      {"elements/button_basic",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "Click Me"),
           action_id: "button",
           value: "click_me"
         )
       end},
      {"elements/button_without_action_id",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "Click Me"),
           value: "click_me"
         )
       end},
      {"elements/button_link",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "Link!"),
           action_id: "button",
           url: "https://ndl.im/"
         )
       end},
      {"elements/button_style",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "Load"),
           action_id: "button",
           value: "im_a_style_button",
           style: :primary
         )
       end},
      {"elements/button_text_at_limit",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: String.duplicate("x", 75)),
           action_id: "button"
         )
       end},
      {"elements/checkbox_basic",
       fn ->
         Elements.Checkboxes.new!(
           action_id: "and...action",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             )
           ],
           initial_options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             )
           ]
         )
       end},
      {"elements/date_picker_basic",
       fn ->
         Elements.DatePicker.new!(
           action_id: "datepicker",
           initial_date: "1970-01-01",
           placeholder: Objects.PlainText.new!(text: "Pick a date")
         )
       end},
      {"elements/datetime_picker_basic",
       fn ->
         Elements.DateTimePicker.new!(
           action_id: "datetime_picker",
           initial_date_time: 1_628_633_830
         )
       end},
      {"elements/email_input_basic",
       fn ->
         Elements.EmailInput.new!(
           action_id: "email_input",
           placeholder: Objects.PlainText.new!(text: "Enter your email")
         )
       end},
      {"elements/file_input_basic",
       fn ->
         Elements.FileInput.new!(
           action_id: "file_input_action_id_1",
           filetypes: ["jpg", "png"],
           max_files: 5
         )
       end},
      {"elements/image_basic",
       fn ->
         Elements.Image.new!(
           alt_text: "Logo for ndl.im",
           image_url: "https://ndl.im/img/logo.png"
         )
       end},
      {"elements/image_slack_file_id",
       fn ->
         Elements.Image.new!(
           alt_text: "An incredibly cute kitten.",
           slack_file: Objects.SlackFile.new!(id: "F0123ABC456")
         )
       end},
      {"elements/image_slack_file_url",
       fn ->
         Elements.Image.new!(
           alt_text: "An incredibly cute kitten.",
           slack_file:
             Objects.SlackFile.new!(
               url: "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
             )
         )
       end},
      {"elements/multi_select_channel",
       fn ->
         Elements.ChannelMultiSelect.new!(
           action_id: "multi_channels_select",
           placeholder: Objects.PlainText.new!(text: "Select channels")
         )
       end},
      {"elements/multi_select_conversation",
       fn ->
         Elements.ConversationMultiSelect.new!(
           action_id: "multi_conversations_select",
           placeholder: Objects.PlainText.new!(text: "Select conversations")
         )
       end},
      {"elements/multi_select_external",
       fn ->
         Elements.ExternalMultiSelect.new!(
           action_id: "multi_external_select",
           min_query_length: 3,
           placeholder: Objects.PlainText.new!(text: "Select items")
         )
       end},
      {"elements/multi_select_static",
       fn ->
         Elements.StaticMultiSelect.new!(
           action_id: "multi_static_select",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             )
           ],
           placeholder: Objects.PlainText.new!(text: "Select one or more")
         )
       end},
      {"elements/multi_select_user",
       fn ->
         Elements.UserMultiSelect.new!(
           action_id: "multi_users_select",
           placeholder: Objects.PlainText.new!(text: "Select one or more users")
         )
       end},
      {"elements/multi_select_user_with_initial_users",
       fn ->
         Elements.UserMultiSelect.new!(
           action_id: "multi_users_select",
           initial_users: ["U064B5H1309", "U063JR973UP"],
           placeholder: Objects.PlainText.new!(text: "Select one or more users")
         )
       end},
      {"elements/number_input_basic",
       fn ->
         Elements.NumberInput.new!(
           action_id: "number_input",
           is_decimal_allowed: false
         )
       end},
      {"elements/number_input_bounds",
       fn ->
         Elements.NumberInput.new!(
           is_decimal_allowed: true,
           action_id: "bounds",
           min_value: -1.5,
           max_value: 10.25
         )
       end},
      {"elements/number_input_scientific_bounds",
       fn ->
         Elements.NumberInput.new!(
           is_decimal_allowed: true,
           min_value: "0.1000",
           max_value: "1e-1"
         )
       end},
      {"elements/number_input_large_exponent",
       fn ->
         Elements.NumberInput.new!(
           is_decimal_allowed: true,
           min_value: "1e309",
           max_value: "2e309"
         )
       end},
      {"elements/overflow_menu_basic",
       fn ->
         Elements.Overflow.new!(
           action_id: "overflow",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "C"),
               value: "C"
             )
           ]
         )
       end},
      {"elements/plaintext_input_basic",
       fn ->
         Elements.PlainTextInput.new!(
           action_id: "plaintext_input",
           placeholder: Objects.PlainText.new!(text: "Enter your plain text")
         )
       end},
      {"elements/radio_button_group_basic",
       fn ->
         Elements.RadioButtons.new!(
           action_id: "radio_buttons",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "C"),
               value: "C"
             )
           ],
           initial_option:
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             )
         )
       end},
      {"elements/rich_text_input_basic",
       fn ->
         Elements.RichTextInput.new!(
           action_id: "action_id",
           initial_value:
             Blocks.RichText.new!(
               elements: [
                 RichText.Section.new!(elements: [RichText.Text.new!(text: "I'm rich")])
               ]
             ),
           focus_on_load: false,
           placeholder: Objects.PlainText.new!(text: "Hello")
         )
       end},
      {"elements/select_menu_channel",
       fn ->
         Elements.ChannelSelect.new!(
           action_id: "channels_select",
           placeholder: Objects.PlainText.new!(text: "Select a channel")
         )
       end},
      {"elements/select_menu_conversation",
       fn ->
         Elements.ConversationSelect.new!(
           action_id: "conversations_select",
           placeholder: Objects.PlainText.new!(text: "Select one conversation")
         )
       end},
      {"elements/select_menu_external",
       fn ->
         Elements.ExternalSelect.new!(
           action_id: "external_select",
           min_query_length: 4,
           placeholder: Objects.PlainText.new!(text: "Select one item")
         )
       end},
      {"elements/select_menu_static",
       fn ->
         Elements.StaticSelect.new!(
           action_id: "static_select",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "C"),
               value: "C"
             )
           ],
           placeholder: Objects.PlainText.new!(text: "Select one item")
         )
       end},
      {"elements/select_menu_user",
       fn ->
         Elements.UserSelect.new!(
           action_id: "users_select",
           placeholder: Objects.PlainText.new!(text: "Select one user")
         )
       end},
      {"elements/timepicker_basic",
       fn ->
         Elements.TimePicker.new!(
           action_id: "timepicker",
           initial_time: "12:00",
           timezone: "Australia/Sydney",
           placeholder: Objects.PlainText.new!(text: "Select your time")
         )
       end},
      {"elements/url_input_basic", fn -> Elements.UrlInput.new!(action_id: "url_text_input") end},
      {"elements/url_source_basic",
       fn ->
         Objects.UrlSource.new!(
           url: "https://docs.slack.dev/",
           text: "Slack API docs"
         )
       end},
      {"elements/workflow_button_basic",
       fn ->
         Elements.WorkflowButton.new!(
           text: Objects.PlainText.new!(text: "Run Your Workflow"),
           workflow:
             Objects.Workflow.new!(
               trigger:
                 Objects.Trigger.new!(
                   url:
                     "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                   customizable_input_parameters: [
                     Objects.InputParameter.new!(name: "name_a", value: "value_a"),
                     Objects.InputParameter.new!(name: "name_b", value: "value_b")
                   ]
                 )
             ),
           action_id: "run_workflow"
         )
       end},
      {"messages/message_two_charts",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks:
             for index <- 0..1 do
               Blocks.DataVisualization.new!(
                 block_id: "chart_#{index}",
                 title: "Chart",
                 chart:
                   Objects.PieChart.new!(
                     segments: [Objects.ChartSegment.new!(label: "A", value: 1)]
                   )
               )
             end
         )
       end},
      {"messages/message_basic",
       fn ->
         Slackblocks.Message.new!(
           channel: "#slackblocks",
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Hello, world!"),
               block_id: "fake_block_id"
             )
           ],
           text: "",
           mrkdwn: true
         )
       end},
      {"messages/message_basic_attachment",
       fn ->
         Slackblocks.Message.new!(
           channel: "#slackblocks",
           attachments: [
             Slackblocks.Attachment.new!(
               blocks: [
                 Blocks.Section.new!(
                   text: Objects.MarkdownText.new!(text: "Hello, world!"),
                   block_id: "block1"
                 )
               ],
               color: "#000000"
             )
           ],
           text: "",
           mrkdwn: true
         )
       end},
      {"messages/message_compound",
       fn ->
         Slackblocks.Message.new!(
           channel: "#slackblocks",
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Block, One"),
               block_id: "fake_block1"
             ),
             Blocks.Image.new!(
               image_url: "http://bit.ly/slack-block-test-image",
               alt_text: "crash",
               title: Objects.PlainText.new!(text: " "),
               block_id: "fake_block3"
             )
           ],
           attachments: [
             Slackblocks.Attachment.new!(
               blocks: [
                 Blocks.Section.new!(
                   text: Objects.MarkdownText.new!(text: "Block, One"),
                   block_id: "fake_block1"
                 )
               ],
               color: "#8800ff"
             ),
             Slackblocks.Attachment.new!(
               blocks: [
                 Blocks.Section.new!(
                   text: Objects.MarkdownText.new!(text: "Block, Two"),
                   block_id: "fake_block2"
                 ),
                 Blocks.Image.new!(
                   image_url: "http://bit.ly/slack-block-test-image",
                   alt_text: "crash",
                   title: Objects.PlainText.new!(text: " "),
                   block_id: "fake_block3"
                 )
               ],
               color: "#ffff00"
             )
           ],
           text: "",
           mrkdwn: true
         )
       end},
      {"messages/message_response",
       fn ->
         Slackblocks.MessageResponse.new!(
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Hello, world!"),
               block_id: "fake_block_id"
             )
           ],
           text: "",
           mrkdwn: true,
           replace_original: false,
           response_type: :ephemeral
         )
       end},
      {"messages/message_with_attachments",
       fn ->
         Slackblocks.Message.new!(
           channel: "#slackblocks",
           attachments: [
             Slackblocks.Attachment.new!(
               blocks: [
                 Blocks.Section.new!(
                   text: Objects.MarkdownText.new!(text: "Hello, world!"),
                   block_id: "fake_block_id"
                 )
               ],
               color: "#ffff00"
             )
           ],
           text: "",
           mrkdwn: true
         )
       end},
      {"messages/message_with_optional_arguments",
       fn ->
         Slackblocks.Message.new!(
           channel: "#slackblocks",
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Hello, world!"),
               block_id: "fake_block_id"
             )
           ],
           text: "",
           mrkdwn: true,
           unfurl_links: false,
           unfurl_media: false
         )
       end},
      {"messages/webhook_message_basic",
       fn ->
         Slackblocks.WebhookMessage.new!(
           blocks: [
             Blocks.Section.new!(
               text:
                 Objects.MarkdownText.new!(
                   text: "You wouldn't do ol' Hook in now, would you, lad?"
                 ),
               block_id: "fake_block_id"
             ),
             Blocks.Section.new!(
               text:
                 Objects.MarkdownText.new!(
                   text: "Well, all right... if you... say you're a codfish."
                 ),
               block_id: "fake_block_id"
             )
           ],
           response_type: :ephemeral,
           replace_original: true,
           unfurl_links: false,
           unfurl_media: false,
           metadata: %{
             "sender" => "Walt",
             "event_payload" => %{
               "type" => "markdown",
               "items" => [
                 %{"type" => "data_table"},
                 %{"type" => "markdown", "text" => String.duplicate("x", 12_001)}
               ]
             }
           }
         )
       end},
      {"messages/webhook_message_delete",
       fn ->
         Slackblocks.WebhookMessage.new!(
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "I'm a codfish."),
               block_id: "fake_block_id"
             ),
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Louder!"),
               block_id: "fake_block_id"
             )
           ],
           attachments: [
             Slackblocks.Attachment.new!(
               blocks: [
                 Blocks.Section.new!(
                   text: Objects.MarkdownText.new!(text: "I'M A CODFISH!"),
                   block_id: "fake_block_id"
                 )
               ]
             )
           ],
           response_type: :in_channel,
           delete_original: true,
           unfurl_links: true,
           unfurl_media: true,
           metadata: %{"sender" => "Walt"}
         )
       end},
      {"objects/confirmation_dialogue_basic",
       fn ->
         Objects.Confirmation.new!(
           title: Objects.PlainText.new!(text: "Maybe?"),
           text: Objects.PlainText.new!(text: "Would you like to play checkers?"),
           confirm: Objects.PlainText.new!(text: "Yes"),
           deny: Objects.PlainText.new!(text: "Nope!")
         )
       end},
      {"objects/conversation_filter_basic",
       fn ->
         Objects.ConversationFilter.new!(
           include: ["public", "mpim"],
           exclude_bot_users: true
         )
       end},
      {"objects/dispatch_action_configuration_basic",
       fn ->
         Objects.DispatchActionConfiguration.new!(trigger_actions_on: ["on_character_entered"])
       end},
      {"objects/input_parameter_basic",
       fn -> Objects.InputParameter.new!(name: "name", value: "value") end},
      {"objects/option_basic",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: "Canberra"),
           value: "canberra"
         )
       end},
      {"objects/option_group_basic",
       fn ->
         Objects.OptionGroup.new!(
           label: Objects.PlainText.new!(text: "Group A"),
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "A"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "B"),
               value: "B"
             ),
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "C"),
               value: "C"
             )
           ]
         )
       end},
      {"objects/option_value_at_limit",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: "At limit"),
           value: String.duplicate("x", 150)
         )
       end},
      {"objects/text_markdown_basic", fn -> Objects.MarkdownText.new!(text: "hi") end},
      {"objects/text_markdown_verbatim",
       fn -> Objects.MarkdownText.new!(text: "hi", verbatim: true) end},
      {"objects/text_plaintext_basic", fn -> Objects.PlainText.new!(text: "hi") end},
      {"objects/text_plaintext_emoji", fn -> Objects.PlainText.new!(text: "hi", emoji: true) end},
      {"objects/trigger_basic",
       fn ->
         Objects.Trigger.new!(
           url: "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
           customizable_input_parameters: [
             Objects.InputParameter.new!(name: "A", value: "A"),
             Objects.InputParameter.new!(name: "B", value: "B")
           ]
         )
       end},
      {"objects/workflow_basic",
       fn ->
         Objects.Workflow.new!(
           trigger:
             Objects.Trigger.new!(
               url: "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
               customizable_input_parameters: [
                 Objects.InputParameter.new!(name: "A", value: "A"),
                 Objects.InputParameter.new!(name: "B", value: "B")
               ]
             )
         )
       end},
      {"rich_text/rich_text_basic",
       fn ->
         RichText.Text.new!(
           text: "I am a bold rich text block!",
           style: RichText.Style.new!(bold: true, italic: true, strike: false)
         )
       end},
      {"rich_text/rich_text_channel_basic",
       fn ->
         RichText.Channel.new!(
           channel_id: "C0261C65XNY",
           style:
             RichText.Style.new!(
               bold: true,
               client_highlight: true,
               highlight: true,
               italic: false,
               strike: true,
               unlink: false
             )
         )
       end},
      {"rich_text/rich_text_code_block_basic",
       fn ->
         RichText.CodeBlock.new!(
           elements: [
             RichText.Text.new!(text: "\ndef hello_world():\n    print('hello, world')")
           ],
           border: 0
         )
       end},
      {"rich_text/rich_text_emoji_basic", fn -> RichText.Emoji.new!(name: "wave") end},
      {"rich_text/rich_text_link_basic",
       fn ->
         RichText.Link.new!(
           url: "https://google.com/",
           text: "Google",
           style: RichText.Style.new!(bold: true, code: true, italic: false, strike: true),
           unsafe: false
         )
       end},
      {"rich_text/rich_text_list_basic",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Oh")]),
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Hi")]),
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Mark")])
           ],
           indent: 0,
           offset: 0,
           border: 1
         )
       end},
      {"rich_text/rich_text_list_ordered",
       fn ->
         RichText.List.new!(
           style: :ordered,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Oh")]),
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Hi")])
           ],
           indent: 1,
           offset: 2,
           border: 1
         )
       end},
      {"rich_text/rich_text_quote_basic",
       fn ->
         RichText.Quote.new!(
           elements: [
             RichText.Text.new!(text: "Great and good are seldom the same man")
           ],
           border: 1
         )
       end},
      {"rich_text/rich_text_section_basic",
       fn ->
         RichText.Section.new!(
           elements: [
             RichText.Text.new!(text: "The only true wisdom is in knowing you know nothing")
           ]
         )
       end},
      {"rich_text/rich_text_user_basic",
       fn ->
         RichText.User.new!(
           user_id: "DR36TNNLA",
           style:
             RichText.Style.new!(
               bold: true,
               client_highlight: true,
               highlight: true,
               italic: false,
               strike: true,
               unlink: false
             )
         )
       end},
      {"rich_text/rich_text_user_group_basic",
       fn ->
         RichText.UserGroup.new!(
           usergroup_id: "C01RGRU0RUK",
           style:
             RichText.Style.new!(
               bold: true,
               client_highlight: true,
               highlight: true,
               italic: false,
               strike: true,
               unlink: false
             )
         )
       end},
      {"views/hometab_view",
       fn ->
         Views.Home.new!(
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "Example Block"),
               block_id: "fake_id"
             )
           ]
         )
       end},
      {"views/modal_with_blocks",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Hello, world!"),
           blocks: [
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "first section block"),
               block_id: "1"
             ),
             Blocks.Divider.new!(block_id: "2"),
             Blocks.Section.new!(
               text: Objects.MarkdownText.new!(text: "second section block"),
               block_id: "3"
             )
           ],
           close: Objects.PlainText.new!(text: "Close button"),
           submit: Objects.PlainText.new!(text: "Submit button")
         )
       end},
      {"views/modal_without_blocks",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Empty"),
           blocks: []
         )
       end}
    ]
  end
end
