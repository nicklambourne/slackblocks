defmodule Slackblocks.Test.InvalidConstructions do
  @moduledoc false
  alias Slackblocks.{Blocks, Elements, Objects, RichText, Views}
  # Explicit native attempts, maintained independently of the fixture JSON and API generator.
  def all do
    [
      {"layout-home-alert",
       fn ->
         Views.Home.new!(
           blocks: [
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block: Blocks.Alert.new!(text: Objects.PlainText.new!(text: "Modal only"))
                     )
                   ]
                 )
               ]
             )
           ]
         )
       end},
      {"layout-modal-fallback-table",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Grid"),
           blocks: [
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
                 )
               ],
               fallback: [Blocks.Table.new!(rows: [[Objects.RawText.new!(text: "Cell")]])]
             )
           ]
         )
       end},
      {"layout-chart-three-items",
       fn ->
         Objects.LayoutRow.new!(
           items: [
             Objects.LayoutItem.new!(block: Blocks.Divider.new!()),
             Objects.LayoutItem.new!(
               block:
                 Blocks.DataVisualization.new!(
                   title: "Chart",
                   chart:
                     Objects.PieChart.new!(
                       segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                     )
                 )
             ),
             Objects.LayoutItem.new!(block: Blocks.Divider.new!())
           ]
         )
       end},
      {"layout-nested-table-total",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.Container.new!(
               title: Objects.PlainText.new!(text: "Group"),
               child_blocks: [
                 Blocks.Layout.new!(
                   label: "Layout",
                   rows: [
                     Objects.LayoutRow.new!(
                       items: [
                         Objects.LayoutItem.new!(
                           block:
                             Blocks.DataTable.new!(
                               rows: [
                                 [Objects.RawText.new!(text: "H")],
                                 [Objects.RawText.new!(text: String.duplicate("x", 10_000))]
                               ],
                               caption: "Data"
                             )
                         )
                       ]
                     )
                   ]
                 ),
                 Blocks.Layout.new!(
                   label: "Layout",
                   rows: [
                     Objects.LayoutRow.new!(
                       items: [
                         Objects.LayoutItem.new!(
                           block:
                             Blocks.DataTable.new!(
                               rows: [
                                 [Objects.RawText.new!(text: "H")],
                                 [Objects.RawText.new!(text: String.duplicate("x", 10_000))]
                               ],
                               caption: "Data"
                             )
                         )
                       ]
                     )
                   ]
                 )
               ]
             )
           ]
         )
       end},
      {"layout-message-budget",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks:
             List.duplicate(
               Blocks.Layout.new!(
                 label: "Layout",
                 rows: [
                   Objects.LayoutRow.new!(
                     items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
                   )
                 ]
               ),
               51
             )
         )
       end},
      {"layout-view-budget",
       fn ->
         Views.Home.new!(
           blocks:
             List.duplicate(
               Blocks.Layout.new!(
                 label: "Layout",
                 rows: [
                   Objects.LayoutRow.new!(
                     items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
                   )
                 ]
               ),
               101
             )
         )
       end},
      {"layout-label-empty",
       fn ->
         Blocks.Layout.new!(
           label: "",
           rows: [
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             )
           ]
         )
       end},
      {"layout-label-too-long",
       fn ->
         Blocks.Layout.new!(
           label: String.duplicate("😀", 151),
           rows: [
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             )
           ]
         )
       end},
      {"layout-rows-empty", fn -> Blocks.Layout.new!(label: "Layout", rows: []) end},
      {"layout-too-many-rows",
       fn ->
         Blocks.Layout.new!(
           label: "Layout",
           rows: [
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             ),
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             ),
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             ),
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             ),
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             ),
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             )
           ]
         )
       end},
      {"layout-items-empty", fn -> Objects.LayoutRow.new!(items: []) end},
      {"layout-too-many-items",
       fn ->
         Objects.LayoutRow.new!(
           items: [
             Objects.LayoutItem.new!(block: Blocks.Divider.new!()),
             Objects.LayoutItem.new!(block: Blocks.Divider.new!()),
             Objects.LayoutItem.new!(block: Blocks.Divider.new!()),
             Objects.LayoutItem.new!(block: Blocks.Divider.new!())
           ]
         )
       end},
      {"layout-markdown-child",
       fn -> Objects.LayoutItem.new!(block: Blocks.Markdown.new!(text: "No")) end},
      {"layout-direct-child",
       fn ->
         Objects.LayoutItem.new!(
           block:
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
                 )
               ]
             )
         )
       end},
      {"layout-fallback-actions",
       fn ->
         Blocks.Layout.new!(
           label: "Layout",
           rows: [
             Objects.LayoutRow.new!(
               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
             )
           ],
           fallback: [
             Blocks.Actions.new!(
               elements: [Elements.Button.new!(text: Objects.PlainText.new!(text: "Go"))]
             )
           ]
         )
       end},
      {"layout-wide-siblings",
       fn ->
         Objects.LayoutRow.new!(
           items: [
             Objects.LayoutItem.new!(
               block:
                 Blocks.Carousel.new!(
                   elements: [Blocks.Card.new!(title: Objects.PlainText.new!(text: "Card"))]
                 )
             ),
             Objects.LayoutItem.new!(
               block:
                 Blocks.DataVisualization.new!(
                   title: "Chart",
                   chart:
                     Objects.PieChart.new!(
                       segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                     )
                 )
             )
           ]
         )
       end},
      {"layout-chart-half",
       fn ->
         Objects.LayoutRow.new!(
           items: [
             Objects.LayoutItem.new!(
               block:
                 Blocks.DataVisualization.new!(
                   title: "Chart",
                   chart:
                     Objects.PieChart.new!(
                       segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                     )
                 )
             ),
             Objects.LayoutItem.new!(block: Blocks.Divider.new!())
           ]
         )
       end},
      {"layout-depth-three",
       fn ->
         Blocks.Layout.new!(
           label: "Layout",
           rows: [
             Objects.LayoutRow.new!(
               items: [
                 Objects.LayoutItem.new!(
                   block:
                     Blocks.Container.new!(
                       title: Objects.PlainText.new!(text: "Container"),
                       child_blocks: [
                         Blocks.Layout.new!(
                           label: "Layout",
                           rows: [
                             Objects.LayoutRow.new!(
                               items: [Objects.LayoutItem.new!(block: Blocks.Divider.new!())]
                             )
                           ]
                         )
                       ]
                     )
                 )
               ]
             )
           ]
         )
       end},
      {"layout-message-alert",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block: Blocks.Alert.new!(text: Objects.PlainText.new!(text: "Modal only"))
                     )
                   ]
                 )
               ]
             )
           ]
         )
       end},
      {"layout-modal-input-submit",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Input"),
           blocks: [
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block:
                         Blocks.Input.new!(
                           label: Objects.PlainText.new!(text: "Name"),
                           element: Elements.PlainTextInput.new!(action_id: "name")
                         )
                     )
                   ]
                 )
               ]
             )
           ]
         )
       end},
      {"layout-three-charts",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.Layout.new!(
               label: "Layout",
               rows: [
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block:
                         Blocks.DataVisualization.new!(
                           title: "Chart",
                           chart:
                             Objects.PieChart.new!(
                               segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                             )
                         )
                     )
                   ]
                 ),
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block:
                         Blocks.DataVisualization.new!(
                           title: "Chart",
                           chart:
                             Objects.PieChart.new!(
                               segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                             )
                         )
                     )
                   ]
                 ),
                 Objects.LayoutRow.new!(
                   items: [
                     Objects.LayoutItem.new!(
                       block:
                         Blocks.DataVisualization.new!(
                           title: "Chart",
                           chart:
                             Objects.PieChart.new!(
                               segments: [Objects.ChartSegment.new!(label: "One", value: 1)]
                             )
                         )
                     )
                   ]
                 )
               ]
             )
           ]
         )
       end},
      {"plain-text-input-inverted-range",
       fn -> Elements.PlainTextInput.new!(min_length: 20, max_length: 10) end},
      {"external-query-negative", fn -> Elements.ExternalSelect.new!(min_query_length: -1) end},
      {"external-multi-query-negative",
       fn -> Elements.ExternalMultiSelect.new!(min_query_length: -1) end},
      {"date-picker-invalid-format",
       fn -> Elements.DatePicker.new!(initial_date: "not a date") end},
      {"time-picker-invalid-format", fn -> Elements.TimePicker.new!(initial_time: "9:00") end},
      {"time-picker-invalid-range", fn -> Elements.TimePicker.new!(initial_time: "25:99") end},
      {"static-select-empty-options", fn -> Elements.StaticSelect.new!(options: []) end},
      {"static-multi-select-empty-options",
       fn -> Elements.StaticMultiSelect.new!(options: []) end},
      {"static-select-empty-groups", fn -> Elements.StaticSelect.new!(option_groups: []) end},
      {"static-multi-select-empty-groups",
       fn -> Elements.StaticMultiSelect.new!(option_groups: []) end},
      {"text-empty", fn -> Objects.PlainText.new!(text: "") end},
      {"text-too-long", fn -> Objects.PlainText.new!(text: String.duplicate("x", 3_001)) end},
      {"button-agent-prompt-too-long",
       fn -> Elements.Button.new!(text: "A", agent_prompt: String.duplicate("😀", 4001)) end},
      {"button-agent-prompt-display-without-prompt",
       fn -> Elements.Button.new!(text: "A", agent_prompt_display: "") end},
      {"button-action-id-too-long",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "A"),
           action_id: String.duplicate("x", 256)
         )
       end},
      {"button-text-too-long",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: String.duplicate("x", 76)),
           action_id: "a"
         )
       end},
      {"button-url-too-long",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "A"),
           action_id: "a",
           url: String.duplicate("x", 3_001)
         )
       end},
      {"button-value-too-long",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "A"),
           action_id: "a",
           value: String.duplicate("x", 2_001)
         )
       end},
      {"confirmation-title-too-long",
       fn ->
         Objects.Confirmation.new!(
           title: Objects.PlainText.new!(text: String.duplicate("x", 101)),
           text: Objects.MarkdownText.new!(text: "Text"),
           confirm: Objects.PlainText.new!(text: "Yes"),
           deny: Objects.PlainText.new!(text: "No")
         )
       end},
      {"confirmation-text-too-long",
       fn ->
         Objects.Confirmation.new!(
           title: Objects.PlainText.new!(text: "Title"),
           text: Objects.MarkdownText.new!(text: String.duplicate("x", 301)),
           confirm: Objects.PlainText.new!(text: "Yes"),
           deny: Objects.PlainText.new!(text: "No")
         )
       end},
      {"confirmation-confirm-too-long",
       fn ->
         Objects.Confirmation.new!(
           title: Objects.PlainText.new!(text: "Title"),
           text: Objects.MarkdownText.new!(text: "Text"),
           confirm: Objects.PlainText.new!(text: String.duplicate("x", 31)),
           deny: Objects.PlainText.new!(text: "No")
         )
       end},
      {"confirmation-deny-too-long",
       fn ->
         Objects.Confirmation.new!(
           title: Objects.PlainText.new!(text: "Title"),
           text: Objects.MarkdownText.new!(text: "Text"),
           confirm: Objects.PlainText.new!(text: "Yes"),
           deny: Objects.PlainText.new!(text: String.duplicate("x", 31))
         )
       end},
      {"option-text-too-long",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: String.duplicate("x", 76)),
           value: "a"
         )
       end},
      {"option-value-too-long",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: "A"),
           value: String.duplicate("x", 151)
         )
       end},
      {"option-url-too-long",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: "A"),
           value: "a",
           url: String.duplicate("🙂", 3_001)
         )
       end},
      {"option-description-too-long",
       fn ->
         Objects.Option.new!(
           text: Objects.PlainText.new!(text: "A"),
           value: "a",
           description: Objects.PlainText.new!(text: String.duplicate("x", 76))
         )
       end},
      {"option-group-label-too-long",
       fn ->
         Objects.OptionGroup.new!(
           label: Objects.PlainText.new!(text: String.duplicate("x", 76)),
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "a"
             )
           ]
         )
       end},
      {"option-group-empty",
       fn ->
         Objects.OptionGroup.new!(
           label: Objects.PlainText.new!(text: "Group"),
           options: []
         )
       end},
      {"option-group-too-many-options",
       fn ->
         Objects.OptionGroup.new!(
           label: Objects.PlainText.new!(text: "Group"),
           options:
             List.duplicate(
               Objects.Option.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 value: "a"
               ),
               101
             )
         )
       end},
      {"select-placeholder-too-long",
       fn ->
         Elements.StaticSelect.new!(
           action_id: "a",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "a"
             )
           ],
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"plain-text-input-placeholder-too-long",
       fn ->
         Elements.PlainTextInput.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"email-input-placeholder-too-long",
       fn ->
         Elements.EmailInput.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"url-input-placeholder-too-long",
       fn ->
         Elements.UrlInput.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"number-input-placeholder-too-long",
       fn ->
         Elements.NumberInput.new!(
           action_id: "a",
           is_decimal_allowed: false,
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"date-picker-placeholder-too-long",
       fn ->
         Elements.DatePicker.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"time-picker-placeholder-too-long",
       fn ->
         Elements.TimePicker.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"rich-text-input-placeholder-too-long",
       fn ->
         Elements.RichTextInput.new!(
           action_id: "a",
           placeholder: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"select-too-many-options",
       fn ->
         Elements.StaticSelect.new!(
           action_id: "a",
           options:
             List.duplicate(
               Objects.Option.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 value: "a"
               ),
               101
             )
         )
       end},
      {"select-too-many-option-groups",
       fn ->
         Elements.StaticSelect.new!(
           action_id: "a",
           option_groups:
             List.duplicate(
               Objects.OptionGroup.new!(
                 label: Objects.PlainText.new!(text: "Group"),
                 options: [
                   Objects.Option.new!(
                     text: Objects.PlainText.new!(text: "A"),
                     value: "a"
                   )
                 ]
               ),
               101
             )
         )
       end},
      {"overflow-empty", fn -> Elements.Overflow.new!(action_id: "a", options: []) end},
      {"overflow-too-many-options",
       fn ->
         Elements.Overflow.new!(
           action_id: "a",
           options:
             List.duplicate(
               Objects.Option.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 value: "a"
               ),
               6
             )
         )
       end},
      {"checkboxes-empty", fn -> Elements.Checkboxes.new!(action_id: "a", options: []) end},
      {"checkboxes-too-many-options",
       fn ->
         Elements.Checkboxes.new!(
           action_id: "a",
           options:
             List.duplicate(
               Objects.Option.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 value: "a"
               ),
               11
             )
         )
       end},
      {"radio-buttons-empty", fn -> Elements.RadioButtons.new!(action_id: "a", options: []) end},
      {"radio-buttons-too-many-options",
       fn ->
         Elements.RadioButtons.new!(
           action_id: "a",
           options:
             List.duplicate(
               Objects.Option.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 value: "a"
               ),
               11
             )
         )
       end},
      {"table-too-many-rows",
       fn ->
         Blocks.Table.new!(rows: List.duplicate([Objects.RawText.new!(text: "A")], 101))
       end},
      {"table-too-many-columns",
       fn ->
         Blocks.Table.new!(rows: [List.duplicate(Objects.RawText.new!(text: "A"), 21)])
       end},
      {"file-input-max-files-too-small",
       fn -> Elements.FileInput.new!(action_id: "a", max_files: 0) end},
      {"file-input-max-files-too-large",
       fn -> Elements.FileInput.new!(action_id: "a", max_files: 11) end},
      {"dispatch-action-no-triggers",
       fn -> Objects.DispatchActionConfiguration.new!(trigger_actions_on: []) end},
      {"dispatch-action-missing-triggers",
       fn -> Objects.DispatchActionConfiguration.new!([]) end},
      {"dispatch-action-too-many-triggers",
       fn ->
         Objects.DispatchActionConfiguration.new!(
           trigger_actions_on: ["on_enter_pressed", "on_character_entered", "on_enter_pressed"]
         )
       end},
      {"plain-text-input-max-length-too-large",
       fn -> Elements.PlainTextInput.new!(action_id: "a", max_length: 3_001) end},
      {"actions-too-many-elements",
       fn ->
         Blocks.Actions.new!(
           elements:
             List.duplicate(
               Elements.Button.new!(
                 text: Objects.PlainText.new!(text: "A"),
                 action_id: "a"
               ),
               26
             )
         )
       end},
      {"context-too-many-elements",
       fn ->
         Blocks.Context.new!(elements: List.duplicate(Objects.MarkdownText.new!(text: "A"), 11))
       end},
      {"header-text-too-long",
       fn ->
         Blocks.Header.new!(text: Objects.PlainText.new!(text: String.duplicate("x", 151)))
       end},
      {"image-url-too-long",
       fn ->
         Blocks.Image.new!(image_url: String.duplicate("x", 3_001), alt_text: "Alt")
       end},
      {"image-alt-text-too-long",
       fn ->
         Blocks.Image.new!(
           image_url: "https://example.com/image.png",
           alt_text: String.duplicate("x", 2_001)
         )
       end},
      {"input-label-too-long",
       fn ->
         Blocks.Input.new!(
           label: Objects.PlainText.new!(text: String.duplicate("x", 2_001)),
           element: Elements.PlainTextInput.new!(action_id: "a")
         )
       end},
      {"input-hint-too-long",
       fn ->
         Blocks.Input.new!(
           label: Objects.PlainText.new!(text: "Label"),
           hint: Objects.PlainText.new!(text: String.duplicate("x", 2_001)),
           element: Elements.PlainTextInput.new!(action_id: "a")
         )
       end},
      {"markdown-too-long", fn -> Blocks.Markdown.new!(text: String.duplicate("x", 12_001)) end},
      {"section-text-too-long",
       fn ->
         Blocks.Section.new!(text: Objects.MarkdownText.new!(text: String.duplicate("x", 3_001)))
       end},
      {"section-too-many-fields",
       fn ->
         Blocks.Section.new!(fields: List.duplicate(Objects.MarkdownText.new!(text: "x"), 11))
       end},
      {"section-field-too-long",
       fn ->
         Blocks.Section.new!(
           fields: [Objects.MarkdownText.new!(text: String.duplicate("x", 2_001))]
         )
       end},
      {"video-alt-text-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: String.duplicate("x", 2_001),
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4"
         )
       end},
      {"video-title-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: String.duplicate("x", 201)),
           video_url: "https://example.com/video.mp4"
         )
       end},
      {"video-author-name-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4",
           author_name: String.duplicate("x", 51)
         )
       end},
      {"video-description-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4",
           description: Objects.PlainText.new!(text: String.duplicate("x", 201))
         )
       end},
      {"video-provider-name-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4",
           provider_name: String.duplicate("x", 51)
         )
       end},
      {"message-channel-empty", fn -> Slackblocks.Message.new!(channel: "") end},
      {"message-too-many-charts",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks:
             for index <- 0..2 do
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
      {"message-too-many-blocks",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: List.duplicate(Blocks.Divider.new!([]), 51)
         )
       end},
      {"message-too-many-attachments",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           attachments:
             List.duplicate(
               Slackblocks.Attachment.new!(blocks: [Blocks.Divider.new!([])]),
               101
             )
         )
       end},
      {"message-invalid-block-surface",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.Alert.new!(text: Objects.PlainText.new!(text: "Modal only"))
           ]
         )
       end},
      {"modal-invalid-block-surface",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Invalid"),
           blocks: [Blocks.Markdown.new!(text: "Message only")]
         )
       end},
      {"home-invalid-block-surface",
       fn ->
         Views.Home.new!(
           blocks: [
             Blocks.Alert.new!(text: Objects.PlainText.new!(text: "Modal only"))
           ]
         )
       end},
      {"modal-input-requires-submit",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Missing submit"),
           blocks: [
             Blocks.Input.new!(
               label: Objects.PlainText.new!(text: "Name"),
               element: Elements.PlainTextInput.new!(action_id: "a")
             )
           ]
         )
       end},
      {"view-too-many-blocks",
       fn ->
         Views.Home.new!(blocks: List.duplicate(Blocks.Divider.new!([]), 101))
       end},
      {"view-private-metadata-too-long",
       fn ->
         Views.Home.new!(
           blocks: [Blocks.Divider.new!([])],
           private_metadata: String.duplicate("x", 3_001)
         )
       end},
      {"view-callback-id-too-long",
       fn ->
         Views.Home.new!(
           blocks: [Blocks.Divider.new!([])],
           callback_id: String.duplicate("x", 256)
         )
       end},
      {"view-title-too-long",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: String.duplicate("x", 25)),
           blocks: [Blocks.Divider.new!([])]
         )
       end},
      {"view-close-too-long",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Title"),
           close: Objects.PlainText.new!(text: String.duplicate("x", 25)),
           blocks: [Blocks.Divider.new!([])]
         )
       end},
      {"view-submit-too-long",
       fn ->
         Views.Modal.new!(
           title: Objects.PlainText.new!(text: "Title"),
           submit: Objects.PlainText.new!(text: String.duplicate("x", 25)),
           blocks: [Blocks.Divider.new!([])]
         )
       end},
      {"section-missing-content", fn -> Blocks.Section.new!([]) end},
      {"section-empty-fields", fn -> Blocks.Section.new!(fields: []) end},
      {"static-select-options-and-groups",
       fn ->
         Elements.StaticSelect.new!(
           action_id: "a",
           options: [
             Objects.Option.new!(
               text: Objects.PlainText.new!(text: "A"),
               value: "a"
             )
           ],
           option_groups: [
             Objects.OptionGroup.new!(
               label: Objects.PlainText.new!(text: "A"),
               options: [
                 Objects.Option.new!(
                   text: Objects.PlainText.new!(text: "B"),
                   value: "b"
                 )
               ]
             )
           ]
         )
       end},
      {"image-url-and-slack-file",
       fn ->
         Elements.Image.new!(
           alt_text: "image",
           image_url: "https://example.com/image.png",
           slack_file: Objects.SlackFile.new!(id: "F0123ABC456")
         )
       end},
      {"number-input-inverted-range",
       fn ->
         Elements.NumberInput.new!(
           action_id: "a",
           is_decimal_allowed: true,
           min_value: 2,
           max_value: 1
         )
       end},
      {"context-invalid-element",
       fn ->
         Slackblocks.from_map!(Blocks.Context, %{
           "type" => "context",
           "elements" => [%{"type" => "divider"}]
         })
       end},
      {"input-invalid-element",
       fn ->
         Slackblocks.from_map!(Blocks.Input, %{
           "type" => "input",
           "label" => %{"type" => "plain_text", "text" => "Label"},
           "element" => %{
             "type" => "button",
             "text" => %{"type" => "plain_text", "text" => "A"},
             "action_id" => "a"
           }
         })
       end},
      {"block-id-too-long", fn -> Blocks.Divider.new!(block_id: String.duplicate("x", 256)) end},
      {"button-accessibility-label-too-long",
       fn ->
         Elements.Button.new!(
           text: Objects.PlainText.new!(text: "A"),
           action_id: "a",
           accessibility_label: String.duplicate("x", 76)
         )
       end},
      {"workflow-button-text-too-long",
       fn ->
         Elements.WorkflowButton.new!(
           text: Objects.PlainText.new!(text: String.duplicate("x", 76)),
           workflow:
             Objects.Workflow.new!(
               trigger: Objects.Trigger.new!(url: "https://slack.com/shortcuts/Ft0/abc")
             ),
           action_id: "a"
         )
       end},
      {"workflow-button-accessibility-label-too-long",
       fn ->
         Elements.WorkflowButton.new!(
           text: Objects.PlainText.new!(text: "Run"),
           workflow:
             Objects.Workflow.new!(
               trigger: Objects.Trigger.new!(url: "https://slack.com/shortcuts/Ft0/abc")
             ),
           action_id: "a",
           accessibility_label: String.duplicate("x", 76)
         )
       end},
      {"alert-text-too-long",
       fn ->
         Blocks.Alert.new!(text: Objects.PlainText.new!(text: String.duplicate("x", 201)))
       end},
      {"card-title-too-long",
       fn ->
         Blocks.Card.new!(title: Objects.PlainText.new!(text: String.duplicate("x", 151)))
       end},
      {"card-subtitle-too-long",
       fn ->
         Blocks.Card.new!(
           title: Objects.PlainText.new!(text: "Card"),
           subtitle: Objects.PlainText.new!(text: String.duplicate("x", 151))
         )
       end},
      {"card-body-too-long",
       fn ->
         Blocks.Card.new!(body: Objects.PlainText.new!(text: String.duplicate("x", 201)))
       end},
      {"card-too-many-actions",
       fn ->
         Blocks.Card.new!(
           title: Objects.PlainText.new!(text: "Card"),
           actions: [
             Elements.Button.new!(
               text: Objects.PlainText.new!(text: "A"),
               action_id: "a"
             ),
             Elements.Button.new!(
               text: Objects.PlainText.new!(text: "A"),
               action_id: "a"
             ),
             Elements.Button.new!(
               text: Objects.PlainText.new!(text: "A"),
               action_id: "a"
             ),
             Elements.Button.new!(
               text: Objects.PlainText.new!(text: "A"),
               action_id: "a"
             )
           ]
         )
       end},
      {"card-subtext-too-long",
       fn ->
         Blocks.Card.new!(
           title: Objects.PlainText.new!(text: "Card"),
           subtext: Objects.PlainText.new!(text: String.duplicate("x", 201))
         )
       end},
      {"carousel-empty", fn -> Blocks.Carousel.new!(elements: []) end},
      {"carousel-too-many-cards",
       fn ->
         Blocks.Carousel.new!(
           elements:
             List.duplicate(
               Blocks.Card.new!(title: Objects.PlainText.new!(text: "Card")),
               11
             )
         )
       end},
      {"container-title-too-long",
       fn ->
         Blocks.Container.new!(
           title: Objects.PlainText.new!(text: String.duplicate("x", 151)),
           child_blocks: [Blocks.Divider.new!([])]
         )
       end},
      {"container-subtitle-too-long",
       fn ->
         Blocks.Container.new!(
           title: Objects.PlainText.new!(text: "Container"),
           subtitle: Objects.PlainText.new!(text: String.duplicate("x", 151)),
           child_blocks: [Blocks.Divider.new!([])]
         )
       end},
      {"container-too-many-child-blocks",
       fn ->
         Blocks.Container.new!(
           title: Objects.PlainText.new!(text: "Container"),
           child_blocks: List.duplicate(Blocks.Divider.new!([]), 11)
         )
       end},
      {"context-actions-too-many-elements",
       fn ->
         Blocks.ContextActions.new!(
           elements:
             List.duplicate(
               Elements.IconButton.new!(
                 text: Objects.PlainText.new!(text: "Delete"),
                 icon: :trash
               ),
               6
             )
         )
       end},
      {"feedback-button-text-too-long",
       fn ->
         Elements.FeedbackButtons.new!(
           positive_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: String.duplicate("x", 76)),
               value: "good"
             ),
           negative_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: "Bad"),
               value: "bad"
             )
         )
       end},
      {"feedback-button-value-too-long",
       fn ->
         Elements.FeedbackButtons.new!(
           positive_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: "Good"),
               value: String.duplicate("x", 2_001)
             ),
           negative_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: "Bad"),
               value: "bad"
             )
         )
       end},
      {"feedback-button-accessibility-label-too-long",
       fn ->
         Elements.FeedbackButtons.new!(
           positive_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: "Good"),
               value: "good",
               accessibility_label: String.duplicate("x", 76)
             ),
           negative_button:
             Objects.FeedbackButton.new!(
               text: Objects.PlainText.new!(text: "Bad"),
               value: "bad"
             )
         )
       end},
      {"icon-button-too-many-visible-users",
       fn ->
         Elements.IconButton.new!(
           text: Objects.PlainText.new!(text: "Delete"),
           icon: :trash,
           visible_to_user_ids: [
             "U0",
             "U1",
             "U2",
             "U3",
             "U4",
             "U5",
             "U6",
             "U7",
             "U8",
             "U9",
             "U10"
           ]
         )
       end},
      {"icon-button-value-too-long",
       fn ->
         Elements.IconButton.new!(
           text: Objects.PlainText.new!(text: "Delete"),
           icon: :trash,
           value: String.duplicate("x", 2_001)
         )
       end},
      {"icon-button-accessibility-label-too-long",
       fn ->
         Elements.IconButton.new!(
           text: Objects.PlainText.new!(text: "Delete"),
           icon: :trash,
           accessibility_label: String.duplicate("x", 76)
         )
       end},
      {"data-table-too-few-rows",
       fn ->
         Blocks.DataTable.new!(
           rows: [[Objects.RawText.new!(text: "Name")]],
           caption: "Names"
         )
       end},
      {"data-table-too-many-rows",
       fn ->
         Blocks.DataTable.new!(
           rows: List.duplicate([Objects.RawText.new!(text: "A")], 202),
           caption: "Names"
         )
       end},
      {"data-table-too-few-columns",
       fn -> Blocks.DataTable.new!(rows: [[], []], caption: "Empty") end},
      {"data-table-too-many-columns",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             List.duplicate(Objects.RawText.new!(text: "A"), 21),
             List.duplicate(Objects.RawText.new!(text: "A"), 21)
           ],
           caption: "Wide"
         )
       end},
      {"data-table-page-size-too-small",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [Objects.RawText.new!(text: "Name")],
             [Objects.RawText.new!(text: "Alice")]
           ],
           caption: "Names",
           page_size: 0
         )
       end},
      {"data-table-page-size-too-large",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [Objects.RawText.new!(text: "Name")],
             [Objects.RawText.new!(text: "Alice")]
           ],
           caption: "Names",
           page_size: 101
         )
       end},
      {"data-table-cell-text-empty",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [Objects.RawText.new!(text: "Name")],
             [Objects.RawText.new!(text: "")]
           ],
           caption: "Names"
         )
       end},
      {"data-table-content-too-long",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [Objects.RawText.new!(text: "Name")],
             [Objects.RawText.new!(text: String.duplicate("x", 20_000))]
           ],
           caption: "Names"
         )
       end},
      {"data-visualization-title-too-long",
       fn ->
         Blocks.DataVisualization.new!(
           title: String.duplicate("x", 51),
           chart:
             Objects.PieChart.new!(segments: [Objects.ChartSegment.new!(label: "A", value: 1)])
         )
       end},
      {"pie-chart-empty", fn -> Objects.PieChart.new!(segments: []) end},
      {"pie-chart-too-many-segments",
       fn ->
         Objects.PieChart.new!(
           segments: List.duplicate(Objects.ChartSegment.new!(label: "S", value: 1), 13)
         )
       end},
      {"chart-segment-label-too-long",
       fn -> Objects.ChartSegment.new!(label: "xxxxxxxxxxxxxxxxxxxxx", value: 1) end},
      {"chart-segment-value-not-positive",
       fn -> Objects.ChartSegment.new!(label: "A", value: 0) end},
      {"chart-series-empty",
       fn ->
         Objects.LineChart.new!(
           axis_config: Objects.AxisConfig.new!(categories: ["A"]),
           series: []
         )
       end},
      {"chart-duplicate-point-labels",
       fn ->
         Objects.LineChart.new!(
           axis_config: Objects.AxisConfig.new!(categories: ["A", "B"]),
           series: [
             Objects.DataSeries.new!(
               name: "Series",
               data: [
                 Objects.DataPoint.new!(label: "A", value: 1),
                 Objects.DataPoint.new!(label: "A", value: 2)
               ]
             )
           ]
         )
       end},
      {"chart-too-many-series",
       fn ->
         Objects.LineChart.new!(
           series: [
             Objects.DataSeries.new!(
               name: "S0",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S1",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S2",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S3",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S4",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S5",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S6",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S7",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S8",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S9",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S10",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S11",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             ),
             Objects.DataSeries.new!(
               name: "S12",
               data: [Objects.DataPoint.new!(label: "A", value: 1)]
             )
           ],
           axis_config: Objects.AxisConfig.new!(categories: ["A"])
         )
       end},
      {"data-series-name-too-long",
       fn ->
         Objects.DataSeries.new!(
           name: "xxxxxxxxxxxxxxxxxxxxx",
           data: [Objects.DataPoint.new!(label: "A", value: 1)]
         )
       end},
      {"data-series-empty", fn -> Objects.DataSeries.new!(name: "Series", data: []) end},
      {"data-series-too-many-points",
       fn ->
         Objects.DataSeries.new!(
           name: "Series",
           data: [
             Objects.DataPoint.new!(label: "P0", value: 0),
             Objects.DataPoint.new!(label: "P1", value: 1),
             Objects.DataPoint.new!(label: "P2", value: 2),
             Objects.DataPoint.new!(label: "P3", value: 3),
             Objects.DataPoint.new!(label: "P4", value: 4),
             Objects.DataPoint.new!(label: "P5", value: 5),
             Objects.DataPoint.new!(label: "P6", value: 6),
             Objects.DataPoint.new!(label: "P7", value: 7),
             Objects.DataPoint.new!(label: "P8", value: 8),
             Objects.DataPoint.new!(label: "P9", value: 9),
             Objects.DataPoint.new!(label: "P10", value: 10),
             Objects.DataPoint.new!(label: "P11", value: 11),
             Objects.DataPoint.new!(label: "P12", value: 12),
             Objects.DataPoint.new!(label: "P13", value: 13),
             Objects.DataPoint.new!(label: "P14", value: 14),
             Objects.DataPoint.new!(label: "P15", value: 15),
             Objects.DataPoint.new!(label: "P16", value: 16),
             Objects.DataPoint.new!(label: "P17", value: 17),
             Objects.DataPoint.new!(label: "P18", value: 18),
             Objects.DataPoint.new!(label: "P19", value: 19),
             Objects.DataPoint.new!(label: "P20", value: 20)
           ]
         )
       end},
      {"data-point-label-too-long",
       fn -> Objects.DataPoint.new!(label: "xxxxxxxxxxxxxxxxxxxxx", value: 1) end},
      {"axis-categories-empty", fn -> Objects.AxisConfig.new!(categories: []) end},
      {"axis-too-many-categories",
       fn ->
         Objects.AxisConfig.new!(
           categories: [
             "X0",
             "X1",
             "X2",
             "X3",
             "X4",
             "X5",
             "X6",
             "X7",
             "X8",
             "X9",
             "X10",
             "X11",
             "X12",
             "X13",
             "X14",
             "X15",
             "X16",
             "X17",
             "X18",
             "X19",
             "X20"
           ]
         )
       end},
      {"axis-category-label-too-long",
       fn -> Objects.AxisConfig.new!(categories: ["xxxxxxxxxxxxxxxxxxxxx"]) end},
      {"axis-label-too-long",
       fn ->
         Objects.AxisConfig.new!(
           categories: ["A"],
           x_label: String.duplicate("x", 51)
         )
       end},
      {"plain-text-input-min-length-negative",
       fn -> Elements.PlainTextInput.new!(action_id: "a", min_length: -1) end},
      {"plain-text-input-min-length-too-large",
       fn -> Elements.PlainTextInput.new!(action_id: "a", min_length: 3_001) end},
      {"plain-text-input-max-length-too-small",
       fn -> Elements.PlainTextInput.new!(action_id: "a", max_length: 0) end},
      {"rich-text-input-min-lines-too-small",
       fn -> Elements.RichTextInput.new!(action_id: "a", min_lines: 0) end},
      {"rich-text-input-min-lines-too-large",
       fn -> Elements.RichTextInput.new!(action_id: "a", min_lines: 101) end},
      {"rich-text-input-max-lines-too-small",
       fn -> Elements.RichTextInput.new!(action_id: "a", max_lines: 0) end},
      {"rich-text-input-max-lines-too-large",
       fn -> Elements.RichTextInput.new!(action_id: "a", max_lines: 101) end},
      {"multi-select-max-selected-items-too-small",
       fn -> Elements.UserMultiSelect.new!(action_id: "a", max_selected_items: 0) end},
      {"conversation-filter-include-empty",
       fn -> Objects.ConversationFilter.new!(include: []) end},
      {"conversation-filter-unknown-include",
       fn -> Objects.ConversationFilter.new!(include: ["channel"]) end},
      {"workflow-button-missing-action-id",
       fn ->
         Slackblocks.from_map!(Elements.WorkflowButton, %{
           "type" => "workflow_button",
           "text" => %{"type" => "plain_text", "text" => "Run"},
           "workflow" => %{"trigger" => %{"url" => "https://slack.com/shortcuts/Ft0/abc"}}
         })
       end},
      {"number-input-missing-decimal-flag",
       fn ->
         Slackblocks.from_map!(Elements.NumberInput, %{
           "type" => "number_input",
           "action_id" => "a"
         })
       end},
      {"slack-icon-missing-name",
       fn -> Slackblocks.from_map!(Objects.SlackIcon, %{"type" => "icon"}) end},
      {"slack-file-id-malformed", fn -> Objects.SlackFile.new!(id: "F0123456") end},
      {"image-element-url-too-long",
       fn ->
         Elements.Image.new!(image_url: String.duplicate("x", 3_001), alt_text: "Alt")
       end},
      {"image-element-alt-text-too-long",
       fn ->
         Elements.Image.new!(
           image_url: "https://example.com/image.png",
           alt_text: String.duplicate("x", 2_001)
         )
       end},
      {"image-block-title-too-long",
       fn ->
         Blocks.Image.new!(
           image_url: "https://example.com/image.png",
           alt_text: "Alt",
           title: Objects.PlainText.new!(text: String.duplicate("x", 2_001))
         )
       end},
      {"image-block-url-and-slack-file",
       fn ->
         Blocks.Image.new!(
           image_url: "https://example.com/image.png",
           slack_file: Objects.SlackFile.new!(id: "F0123ABC456"),
           alt_text: "Alt"
         )
       end},
      {"image-block-missing-source", fn -> Blocks.Image.new!(alt_text: "Alt") end},
      {"container-no-child-blocks",
       fn ->
         Blocks.Container.new!(
           title: Objects.PlainText.new!(text: "Container"),
           child_blocks: []
         )
       end},
      {"table-too-many-column-settings",
       fn ->
         Blocks.Table.new!(
           rows: [[Objects.RawText.new!(text: "A")]],
           column_settings: List.duplicate(Objects.ColumnSettings.new!(is_wrapped: true), 21)
         )
       end},
      {"data-table-row-header-index-negative",
       fn ->
         Blocks.DataTable.new!(
           rows: [
             [Objects.RawText.new!(text: "Name")],
             [Objects.RawText.new!(text: "Alice")]
           ],
           caption: "Names",
           row_header_column_index: -1
         )
       end},
      {"data-table-total-content-too-long",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.DataTable.new!(
               rows: [
                 [Objects.RawText.new!(text: "Name")],
                 [Objects.RawText.new!(text: String.duplicate("x", 9_997))]
               ],
               caption: "Names"
             ),
             Blocks.DataTable.new!(
               rows: [
                 [Objects.RawText.new!(text: "Name")],
                 [Objects.RawText.new!(text: String.duplicate("x", 9_997))]
               ],
               caption: "Names"
             )
           ]
         )
       end},
      {"markdown-total-too-long",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.Markdown.new!(text: String.duplicate("x", 6_001)),
             Blocks.Markdown.new!(text: String.duplicate("x", 6_001))
           ]
         )
       end},
      {"plan-missing-tasks",
       fn ->
         Slackblocks.from_map!(Blocks.Plan, %{"type" => "plan", "title" => "Plan"})
       end},
      {"plan-too-many-tasks",
       fn ->
         Blocks.Plan.new!(
           title: "Plan",
           tasks: [
             Blocks.TaskCard.new!(
               task_id: "task_0",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_1",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_2",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_3",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_4",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_5",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_6",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_7",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_8",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_9",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_10",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_11",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_12",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_13",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_14",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_15",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_16",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_17",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_18",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_19",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_20",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_21",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_22",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_23",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_24",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_25",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_26",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_27",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_28",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_29",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_30",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_31",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_32",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_33",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_34",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_35",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_36",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_37",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_38",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_39",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_40",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_41",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_42",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_43",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_44",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_45",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_46",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_47",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_48",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_49",
               title: "Task",
               status: :complete
             ),
             Blocks.TaskCard.new!(
               task_id: "task_50",
               title: "Task",
               status: :complete
             )
           ]
         )
       end},
      {"plan-duplicate-task-ids",
       fn ->
         Blocks.Plan.new!(
           title: "Plan",
           tasks: [
             Blocks.TaskCard.new!(task_id: "task", title: "Task", status: :complete),
             Blocks.TaskCard.new!(task_id: "task", title: "Task", status: :complete)
           ]
         )
       end},
      {"task-card-missing-status",
       fn ->
         Slackblocks.from_map!(Blocks.TaskCard, %{
           "type" => "task_card",
           "task_id" => "task",
           "title" => "Task"
         })
       end},
      {"task-card-pending-status",
       fn ->
         Slackblocks.Message.new!(
           channel: "C123",
           blocks: [
             Blocks.TaskCard.new!(task_id: "task", title: "Task", status: :pending)
           ]
         )
       end},
      {"rich-text-list-indent-negative",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Item")])
           ],
           indent: -1
         )
       end},
      {"rich-text-list-indent-too-large",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Item")])
           ],
           indent: 9
         )
       end},
      {"rich-text-list-offset-negative",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Item")])
           ],
           offset: -1
         )
       end},
      {"rich-text-list-border-negative",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Item")])
           ],
           border: -1
         )
       end},
      {"rich-text-list-border-too-large",
       fn ->
         RichText.List.new!(
           style: :bullet,
           elements: [
             RichText.Section.new!(elements: [RichText.Text.new!(text: "Item")])
           ],
           border: 2
         )
       end},
      {"rich-text-quote-border-negative",
       fn ->
         RichText.Quote.new!(
           elements: [RichText.Text.new!(text: "Text")],
           border: -1
         )
       end},
      {"rich-text-quote-border-too-large",
       fn ->
         RichText.Quote.new!(
           elements: [RichText.Text.new!(text: "Text")],
           border: 2
         )
       end},
      {"rich-text-preformatted-border-negative",
       fn ->
         RichText.CodeBlock.new!(
           elements: [RichText.Text.new!(text: "Text")],
           border: -1
         )
       end},
      {"rich-text-preformatted-border-too-large",
       fn ->
         RichText.CodeBlock.new!(
           elements: [RichText.Text.new!(text: "Text")],
           border: 2
         )
       end},
      {"video-thumbnail-url-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: String.duplicate("x", 3_001),
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4"
         )
       end},
      {"video-url-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: String.duplicate("x", 3_001)
         )
       end},
      {"video-title-url-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4",
           title_url: String.duplicate("x", 2_001)
         )
       end},
      {"video-provider-icon-url-too-long",
       fn ->
         Blocks.Video.new!(
           alt_text: "Video",
           thumbnail_url: "https://example.com/thumbnail.png",
           title: Objects.PlainText.new!(text: "Title"),
           video_url: "https://example.com/video.mp4",
           provider_icon_url: String.duplicate("x", 2_001)
         )
       end},
      {"view-external-id-too-long",
       fn ->
         Views.Home.new!(
           blocks: [Blocks.Divider.new!([])],
           external_id: String.duplicate("x", 256)
         )
       end},
      {"number-bound-inverted-large-integer",
       fn ->
         Elements.NumberInput.new!(
           is_decimal_allowed: true,
           min_value: "9007199254740993",
           max_value: "9007199254740992"
         )
       end},
      {"number-bound-inverted-long-fraction",
       fn ->
         Elements.NumberInput.new!(
           is_decimal_allowed: true,
           min_value: "0.10000000000000001",
           max_value: "0.1"
         )
       end},
      {"number-bound-hex",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: "0x10") end},
      {"number-bound-underscore",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: "1_000") end},
      {"number-bound-leading-space",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: " 1") end},
      {"number-bound-trailing-space",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: "1 ") end},
      {"number-bound-nan",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: "NaN") end},
      {"number-bound-infinity",
       fn -> Elements.NumberInput.new!(is_decimal_allowed: true, min_value: "Infinity") end},
      {"external-multi-focus-false",
       fn -> Elements.ExternalMultiSelect.new!(focus_on_load: false) end},
      {"external-multi-focus-true",
       fn -> Elements.ExternalMultiSelect.new!(focus_on_load: true) end},
      {"attachment-missing-blocks",
       fn -> Slackblocks.from_map!(Slackblocks.Attachment, %{"fallback" => "Summary"}) end}
    ]
  end
end
