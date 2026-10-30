defmodule Slackblocks.Internal.Rules do
  @moduledoc false
  alias Slackblocks.Internal.{Codec, Schema}
  import Slackblocks.Internal.Value, only: [fail: 3]

  def validate(name, wire, path, context) do
    local(name, wire, path)

    if name == "TaskCardBlock" and context not in [:local, :plan] and wire["status"] == "pending",
      do: fail(:type_mismatch, path ++ ["status"], "pending tasks require a plan")

    :ok
  end

  defp local("ExternalMultiSelectElement", w, p) do
    if Map.has_key?(w, "focus_on_load"),
      do:
        fail(
          :invalid_usage,
          p ++ ["focus_on_load"],
          "unsupported for external multi-select menus; omit this field"
        )
  end

  defp local("ButtonElement", w, p) do
    if Map.has_key?(w, "agent_prompt_display") and not Map.has_key?(w, "agent_prompt"),
      do:
        fail(
          :missing_required,
          p ++ ["agent_prompt"],
          "required when agent_prompt_display is supplied"
        )
  end

  defp local("SectionBlock", w, p) do
    if not Map.has_key?(w, "text") and Map.get(w, "fields", []) == [],
      do: fail(:missing_required, p, "expected text or fields")

    if w["fields"] == [], do: fail(:missing_required, p ++ ["fields"], "fields cannot be empty")

    each(w["fields"], fn f, i ->
      max_length(f["text"], "section.fields.item_max_length", p ++ ["fields", i, "text"])
    end)
  end

  defp local(name, w, p) when name in ["TableBlock", "DataTableBlock"],
    do: table(w, p, name == "DataTableBlock")

  defp local("PlanBlock", w, p), do: unique(Enum.map(w["tasks"], & &1["task_id"]), p ++ ["tasks"])

  defp local(name, w, p) when name in ["MessagePayload", "MessageResponse", "WebhookMessage"] do
    blocks = Map.get(w, "blocks", [])
    surface(blocks, "message", p ++ ["blocks"])
    combined = blocks ++ Enum.flat_map(Map.get(w, "attachments", []), &Map.get(&1, "blocks", []))
    max_size(chart_count(combined), "message.data_visualizations.max_items", p)
    {markdown, cells} = totals(combined)
    max_size(markdown, "markdown.total_text.max_length", p)
    max_size(cells, "data_table.total_content.max_length", p)
  end

  defp local("Attachment", w, p) do
    color = w["color"]

    if color != nil and color not in ["good", "warning", "danger"] and
         not Regex.match?(~r/\A#[a-fA-F0-9]{6}\z/, color),
       do: fail(:type_mismatch, p ++ ["color"], "invalid attachment color")

    surface(Map.get(w, "blocks", []), "message", p ++ ["blocks"])
  end

  defp local(name, w, p) when name in ["ModalView", "HomeTabView"] do
    surface(w["blocks"], if(name == "ModalView", do: "modal", else: "home"), p ++ ["blocks"])

    if name == "ModalView" and not Map.has_key?(w, "submit") and
         Enum.any?(descendants(w["blocks"]), &(&1["type"] == "input")),
       do: fail(:missing_required, p ++ ["submit"], "required for a modal containing input")
  end

  defp local("SlackIcon", w, p) do
    unless w["name"] in Schema.vocabulary()["slack_icon_names"],
      do: fail(:type_mismatch, p ++ ["name"], "unknown Slack icon")
  end

  defp local(name, w, p) when name in ["StaticSelectElement", "StaticMultiSelectElement"],
    do: exclusive(w, "options", "option_groups", p)

  defp local("CardBlock", w, p) do
    unless Enum.any?(~w(hero_image title actions body), &Map.has_key?(w, &1)),
      do: fail(:missing_required, p, "card needs content")

    exclusive(w, "icon", "slack_icon", p)
  end

  defp local("LayoutBlock", w, p), do: composite_depth(w, p, 0, false)

  defp local("LayoutRow", w, p) do
    if Map.keys(w) -- ["items"] != [], do: fail(:type_mismatch, p, "a row only supports items")
    items = w["items"]
    wide = Enum.count(items, &(&1["block"]["type"] in ~w(carousel data_table data_visualization)))
    if wide > 1, do: fail(:invalid_usage, p, "at most one wide-content block per row")
    total = Enum.sum(Enum.map(items, &layout_weight/1))

    Enum.each(items, fn item ->
      if item["block"]["type"] == "data_visualization" and layout_weight(item) * 2 <= total,
        do: fail(:invalid_usage, p, "a data visualization requires more than half the row weight")
    end)
  end

  defp local("LayoutItem", w, p) do
    if Map.keys(w) -- ~w(block width horizontal_alignment) != [],
      do: fail(:type_mismatch, p, "unsupported layout item property")
  end

  defp local("ContainerBlock", w, p) do
    composite_depth(w, p, 0, false)

    unless Map.has_key?(w, "title") or Map.has_key?(w, "rich_text_title"),
      do: fail(:missing_required, p, "container needs a title")

    if w["default_collapsed"] == true and w["is_collapsible"] != true,
      do: fail(:invalid_usage, p ++ ["default_collapsed"], "requires is_collapsible")

    if w["has_header_divider"] == true and w["is_collapsible"] == true,
      do:
        fail(:invalid_usage, p ++ ["has_header_divider"], "requires a non-collapsible container")
  end

  defp local(name, w, p) when name in ["ImageBlock", "ImageElement"] do
    unless Map.has_key?(w, "image_url") or Map.has_key?(w, "slack_file"),
      do: fail(:missing_required, p, "expected an image source")

    exclusive(w, "image_url", "slack_file", p)
  end

  defp local("SlackFile", w, p) do
    if Map.has_key?(w, "id") == Map.has_key?(w, "url"),
      do: fail(:mutually_exclusive, p, "expected exactly one source")

    if w["id"] != nil and not Regex.match?(~r/\AF[A-Z0-9]{8,}\z/, w["id"]),
      do: fail(:type_mismatch, p ++ ["id"], "invalid Slack file ID")
  end

  defp local("ConversationFilter", w, p) do
    if map_size(w) == 0, do: fail(:missing_required, p, "expected at least one filter field")

    each(w["include"], fn item, i ->
      unless item in ~w(im mpim private public),
        do: fail(:type_mismatch, p ++ ["include", i], "unknown conversation type")
    end)
  end

  defp local("DispatchActionConfiguration", w, p) do
    unless Map.has_key?(w, "trigger_actions_on"),
      do: fail(:missing_required, p ++ ["trigger_actions_on"], "required field is missing")
  end

  defp local("DatePickerElement", w, p) do
    if w["initial_date"] != nil and
         not Regex.match?(~r/\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/, w["initial_date"]),
       do: fail(:type_mismatch, p ++ ["initial_date"], "expected YYYY-MM-DD")
  end

  defp local("TimePickerElement", w, p) do
    if w["initial_time"] != nil and
         not Regex.match?(~r/\A([01][0-9]|2[0-3]):[0-5][0-9]\z/, w["initial_time"]),
       do: fail(:type_mismatch, p ++ ["initial_time"], "expected HH:mm in 24-hour time")
  end

  defp local("PlainTextInputElement", w, p) do
    if w["min_length"] != nil and w["max_length"] != nil and
         w["min_length"] > w["max_length"],
       do: fail(:out_of_range, p ++ ["min_length"], "cannot exceed max_length")
  end

  defp local("NumberInputElement", w, p) do
    for field <- ["min_value", "max_value"], text = w[field], text != nil do
      unless Regex.match?(
               ~r/\A[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?\z/,
               text
             ),
             do: fail(:type_mismatch, p ++ [field], "expected a decimal string")
    end

    if w["min_value"] != nil and w["max_value"] != nil and
         decimal_greater?(w["min_value"], w["max_value"]),
       do: fail(:out_of_range, p, "minimum exceeds maximum")
  end

  defp local("ChartSegment", w, p),
    do: Codec.limits(w["value"], "data_visualization.segment.value", p ++ ["value"])

  defp local("AxisConfig", w, p) do
    each(w["categories"], fn c, i ->
      Codec.limits(c, "data_visualization.category_label", p ++ ["categories", i])
    end)

    unique(w["categories"], p ++ ["categories"])
  end

  defp local(name, w, p) when name in ["LineChart", "BarChart", "AreaChart"] do
    categories = w["axis_config"]["categories"] |> Enum.sort()
    unique(Enum.map(w["series"], & &1["name"]), p ++ ["series"])

    each(w["series"], fn series, i ->
      labels = Enum.map(series["data"], & &1["label"]) |> Enum.sort()

      if labels != categories,
        do:
          fail(
            :invalid_usage,
            p ++ ["series", i, "data"],
            "points must cover every category exactly once"
          )
    end)
  end

  defp local(_name, _wire, _path), do: :ok

  # Compare coefficients and arbitrary-size exponent ranks, without expanding exponent zeros.
  defp decimal_greater?(left, right) do
    {a, rank_a, x} = decimal_parts(left)
    {b, rank_b, y} = decimal_parts(right)
    width = max(byte_size(x), byte_size(y))
    left_magnitude = {rank_a, String.pad_trailing(x, width, "0")}
    right_magnitude = {rank_b, String.pad_trailing(y, width, "0")}

    cond do
      a != b -> a > b
      a == 0 -> false
      a > 0 -> left_magnitude > right_magnitude
      true -> left_magnitude < right_magnitude
    end
  end

  defp decimal_parts(text) do
    [mantissa | exponent] = Regex.split(~r/[eE]/, text)
    rank = String.to_integer(List.first(exponent) || "0")
    unsigned = String.trim_leading(String.trim_leading(mantissa, "-"), "+")
    [whole | fraction] = String.split(unsigned, ".")
    fraction = List.first(fraction) || ""
    digits = String.trim_leading(whole <> fraction, "0")

    sign =
      cond do
        digits == "" -> 0
        String.starts_with?(mantissa, "-") -> -1
        true -> 1
      end

    {sign, rank + byte_size(digits) - byte_size(fraction), digits}
  end

  defp exclusive(w, a, b, p) do
    if Map.has_key?(w, a) and Map.has_key?(w, b),
      do: fail(:mutually_exclusive, p, "#{a} and #{b} cannot coexist")
  end

  defp unique(values, path) do
    if length(values) != length(Enum.uniq(values)),
      do: fail(:invalid_usage, path, "duplicate value")
  end

  defp each(nil, _fun), do: :ok

  defp each(values, fun),
    do: values |> Enum.with_index() |> Enum.each(fn {v, i} -> fun.(v, i) end)

  defp max_length(nil, _limit, _path), do: :ok
  defp max_length(text, limit, path), do: max_size(Codec.length_codepoints(text), limit, path)

  defp max_size(size, limit, path) do
    if size > Map.fetch!(Schema.limits(), limit),
      do: fail(:length_exceeded, path, "exceeds #{limit}")
  end

  defp layout_weight(%{"width" => "narrow"}), do: 1
  defp layout_weight(%{"width" => "wide"}), do: 3
  defp layout_weight(_), do: 2

  defp composite_children(block, include_fallback \\ true)
  defp composite_children(%{"type" => "container", "child_blocks" => blocks}, _), do: blocks

  defp composite_children(%{"type" => "layout"} = block, include_fallback) do
    Enum.flat_map(block["rows"], fn row -> Enum.map(row["items"], & &1["block"]) end) ++
      if(include_fallback, do: Map.get(block, "fallback", []), else: [])
  end

  defp composite_children(_, _), do: []

  defp composite_depth(block, p, depth, has_layout) do
    has_layout = has_layout or block["type"] == "layout"
    depth = depth + if(block["type"] in ~w(layout container), do: 1, else: 0)

    if has_layout and depth > 2,
      do: fail(:invalid_usage, p, "layout/container nesting exceeds two composite levels")

    Enum.each(composite_children(block, false), &composite_depth(&1, p, depth, has_layout))
  end

  defp descendants(blocks),
    do: Enum.flat_map(blocks, fn b -> [b | descendants(composite_children(b))] end)

  defp block_cost(block, nested \\ false) do
    children = composite_children(block, false)

    cond do
      block["type"] == "layout" ->
        max(
          1 + Enum.sum(Enum.map(children, &block_cost(&1, true))),
          Enum.sum(Enum.map(block["fallback"] || [], &block_cost/1))
        )

      block["type"] == "container" and
          (nested or Enum.any?(descendants(children), &(&1["type"] == "layout"))) ->
        1 + Enum.sum(Enum.map(children, &block_cost(&1, nested)))

      true ->
        1
    end
  end

  defp surface(blocks, surface, path) do
    maximum =
      if surface == "message", do: "message.blocks.max_items", else: "view.blocks.max_items"

    max_size(Enum.sum(Enum.map(blocks || [], &block_cost/1)), maximum, path)
    allowed = Schema.vocabulary()["surface_block_types"][surface]

    each(blocks, fn block, i ->
      surface(composite_children(block, false), surface, path ++ [i, "children"])

      if block["type"] == "layout",
        do: surface(block["fallback"] || [], surface, path ++ [i, "fallback"])

      unless block["type"] in allowed,
        do: fail(:type_mismatch, path ++ [i, "type"], "unsupported block on #{surface} surface")
    end)
  end

  defp table(w, p, data?) do
    each(w["rows"], fn row, i ->
      key = if data?, do: "data_table.columns", else: "table.columns"
      Codec.limits(row, key, p ++ ["rows", i])

      if data?, do: data_row(row, i, length(hd(w["rows"])), p ++ ["rows", i])
    end)

    if data?, do: max_size(table_characters(w), "data_table.content.max_length", p ++ ["rows"])
  end

  defp data_row(row, i, columns, path) do
    if length(row) != columns, do: fail(:invalid_usage, path, "rows must be rectangular")
    each(row, fn cell, j -> data_cell(cell, i, path ++ [j]) end)
  end

  defp data_cell(cell, row, path) do
    if row == 0 and cell["type"] == "rich_text",
      do: fail(:type_mismatch, path, "header cells must be raw")

    if cell["type"] in ~w(raw_text raw_number) and cell["text"] == "",
      do: fail(:length_exceeded, path ++ ["text"], "empty data table cell")
  end

  defp table_characters(w),
    do:
      Enum.reduce(w["rows"], 0, fn row, total ->
        total + Enum.sum(Enum.map(row, &cell_characters/1))
      end)

  defp cell_characters(%{"type" => kind, "text" => text})
       when kind in ["raw_text", "raw_number", "text", "link"],
       do: Codec.length_codepoints(text)

  defp cell_characters(%{"elements" => children}),
    do: Enum.sum(Enum.map(children, &cell_characters/1))

  defp cell_characters(_), do: 0

  defp chart_count(blocks) do
    Enum.reduce(blocks, 0, fn block, count ->
      case block["type"] do
        "data_visualization" -> count + 1
        kind when kind in ~w(container layout) -> count + chart_count(composite_children(block))
        _ -> count
      end
    end)
  end

  defp totals(blocks) do
    Enum.reduce(blocks, {0, 0}, fn block, {markdown, table} ->
      case block["type"] do
        "markdown" ->
          {markdown + Codec.length_codepoints(block["text"]), table}

        "data_table" ->
          {markdown, table + table_characters(block)}

        kind when kind in ~w(container layout) ->
          {m, t} = totals(composite_children(block))
          {markdown + m, table + t}

        _ ->
          {markdown, table}
      end
    end)
  end
end
