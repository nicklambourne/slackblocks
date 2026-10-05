# Explicit public-API constructions, independent of fixture contents at test time.
# Keep one entry for every ID in spec/manifest.json.
module ValidConstructions
  CONSTRUCTIONS = {
    "attachments/attachment_multi_block" => -> {
      Slackblocks::Attachment.new(
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "I like pretty colours"
            ),
            block_id: "fake_block_id_0"
          ),
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "I don't like pretty colours"
            ),
            block_id: "fake_block_id_1"
          )
        ],
        color: "#8800ff"
      )
    },
    "attachments/attachment_simple" => -> {
      Slackblocks::Attachment.new(
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "I like pretty colours"
            ),
            block_id: "fake_block_id"
          )
        ],
        color: "#000000",
        fallback: "Colours preference"
      )
    },
    "blocks/actions_block_checkboxes" => -> {
      Slackblocks::ActionsBlock.new(
        elements: [
          Slackblocks::CheckboxesElement.new(
            action_id: "actionId-0",
            options: [
              Slackblocks::Option.new(
                text: Slackblocks::MarkdownText.new(
                  text: "*a*"
                ),
                value: "a",
                description: Slackblocks::PlainText.new(
                  text: "*a*"
                )
              ),
              Slackblocks::Option.new(
                text: Slackblocks::MarkdownText.new(
                  text: "*b*"
                ),
                value: "b",
                description: Slackblocks::PlainText.new(
                  text: "*b*"
                )
              ),
              Slackblocks::Option.new(
                text: Slackblocks::MarkdownText.new(
                  text: "*c*"
                ),
                value: "c",
                description: Slackblocks::PlainText.new(
                  text: "*c*"
                )
              )
            ]
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/alert_block" => -> {
      Slackblocks::AlertBlock.new(
        text: Slackblocks::MarkdownText.new(
          text: "The work is mysterious and important."
        ),
        level: :info,
        block_id: "fake_block_id"
      )
    },
    "blocks/card_block" => -> {
      Slackblocks::CardBlock.new(
        hero_image: Slackblocks::ImageElement.new(
          alt_text: "Sample hero image",
          image_url: "https://picsum.photos/400/300"
        ),
        title: Slackblocks::MarkdownText.new(
          text: "Lumon Industries"
        ),
        subtitle: Slackblocks::MarkdownText.new(
          text: "Committed to work-life balance"
        ),
        body: Slackblocks::MarkdownText.new(
          text: "Please enjoy each card equally."
        ),
        actions: [
          Slackblocks::ButtonElement.new(
            text: Slackblocks::PlainText.new(
              text: "Action Button"
            ),
            action_id: "button_action"
          )
        ],
        slack_icon: Slackblocks::SlackIcon.new(
          name: "bot"
        ),
        subtext: Slackblocks::MarkdownText.new(
          text: "A card assembled by slackblocks."
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/carousel_block" => -> {
      Slackblocks::CarouselBlock.new(
        elements: [
          Slackblocks::CardBlock.new(
            title: Slackblocks::MarkdownText.new(
              text: "First result"
            ),
            block_id: "card_1"
          ),
          Slackblocks::CardBlock.new(
            title: Slackblocks::MarkdownText.new(
              text: "Second result"
            ),
            block_id: "card_2"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/container_block" => -> {
      Slackblocks::ContainerBlock.new(
        child_blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "All systems operational."
            ),
            block_id: "child_1"
          )
        ],
        title: Slackblocks::PlainText.new(
          text: "Deployment summary"
        ),
        subtitle: Slackblocks::MarkdownText.new(
          text: "Production is healthy"
        ),
        width: :standard,
        is_collapsible: false,
        default_collapsed: false,
        has_header_divider: true,
        block_id: "fake_block_id"
      )
    },
    "blocks/context_actions_feedback_buttons" => -> {
      Slackblocks::ContextActionsBlock.new(
        elements: [
          Slackblocks::FeedbackButtonsElement.new(
            positive_button: Slackblocks::FeedbackButton.new(
              text: Slackblocks::PlainText.new(
                text: "Good"
              ),
              value: "positive_feedback",
              accessibility_label: "Mark this response as good"
            ),
            negative_button: Slackblocks::FeedbackButton.new(
              text: Slackblocks::PlainText.new(
                text: "Bad"
              ),
              value: "negative_feedback",
              accessibility_label: "Mark this response as bad"
            ),
            action_id: "feedback_buttons_1"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/context_actions_icon_button" => -> {
      Slackblocks::ContextActionsBlock.new(
        elements: [
          Slackblocks::IconButtonElement.new(
            text: Slackblocks::PlainText.new(
              text: "Delete"
            ),
            icon: :trash,
            action_id: "delete_button",
            value: "delete_item"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/context_block_text_only" => -> {
      Slackblocks::ContextBlock.new(
        elements: [
          Slackblocks::MarkdownText.new(
            text: "Hello, world!"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/data_table_block" => -> {
      Slackblocks::DataTableBlock.new(
        rows: [
          [
            Slackblocks::RawText.new(
              text: "Name"
            ),
            Slackblocks::RawText.new(
              text: "Score"
            )
          ],
          [
            Slackblocks::RawText.new(
              text: "Alice"
            ),
            Slackblocks::RawNumber.new(
              value: 42,
              text: "42"
            )
          ]
        ],
        caption: "Team scores",
        page_size: 5,
        row_header_column_index: 0,
        block_id: "fake_block_id"
      )
    },
    "blocks/data_visualization_area" => -> {
      Slackblocks::DataVisualizationBlock.new(
        title: "Daily Active Users",
        chart: Slackblocks::AreaChart.new(
          series: [
            Slackblocks::DataSeries.new(
              name: "Free Tier",
              data: [
                Slackblocks::DataPoint.new(
                  label: "Mon",
                  value: 12000
                ),
                Slackblocks::DataPoint.new(
                  label: "Tue",
                  value: 13500
                )
              ]
            )
          ],
          axis_config: Slackblocks::AxisConfig.new(
            categories: [
              "Mon",
              "Tue"
            ],
            x_label: "Day",
            y_label: "Users"
          )
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/data_visualization_bar" => -> {
      Slackblocks::DataVisualizationBlock.new(
        title: "Pies by Tastiness",
        chart: Slackblocks::BarChart.new(
          series: [
            Slackblocks::DataSeries.new(
              name: "Pies",
              data: [
                Slackblocks::DataPoint.new(
                  label: "Pumpkin",
                  value: 70
                ),
                Slackblocks::DataPoint.new(
                  label: "Blueberry",
                  value: 90
                )
              ]
            )
          ],
          axis_config: Slackblocks::AxisConfig.new(
            categories: [
              "Pumpkin",
              "Blueberry"
            ],
            x_label: "Pies",
            y_label: "Tastiness"
          )
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/data_visualization_line" => -> {
      Slackblocks::DataVisualizationBlock.new(
        title: "Weekly Paper Sales",
        chart: Slackblocks::LineChart.new(
          series: [
            Slackblocks::DataSeries.new(
              name: "Website",
              data: [
                Slackblocks::DataPoint.new(
                  label: "Week 1",
                  value: 32000
                ),
                Slackblocks::DataPoint.new(
                  label: "Week 2",
                  value: 35000
                )
              ]
            ),
            Slackblocks::DataSeries.new(
              name: "In-store",
              data: [
                Slackblocks::DataPoint.new(
                  label: "Week 1",
                  value: 28000
                ),
                Slackblocks::DataPoint.new(
                  label: "Week 2",
                  value: 31000
                )
              ]
            )
          ],
          axis_config: Slackblocks::AxisConfig.new(
            categories: [
              "Week 1",
              "Week 2"
            ],
            x_label: "Week",
            y_label: "Paper Sales (USD)"
          )
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/data_visualization_pie" => -> {
      Slackblocks::DataVisualizationBlock.new(
        title: "My Favorite Candy Bars",
        chart: Slackblocks::PieChart.new(
          segments: [
            Slackblocks::ChartSegment.new(
              label: "Kit Kat",
              value: 45
            ),
            Slackblocks::ChartSegment.new(
              label: "Twix",
              value: 28
            ),
            Slackblocks::ChartSegment.new(
              label: "Crunch",
              value: 18
            ),
            Slackblocks::ChartSegment.new(
              label: "Milky Way",
              value: 9
            )
          ]
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/divider_block_only" => -> {
      Slackblocks::DividerBlock.new(
              block_id: "fake_block_id"
            )
    },
    "blocks/file_block_only" => -> {
      Slackblocks::FileBlock.new(
        external_id: "external_id",
        source: "remote",
        block_id: "fake_block_id"
      )
    },
    "blocks/header_block_emoji_at_limit" => -> {
      Slackblocks::HeaderBlock.new(
        text: Slackblocks::PlainText.new(
          text: "😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀"
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/header_block_only" => -> {
      Slackblocks::HeaderBlock.new(
        text: Slackblocks::PlainText.new(
          text: "AloHa!"
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/header_block_text_at_limit" => -> {
      Slackblocks::HeaderBlock.new(
        text: Slackblocks::PlainText.new(
          text: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/image_block_only" => -> {
      Slackblocks::ImageBlock.new(
        image_url: "https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
        alt_text: "image1",
        title: Slackblocks::PlainText.new(
          text: "image1"
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/image_block_slack_file" => -> {
      Slackblocks::ImageBlock.new(
        slack_file: Slackblocks::SlackFile.new(
          url: "https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png"
        ),
        alt_text: "An incredibly cute kitten.",
        block_id: "fake_block_id"
      )
    },
    "blocks/input_block_only" => -> {
      Slackblocks::InputBlock.new(
        label: Slackblocks::PlainText.new(
          text: "Label",
          emoji: true
        ),
        element: Slackblocks::PlainTextInputElement.new(
          action_id: "action"
        ),
        block_id: "fake_block_id",
        hint: Slackblocks::PlainText.new(
          text: "Hint",
          emoji: true
        ),
        optional: true
      )
    },
    "blocks/markdown_block_basic" => -> {
      Slackblocks::MarkdownBlock.new(
        text: "**Hello**, _world_!",
        block_id: "fake_block_id"
      )
    },
    "blocks/markdown_empty" => -> {
      Slackblocks::MarkdownBlock.new(
        text: "",
        block_id: "fake_block_id"
      )
    },
    "blocks/plan_block" => -> {
      Slackblocks::PlanBlock.new(
        title: "Thinking completed",
        tasks: [
          Slackblocks::TaskCardBlock.new(
            task_id: "call_001",
            title: "Fetched user profile information",
            output: Slackblocks::RichTextBlock.new(
              elements: [
                Slackblocks::RichTextSection.new(
                  elements: [
                    Slackblocks::RichTextText.new(
                      text: "Profile data loaded"
                    )
                  ]
                )
              ],
              block_id: "plan_output"
            ),
            status: :complete
          ),
          Slackblocks::TaskCardBlock.new(
            task_id: "call_002",
            title: "Checked user permissions",
            status: :pending
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/rich_text_block_basic" => -> {
      Slackblocks::RichTextBlock.new(
        elements: [
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "You 'bout to witness hip-hop in its most purest",
                style: Slackblocks::RichTextStyle.new(
                  bold: true
                )
              ),
              Slackblocks::RichTextText.new(
                text: "Most rawest form, flow almost flawless",
                style: Slackblocks::RichTextStyle.new(
                  strike: true
                )
              ),
              Slackblocks::RichTextText.new(
                text: "Most hardest, most honest known artist",
                style: Slackblocks::RichTextStyle.new(
                  italic: true
                )
              )
            ]
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/section_block_both_text_and_fields" => -> {
      Slackblocks::SectionBlock.new(
        text: Slackblocks::MarkdownText.new(
          text: "Hello"
        ),
        fields: [
          Slackblocks::MarkdownText.new(
            text: "Are you"
          ),
          Slackblocks::PlainText.new(
            text: "There?",
            emoji: true
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/section_block_empty_text_field_value" => -> {
      Slackblocks::SectionBlock.new(
        fields: [
          Slackblocks::MarkdownText.new(
            text: "Highly"
          ),
          Slackblocks::PlainText.new(
            text: "Strung",
            emoji: true
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/section_block_fields" => -> {
      Slackblocks::SectionBlock.new(
        text: Slackblocks::MarkdownText.new(
          text: "Test:"
        ),
        fields: [
          Slackblocks::PlainText.new(
            text: "foo"
          ),
          Slackblocks::MarkdownText.new(
            text: "bar"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/section_block_single_field_value_coercion" => -> {
      Slackblocks::SectionBlock.new(
        fields: [
          Slackblocks::MarkdownText.new(
            text: "Lowly"
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/section_block_text_only" => -> {
      Slackblocks::SectionBlock.new(
        text: Slackblocks::MarkdownText.new(
          text: "Hello, world!"
        ),
        block_id: "fake_block_id"
      )
    },
    "blocks/table_block" => -> {
      Slackblocks::TableBlock.new(
        rows: [
          [
            Slackblocks::RawText.new(
              text: "Header A"
            ),
            Slackblocks::RawText.new(
              text: "Header B"
            )
          ],
          [
            Slackblocks::RawText.new(
              text: "Data 1A"
            ),
            Slackblocks::RichTextBlock.new(
              elements: [
                Slackblocks::RichTextSection.new(
                  elements: [
                    Slackblocks::RichTextLink.new(
                      url: "https://slack.com",
                      text: "Data 1B"
                    )
                  ]
                )
              ]
            )
          ],
          [
            Slackblocks::RawText.new(
              text: "Data 2A"
            ),
            Slackblocks::RichTextBlock.new(
              elements: [
                Slackblocks::RichTextSection.new(
                  elements: [
                    Slackblocks::RichTextLink.new(
                      url: "https://slack.com",
                      text: "Data 2B"
                    )
                  ]
                )
              ]
            )
          ]
        ],
        column_settings: [
          Slackblocks::ColumnSettings.new(
            is_wrapped: true
          ),
          Slackblocks::ColumnSettings.new(
            align: :right
          )
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/table_ragged_rows" => -> {
      Slackblocks::TableBlock.new(
        rows: [
          [
            Slackblocks::RawText.new(
              text: "Header A"
            ),
            Slackblocks::RawText.new(
              text: "Header B"
            )
          ],
          [
            Slackblocks::RawText.new(
              text: "Only one cell"
            )
          ]
        ],
        block_id: "fake_block_id"
      )
    },
    "blocks/task_card_block" => -> {
      Slackblocks::TaskCardBlock.new(
        task_id: "task_1",
        title: "Fetching weather data",
        output: Slackblocks::RichTextBlock.new(
          elements: [
            Slackblocks::RichTextSection.new(
              elements: [
                Slackblocks::RichTextText.new(
                  text: "Found weather data for Chicago from 2 sources"
                )
              ]
            )
          ],
          block_id: "task_output"
        ),
        sources: [
          Slackblocks::UrlSource.new(
            url: "https://weather.com/",
            text: "weather.com"
          ),
          Slackblocks::UrlSource.new(
            url: "https://www.accuweather.com/",
            text: "accuweather.com"
          )
        ],
        status: :in_progress,
        block_id: "fake_block_id"
      )
    },
    "blocks/video_block_basic" => -> {
      Slackblocks::VideoBlock.new(
        alt_text: "alt",
        thumbnail_url: "https://example.com/t.png",
        title: Slackblocks::PlainText.new(
          text: "Title"
        ),
        video_url: "https://example.com/v.mp4",
        block_id: "b1"
      )
    },
    "blocks/video_block_full" => -> {
      Slackblocks::VideoBlock.new(
        alt_text: "How to use Slack",
        thumbnail_url: "https://example.com/thumb.png",
        title: Slackblocks::PlainText.new(
          text: "Getting Started"
        ),
        video_url: "https://example.com/video.mp4",
        block_id: "video_1",
        author_name: "Slack",
        description: Slackblocks::PlainText.new(
          text: "A short intro"
        ),
        provider_icon_url: "https://example.com/icon.png",
        provider_name: "YouTube",
        title_url: "https://example.com"
      )
    },
    "elements/button_basic" => -> {
      Slackblocks::ButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "Click Me"
        ),
        action_id: "button",
        value: "click_me"
      )
    },
    "elements/button_without_action_id" => -> {
      Slackblocks::ButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "Click Me"
        ),
        value: "click_me"
      )
    },
    "elements/button_link" => -> {
      Slackblocks::ButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "Link!"
        ),
        action_id: "button",
        url: "https://ndl.im/"
      )
    },
    "elements/button_style" => -> {
      Slackblocks::ButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "Load"
        ),
        action_id: "button",
        value: "im_a_style_button",
        style: :primary
      )
    },
    "elements/button_text_at_limit" => -> {
      Slackblocks::ButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        ),
        action_id: "button"
      )
    },
    "elements/checkbox_basic" => -> {
      Slackblocks::CheckboxesElement.new(
        action_id: "and...action",
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          )
        ],
        initial_options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          )
        ]
      )
    },
    "elements/date_picker_basic" => -> {
      Slackblocks::DatePickerElement.new(
        action_id: "datepicker",
        initial_date: "1970-01-01",
        placeholder: Slackblocks::PlainText.new(
          text: "Pick a date"
        )
      )
    },
    "elements/datetime_picker_basic" => -> {
      Slackblocks::DateTimePickerElement.new(
        action_id: "datetime_picker",
        initial_date_time: 1628633830
      )
    },
    "elements/email_input_basic" => -> {
      Slackblocks::EmailInputElement.new(
        action_id: "email_input",
        placeholder: Slackblocks::PlainText.new(
          text: "Enter your email"
        )
      )
    },
    "elements/file_input_basic" => -> {
      Slackblocks::FileInputElement.new(
        action_id: "file_input_action_id_1",
        filetypes: [
          "jpg",
          "png"
        ],
        max_files: 5
      )
    },
    "elements/image_basic" => -> {
      Slackblocks::ImageElement.new(
        alt_text: "Logo for ndl.im",
        image_url: "https://ndl.im/img/logo.png"
      )
    },
    "elements/image_slack_file_id" => -> {
      Slackblocks::ImageElement.new(
        alt_text: "An incredibly cute kitten.",
        slack_file: Slackblocks::SlackFile.new(
          id: "F0123ABC456"
        )
      )
    },
    "elements/image_slack_file_url" => -> {
      Slackblocks::ImageElement.new(
        alt_text: "An incredibly cute kitten.",
        slack_file: Slackblocks::SlackFile.new(
          url: "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
        )
      )
    },
    "elements/multi_select_channel" => -> {
      Slackblocks::ChannelMultiSelectElement.new(
        action_id: "multi_channels_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select channels"
        )
      )
    },
    "elements/multi_select_conversation" => -> {
      Slackblocks::ConversationMultiSelectElement.new(
        action_id: "multi_conversations_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select conversations"
        )
      )
    },
    "elements/multi_select_external" => -> {
      Slackblocks::ExternalMultiSelectElement.new(
        action_id: "multi_external_select",
        min_query_length: 3,
        placeholder: Slackblocks::PlainText.new(
          text: "Select items"
        )
      )
    },
    "elements/multi_select_static" => -> {
      Slackblocks::StaticMultiSelectElement.new(
        action_id: "multi_static_select",
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          )
        ],
        placeholder: Slackblocks::PlainText.new(
          text: "Select one or more"
        )
      )
    },
    "elements/multi_select_user" => -> {
      Slackblocks::UserMultiSelectElement.new(
        action_id: "multi_users_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select one or more users"
        )
      )
    },
    "elements/multi_select_user_with_initial_users" => -> {
      Slackblocks::UserMultiSelectElement.new(
        action_id: "multi_users_select",
        initial_users: [
          "U064B5H1309",
          "U063JR973UP"
        ],
        placeholder: Slackblocks::PlainText.new(
          text: "Select one or more users"
        )
      )
    },
    "elements/number_input_basic" => -> {
      Slackblocks::NumberInputElement.new(
        action_id: "number_input",
        is_decimal_allowed: false
      )
    },
    "elements/overflow_menu_basic" => -> {
      Slackblocks::OverflowElement.new(
        action_id: "overflow",
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "C"
            ),
            value: "C"
          )
        ]
      )
    },
    "elements/plaintext_input_basic" => -> {
      Slackblocks::PlainTextInputElement.new(
        action_id: "plaintext_input",
        placeholder: Slackblocks::PlainText.new(
          text: "Enter your plain text"
        )
      )
    },
    "elements/radio_button_group_basic" => -> {
      Slackblocks::RadioButtonsElement.new(
        action_id: "radio_buttons",
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "C"
            ),
            value: "C"
          )
        ],
        initial_option: Slackblocks::Option.new(
          text: Slackblocks::PlainText.new(
            text: "A"
          ),
          value: "A"
        )
      )
    },
    "elements/rich_text_input_basic" => -> {
      Slackblocks::RichTextInputElement.new(
        action_id: "action_id",
        initial_value: Slackblocks::RichTextBlock.new(
          elements: [
            Slackblocks::RichTextSection.new(
              elements: [
                Slackblocks::RichTextText.new(
                  text: "I'm rich"
                )
              ]
            )
          ]
        ),
        focus_on_load: false,
        placeholder: Slackblocks::PlainText.new(
          text: "Hello"
        )
      )
    },
    "elements/select_menu_channel" => -> {
      Slackblocks::ChannelSelectElement.new(
        action_id: "channels_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select a channel"
        )
      )
    },
    "elements/select_menu_conversation" => -> {
      Slackblocks::ConversationSelectElement.new(
        action_id: "conversations_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select one conversation"
        )
      )
    },
    "elements/select_menu_external" => -> {
      Slackblocks::ExternalSelectElement.new(
        action_id: "external_select",
        min_query_length: 4,
        placeholder: Slackblocks::PlainText.new(
          text: "Select one item"
        )
      )
    },
    "elements/select_menu_static" => -> {
      Slackblocks::StaticSelectElement.new(
        action_id: "static_select",
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "C"
            ),
            value: "C"
          )
        ],
        placeholder: Slackblocks::PlainText.new(
          text: "Select one item"
        )
      )
    },
    "elements/select_menu_user" => -> {
      Slackblocks::UserSelectElement.new(
        action_id: "users_select",
        placeholder: Slackblocks::PlainText.new(
          text: "Select one user"
        )
      )
    },
    "elements/timepicker_basic" => -> {
      Slackblocks::TimePickerElement.new(
        action_id: "timepicker",
        initial_time: "12:00",
        timezone: "Australia/Sydney",
        placeholder: Slackblocks::PlainText.new(
          text: "Select your time"
        )
      )
    },
    "elements/url_input_basic" => -> {
      Slackblocks::UrlInputElement.new(
              action_id: "url_text_input"
            )
    },
    "elements/url_source_basic" => -> {
      Slackblocks::UrlSource.new(
        url: "https://docs.slack.dev/",
        text: "Slack API docs"
      )
    },
    "elements/workflow_button_basic" => -> {
      Slackblocks::WorkflowButtonElement.new(
        text: Slackblocks::PlainText.new(
          text: "Run Your Workflow"
        ),
        workflow: Slackblocks::Workflow.new(
          trigger: Slackblocks::Trigger.new(
            url: "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
            customizable_input_parameters: [
              Slackblocks::InputParameter.new(
                name: "name_a",
                value: "value_a"
              ),
              Slackblocks::InputParameter.new(
                name: "name_b",
                value: "value_b"
              )
            ]
          )
        ),
        action_id: "run_workflow"
      )
    },
    "messages/message_basic" => -> {
      Slackblocks::MessagePayload.new(
        channel: "#slackblocks",
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Hello, world!"
            ),
            block_id: "fake_block_id"
          )
        ],
        text: "",
        mrkdwn: true
      )
    },
    "messages/message_basic_attachment" => -> {
      Slackblocks::MessagePayload.new(
        channel: "#slackblocks",
        attachments: [
          Slackblocks::Attachment.new(
            blocks: [
              Slackblocks::SectionBlock.new(
                text: Slackblocks::MarkdownText.new(
                  text: "Hello, world!"
                ),
                block_id: "block1"
              )
            ],
            color: "#000000"
          )
        ],
        text: "",
        mrkdwn: true
      )
    },
    "messages/message_compound" => -> {
      Slackblocks::MessagePayload.new(
        channel: "#slackblocks",
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Block, One"
            ),
            block_id: "fake_block1"
          ),
          Slackblocks::ImageBlock.new(
            image_url: "http://bit.ly/slack-block-test-image",
            alt_text: "crash",
            title: Slackblocks::PlainText.new(
              text: " "
            ),
            block_id: "fake_block3"
          )
        ],
        attachments: [
          Slackblocks::Attachment.new(
            blocks: [
              Slackblocks::SectionBlock.new(
                text: Slackblocks::MarkdownText.new(
                  text: "Block, One"
                ),
                block_id: "fake_block1"
              )
            ],
            color: "#8800ff"
          ),
          Slackblocks::Attachment.new(
            blocks: [
              Slackblocks::SectionBlock.new(
                text: Slackblocks::MarkdownText.new(
                  text: "Block, Two"
                ),
                block_id: "fake_block2"
              ),
              Slackblocks::ImageBlock.new(
                image_url: "http://bit.ly/slack-block-test-image",
                alt_text: "crash",
                title: Slackblocks::PlainText.new(
                  text: " "
                ),
                block_id: "fake_block3"
              )
            ],
            color: "#ffff00"
          )
        ],
        text: "",
        mrkdwn: true
      )
    },
    "messages/message_response" => -> {
      Slackblocks::MessageResponse.new(
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Hello, world!"
            ),
            block_id: "fake_block_id"
          )
        ],
        text: "",
        mrkdwn: true,
        replace_original: false,
        response_type: :ephemeral
      )
    },
    "messages/message_with_attachments" => -> {
      Slackblocks::MessagePayload.new(
        channel: "#slackblocks",
        attachments: [
          Slackblocks::Attachment.new(
            blocks: [
              Slackblocks::SectionBlock.new(
                text: Slackblocks::MarkdownText.new(
                  text: "Hello, world!"
                ),
                block_id: "fake_block_id"
              )
            ],
            color: "#ffff00"
          )
        ],
        text: "",
        mrkdwn: true
      )
    },
    "messages/message_with_optional_arguments" => -> {
      Slackblocks::MessagePayload.new(
        channel: "#slackblocks",
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Hello, world!"
            ),
            block_id: "fake_block_id"
          )
        ],
        text: "",
        mrkdwn: true,
        unfurl_links: false,
        unfurl_media: false
      )
    },
    "messages/webhook_message_basic" => -> {
      Slackblocks::WebhookMessage.new(
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "You wouldn't do ol' Hook in now, would you, lad?"
            ),
            block_id: "fake_block_id"
          ),
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Well, all right... if you... say you're a codfish."
            ),
            block_id: "fake_block_id"
          )
        ],
        response_type: :ephemeral,
        replace_original: true,
        unfurl_links: false,
        unfurl_media: false,
        metadata: {
          "sender" => "Walt",
          "event_payload" => {"type" => "markdown", "items" => [{"type" => "data_table"}, {"type" => "markdown", "text" => "x" * 12001}]}
        }
      )
    },
    "messages/webhook_message_delete" => -> {
      Slackblocks::WebhookMessage.new(
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "I'm a codfish."
            ),
            block_id: "fake_block_id"
          ),
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "Louder!"
            ),
            block_id: "fake_block_id"
          )
        ],
        attachments: [
          Slackblocks::Attachment.new(
            blocks: [
              Slackblocks::SectionBlock.new(
                text: Slackblocks::MarkdownText.new(
                  text: "I'M A CODFISH!"
                ),
                block_id: "fake_block_id"
              )
            ]
          )
        ],
        response_type: :in_channel,
        delete_original: true,
        unfurl_links: true,
        unfurl_media: true,
        metadata: {
          "sender" => "Walt"
        }
      )
    },
    "objects/confirmation_dialogue_basic" => -> {
      Slackblocks::Confirmation.new(
        title: Slackblocks::PlainText.new(
          text: "Maybe?"
        ),
        text: Slackblocks::PlainText.new(
          text: "Would you like to play checkers?"
        ),
        confirm: Slackblocks::PlainText.new(
          text: "Yes"
        ),
        deny: Slackblocks::PlainText.new(
          text: "Nope!"
        )
      )
    },
    "objects/conversation_filter_basic" => -> {
      Slackblocks::ConversationFilter.new(
        include: [
          "public",
          "mpim"
        ],
        exclude_bot_users: true
      )
    },
    "objects/dispatch_action_configuration_basic" => -> {
      Slackblocks::DispatchActionConfiguration.new(
              trigger_actions_on: [
                "on_character_entered"
              ]
            )
    },
    "objects/input_parameter_basic" => -> {
      Slackblocks::InputParameter.new(
        name: "name",
        value: "value"
      )
    },
    "objects/option_basic" => -> {
      Slackblocks::Option.new(
        text: Slackblocks::PlainText.new(
          text: "Canberra"
        ),
        value: "canberra"
      )
    },
    "objects/option_group_basic" => -> {
      Slackblocks::OptionGroup.new(
        label: Slackblocks::PlainText.new(
          text: "Group A"
        ),
        options: [
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "A"
            ),
            value: "A"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "B"
            ),
            value: "B"
          ),
          Slackblocks::Option.new(
            text: Slackblocks::PlainText.new(
              text: "C"
            ),
            value: "C"
          )
        ]
      )
    },
    "objects/option_value_at_limit" => -> {
      Slackblocks::Option.new(
        text: Slackblocks::PlainText.new(
          text: "At limit"
        ),
        value: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
      )
    },
    "objects/text_markdown_basic" => -> {
      Slackblocks::MarkdownText.new(
              text: "hi"
            )
    },
    "objects/text_markdown_verbatim" => -> {
      Slackblocks::MarkdownText.new(
        text: "hi",
        verbatim: true
      )
    },
    "objects/text_plaintext_basic" => -> {
      Slackblocks::PlainText.new(
              text: "hi"
            )
    },
    "objects/text_plaintext_emoji" => -> {
      Slackblocks::PlainText.new(
        text: "hi",
        emoji: true
      )
    },
    "objects/trigger_basic" => -> {
      Slackblocks::Trigger.new(
        url: "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
        customizable_input_parameters: [
          Slackblocks::InputParameter.new(
            name: "A",
            value: "A"
          ),
          Slackblocks::InputParameter.new(
            name: "B",
            value: "B"
          )
        ]
      )
    },
    "objects/workflow_basic" => -> {
      Slackblocks::Workflow.new(
              trigger: Slackblocks::Trigger.new(
                url: "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                customizable_input_parameters: [
                  Slackblocks::InputParameter.new(
                    name: "A",
                    value: "A"
                  ),
                  Slackblocks::InputParameter.new(
                    name: "B",
                    value: "B"
                  )
                ]
              )
            )
    },
    "rich_text/rich_text_basic" => -> {
      Slackblocks::RichTextText.new(
        text: "I am a bold rich text block!",
        style: Slackblocks::RichTextStyle.new(
          bold: true,
          italic: true,
          strike: false
        )
      )
    },
    "rich_text/rich_text_channel_basic" => -> {
      Slackblocks::RichTextChannel.new(
        channel_id: "C0261C65XNY",
        style: Slackblocks::RichTextStyle.new(
          bold: true,
          client_highlight: true,
          highlight: true,
          italic: false,
          strike: true,
          unlink: false
        )
      )
    },
    "rich_text/rich_text_code_block_basic" => -> {
      Slackblocks::RichTextCodeBlock.new(
        elements: [
          Slackblocks::RichTextText.new(
            text: "\ndef hello_world():\n    print('hello, world')"
          )
        ],
        border: 0
      )
    },
    "rich_text/rich_text_emoji_basic" => -> {
      Slackblocks::RichTextEmoji.new(
              name: "wave"
            )
    },
    "rich_text/rich_text_link_basic" => -> {
      Slackblocks::RichTextLink.new(
        url: "https://google.com/",
        text: "Google",
        style: Slackblocks::RichTextStyle.new(
          bold: true,
          code: true,
          italic: false,
          strike: true
        ),
        unsafe: false
      )
    },
    "rich_text/rich_text_list_basic" => -> {
      Slackblocks::RichTextList.new(
        style: :bullet,
        elements: [
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "Oh"
              )
            ]
          ),
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "Hi"
              )
            ]
          ),
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "Mark"
              )
            ]
          )
        ],
        indent: 0,
        offset: 0,
        border: 1
      )
    },
    "rich_text/rich_text_list_ordered" => -> {
      Slackblocks::RichTextList.new(
        style: :ordered,
        elements: [
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "Oh"
              )
            ]
          ),
          Slackblocks::RichTextSection.new(
            elements: [
              Slackblocks::RichTextText.new(
                text: "Hi"
              )
            ]
          )
        ],
        indent: 1,
        offset: 2,
        border: 1
      )
    },
    "rich_text/rich_text_quote_basic" => -> {
      Slackblocks::RichTextQuote.new(
        elements: [
          Slackblocks::RichTextText.new(
            text: "Great and good are seldom the same man"
          )
        ],
        border: 1
      )
    },
    "rich_text/rich_text_section_basic" => -> {
      Slackblocks::RichTextSection.new(
              elements: [
                Slackblocks::RichTextText.new(
                  text: "The only true wisdom is in knowing you know nothing"
                )
              ]
            )
    },
    "rich_text/rich_text_user_basic" => -> {
      Slackblocks::RichTextUser.new(
        user_id: "DR36TNNLA",
        style: Slackblocks::RichTextStyle.new(
          bold: true,
          client_highlight: true,
          highlight: true,
          italic: false,
          strike: true,
          unlink: false
        )
      )
    },
    "rich_text/rich_text_user_group_basic" => -> {
      Slackblocks::RichTextUserGroup.new(
        user_group_id: "C01RGRU0RUK",
        style: Slackblocks::RichTextStyle.new(
          bold: true,
          client_highlight: true,
          highlight: true,
          italic: false,
          strike: true,
          unlink: false
        )
      )
    },
    "views/hometab_view" => -> {
      Slackblocks::HomeTabView.new(
              blocks: [
                Slackblocks::SectionBlock.new(
                  text: Slackblocks::MarkdownText.new(
                    text: "Example Block"
                  ),
                  block_id: "fake_id"
                )
              ]
            )
    },
    "views/modal_with_blocks" => -> {
      Slackblocks::ModalView.new(
        title: Slackblocks::PlainText.new(
          text: "Hello, world!"
        ),
        blocks: [
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "first section block"
            ),
            block_id: "1"
          ),
          Slackblocks::DividerBlock.new(
            block_id: "2"
          ),
          Slackblocks::SectionBlock.new(
            text: Slackblocks::MarkdownText.new(
              text: "second section block"
            ),
            block_id: "3"
          )
        ],
        close: Slackblocks::PlainText.new(
          text: "Close button"
        ),
        submit: Slackblocks::PlainText.new(
          text: "Submit button"
        )
      )
    },
    "views/modal_without_blocks" => -> {
      Slackblocks::ModalView.new(
        title: Slackblocks::PlainText.new(
          text: "Empty"
        ),
        blocks: []
      )
    }
  }.freeze
end
