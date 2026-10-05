//! Handwritten contextual rules. Typed values remain the public representation;
//! aggregate traversal borrows modeled children and keeps arbitrary JSON opaque.
use crate::generated::{HOME_BLOCKS, ICONS, LIMITS, MESSAGE_BLOCKS, MODAL_BLOCKS};
use crate::{
    Attachment, Block, DataTableBlock, DataTableCell, ErrorCategory as C, RichTextBlockElement,
    RichTextSectionElement, RichTextStyle, TaskCardBlock, TaskStatus, Text, ValidationError,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_json::Value;

fn fail(category: C, path: &str, message: &str) -> ValidationError {
    ValidationError::new(category, path, message)
}
fn limit(key: &str) -> i64 {
    LIMITS
        .iter()
        .find(|(k, _)| *k == key)
        .expect("generated limit key exists")
        .1
}
fn chars(v: &Value) -> usize {
    v.as_str().map_or(0, |s| s.chars().count())
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map_or(&[], Vec::as_slice)
}
fn wire_length(v: &Value, key: &str, path: &str) -> Result<(), ValidationError> {
    if chars(v) > limit(key) as usize {
        return Err(fail(
            C::LengthExceeded,
            path,
            "text exceeds the field limit",
        ));
    }
    Ok(())
}
pub(crate) fn length(
    len: usize,
    min: Option<usize>,
    max: Option<usize>,
    path: &str,
) -> Result<(), ValidationError> {
    if min.is_some_and(|min| len < min) || max.is_some_and(|max| len > max) {
        return Err(fail(C::LengthExceeded, path, "length outside field bounds"));
    }
    Ok(())
}
pub(crate) fn text_len(text: &Text) -> usize {
    match text {
        Text::Plain(v) => v.text().chars().count(),
        Text::Markdown(v) => v.text().chars().count(),
    }
}
pub(crate) fn style(
    value: &RichTextStyle,
    allowed: &[&str],
    path: &str,
) -> Result<(), ValidationError> {
    for (key, value) in [
        ("bold", value.is_bold()),
        ("italic", value.is_italic()),
        ("strike", value.is_strike()),
        ("code", value.is_code()),
        ("highlight", value.is_highlight()),
        ("client_highlight", value.is_client_highlight()),
        ("unlink", value.is_unlink()),
    ] {
        if value.is_some() && !allowed.contains(&key) {
            return Err(fail(
                C::TypeMismatch,
                &format!("{path}.{key}"),
                "style is not allowed on this element",
            ));
        }
    }
    Ok(())
}
pub(crate) fn coerce_text(value: Option<&mut Value>, kind: &str) {
    if let Some(v) = value {
        if let Some(text) = v.as_str() {
            *v = serde_json::json!({"type":kind,"text":text});
        }
    }
}
pub(crate) fn restore_tasks(value: Option<&mut Value>) {
    if let Some(Value::Array(tasks)) = value {
        for task in tasks {
            if let Value::Object(task) = task {
                task.entry("type")
                    .or_insert_with(|| Value::String("task_card".into()));
            }
        }
    }
}
struct PlanTask<'a>(&'a TaskCardBlock);
impl Serialize for PlanTask<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize_value(serializer, false)
    }
}
pub(crate) struct PlanTasks<'a>(pub(crate) &'a [TaskCardBlock]);
impl Serialize for PlanTasks<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for task in self.0 {
            seq.serialize_element(&PlanTask(task))?;
        }
        seq.end()
    }
}

pub(crate) fn validate(kind: &str, v: &Value, path: &str) -> Result<(), ValidationError> {
    let has = |key| v.get(key).is_some();
    let at = |key| format!("{path}.{key}");
    match kind {
        "SlackIcon" => {
            if !ICONS.contains(&v["name"].as_str().unwrap_or("")) {
                return Err(fail(C::TypeMismatch, &at("name"), "unknown Slack icon"));
            }
        }
        "SectionBlock" => {
            if !has("text") && array(&v["fields"]).is_empty() {
                return Err(fail(C::MissingRequired, path, "expected text or fields"));
            }
            if has("fields") && array(&v["fields"]).is_empty() {
                return Err(fail(
                    C::MissingRequired,
                    &at("fields"),
                    "fields cannot be empty",
                ));
            }
        }
        "StaticSelectElement" | "StaticMultiSelectElement" => {
            if has("options") && has("option_groups") {
                return Err(fail(
                    C::MutuallyExclusive,
                    path,
                    "options and option_groups cannot coexist",
                ));
            }
        }
        "CardBlock" => {
            if !["hero_image", "title", "actions", "body"]
                .iter()
                .any(|k| has(k))
            {
                return Err(fail(C::MissingRequired, path, "card needs content"));
            }
            if has("icon") && has("slack_icon") {
                return Err(fail(
                    C::MutuallyExclusive,
                    path,
                    "image and Slack icons cannot coexist",
                ));
            }
        }
        "ContainerBlock" => {
            if !has("title") && !has("rich_text_title") {
                return Err(fail(C::MissingRequired, path, "expected a title"));
            }
            if v["default_collapsed"] == true && v["is_collapsible"] != true {
                return Err(fail(
                    C::InvalidUsage,
                    &at("default_collapsed"),
                    "requires is_collapsible",
                ));
            }
            if v["has_header_divider"] == true && v["is_collapsible"] == true {
                return Err(fail(
                    C::InvalidUsage,
                    &at("has_header_divider"),
                    "requires a non-collapsible container",
                ));
            }
        }
        "ImageBlock" | "ImageElement" => {
            if !has("image_url") && !has("slack_file") {
                return Err(fail(C::MissingRequired, path, "expected an image source"));
            }
            if has("image_url") && has("slack_file") {
                return Err(fail(
                    C::MutuallyExclusive,
                    path,
                    "expected one image source",
                ));
            }
        }
        "SlackFile" => {
            if has("id") == has("url") {
                return Err(fail(
                    C::MutuallyExclusive,
                    path,
                    "expected exactly one of id or url",
                ));
            }
            if let Some(id) = v["id"].as_str() {
                if id.len() < 9
                    || !id.starts_with('F')
                    || !id
                        .bytes()
                        .skip(1)
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                {
                    return Err(fail(C::TypeMismatch, &at("id"), "invalid Slack file ID"));
                }
            }
        }
        "ConversationFilter" => {
            if ![
                "include",
                "exclude_external_shared_channels",
                "exclude_bot_users",
            ]
            .iter()
            .any(|k| has(k))
            {
                return Err(fail(C::MissingRequired, path, "expected a filter field"));
            }
            for (i, kind) in array(&v["include"]).iter().enumerate() {
                if !["im", "mpim", "private", "public"].contains(&kind.as_str().unwrap_or("")) {
                    return Err(fail(
                        C::TypeMismatch,
                        &format!("{path}.include[{i}]"),
                        "unknown conversation kind",
                    ));
                }
            }
        }
        "DispatchActionConfiguration" => {
            if !has("trigger_actions_on") {
                return Err(fail(
                    C::MissingRequired,
                    &at("trigger_actions_on"),
                    "required field is missing",
                ));
            }
        }
        "NumberInputElement" => {
            if let (Some(min), Some(max)) = (v["min_value"].as_f64(), v["max_value"].as_f64()) {
                if min > max {
                    return Err(fail(C::OutOfRange, path, "minimum exceeds maximum"));
                }
            }
        }
        "TableBlock" => table(v, path)?,
        "AxisConfig" => {
            let categories = array(&v["categories"]);
            for (i, cat) in categories.iter().enumerate() {
                wire_length(
                    cat,
                    "data_visualization.category_label.max_length",
                    &format!("{path}.categories[{i}]"),
                )?;
                if categories[..i].contains(cat) {
                    return Err(fail(
                        C::InvalidUsage,
                        &at("categories"),
                        "duplicate category",
                    ));
                }
            }
        }
        "ChartSegment" => {
            if v["value"].as_f64().is_some_and(|n| {
                n <= limit("data_visualization.segment.value.exclusive_min") as f64
            }) {
                return Err(fail(
                    C::OutOfRange,
                    &at("value"),
                    "expected a positive segment value",
                ));
            }
        }
        "LineChart" | "BarChart" | "AreaChart" => chart(v, path)?,
        "PlanBlock" => {
            let tasks = array(&v["tasks"]);
            for (i, task) in tasks.iter().enumerate() {
                if tasks[..i].iter().any(|t| t["task_id"] == task["task_id"]) {
                    return Err(fail(C::InvalidUsage, &at("tasks"), "duplicate task ID"));
                }
            }
        }
        _ => {}
    }
    Ok(())
}
pub(crate) fn attachment(value: &Attachment, path: &str) -> Result<(), ValidationError> {
    if let Some(color) = value.color() {
        let hex = color
            .strip_prefix('#')
            .is_some_and(|s| s.len() == 6 && s.bytes().all(|c| c.is_ascii_hexdigit()));
        if !hex && !["good", "warning", "danger"].contains(&color) {
            return Err(fail(
                C::TypeMismatch,
                &format!("{path}.color"),
                "expected a semantic color or six-digit hex color",
            ));
        }
    }
    surface(value.blocks(), "message", &format!("{path}.blocks"))
}

pub(crate) fn message(
    blocks: &[Block],
    attachments: &[Attachment],
    path: &str,
) -> Result<(), ValidationError> {
    surface(blocks, "message", &format!("{path}.blocks"))?;
    let (markdown, data_table) = blocks
        .iter()
        .chain(attachments.iter().flat_map(Attachment::blocks))
        .map(totals)
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    if markdown > limit("markdown.total_text.max_length") as usize {
        return Err(fail(
            C::LengthExceeded,
            path,
            "message markdown total exceeded",
        ));
    }
    if data_table > limit("data_table.total_content.max_length") as usize {
        return Err(fail(
            C::LengthExceeded,
            path,
            "message table total exceeded",
        ));
    }
    Ok(())
}

pub(crate) fn view(
    blocks: &[Block],
    name: &str,
    has_submit: bool,
    path: &str,
) -> Result<(), ValidationError> {
    surface(blocks, name, &format!("{path}.blocks"))?;
    if name == "modal" && !has_submit && blocks.iter().any(|b| matches!(b, Block::Input(_))) {
        return Err(fail(
            C::MissingRequired,
            &format!("{path}.submit"),
            "modal inputs require a submit label",
        ));
    }
    Ok(())
}

fn surface(blocks: &[Block], name: &str, path: &str) -> Result<(), ValidationError> {
    let allowed = match name {
        "modal" => MODAL_BLOCKS,
        "home" => HOME_BLOCKS,
        _ => MESSAGE_BLOCKS,
    };
    for (i, block) in blocks.iter().enumerate() {
        let path = format!("{path}[{i}]");
        if !allowed.contains(&block.wire_type()) {
            return Err(fail(
                C::TypeMismatch,
                &format!("{path}.type"),
                "unsupported block on this surface",
            ));
        }
        standalone_task(block, &path)?;
    }
    Ok(())
}

fn standalone_task(block: &Block, path: &str) -> Result<(), ValidationError> {
    match block {
        Block::TaskCard(task) if task.status() == TaskStatus::Pending => {
            return Err(fail(
                C::TypeMismatch,
                &format!("{path}.status"),
                "pending tasks are plan-only",
            ));
        }
        Block::Container(container) => {
            for (i, child) in container.child_blocks().iter().enumerate() {
                standalone_task(child, &format!("{path}.child_blocks[{i}]"))?;
            }
        }
        // Plan entries may be pending. Other block types cannot contain task cards.
        _ => {}
    }
    Ok(())
}

fn totals(block: &Block) -> (usize, usize) {
    match block {
        Block::Markdown(value) => (value.text().chars().count(), 0),
        Block::DataTable(value) => (0, table_characters(value.rows())),
        Block::Container(value) => value
            .child_blocks()
            .iter()
            .map(totals)
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d)),
        _ => (0, 0),
    }
}

fn table_characters(rows: &[Vec<DataTableCell>]) -> usize {
    rows.iter()
        .flatten()
        .map(|cell| match cell {
            DataTableCell::RawText(v) => v.text().chars().count(),
            DataTableCell::RawNumber(v) => v.text().chars().count(),
            DataTableCell::RichText(v) => v
                .elements()
                .iter()
                .map(|element| match element {
                    RichTextBlockElement::Section(v) => inline_characters(v.elements()),
                    RichTextBlockElement::Quote(v) => inline_characters(v.elements()),
                    RichTextBlockElement::CodeBlock(v) => inline_characters(v.elements()),
                    RichTextBlockElement::List(v) => v
                        .elements()
                        .iter()
                        .map(|v| inline_characters(v.elements()))
                        .sum(),
                })
                .sum(),
        })
        .sum()
}

fn inline_characters(elements: &[RichTextSectionElement]) -> usize {
    elements
        .iter()
        .map(|element| match element {
            RichTextSectionElement::Text(v) => v.text().chars().count(),
            RichTextSectionElement::Link(v) => v.text().unwrap_or_default().chars().count(),
            _ => 0,
        })
        .sum()
}

pub(crate) fn data_table(value: &DataTableBlock, path: &str) -> Result<(), ValidationError> {
    let rows = value.rows();
    for (i, cells) in rows.iter().enumerate() {
        let p = format!("{path}.rows[{i}]");
        if cells.is_empty() || cells.len() > limit("data_table.columns.max_items") as usize {
            return Err(fail(C::LengthExceeded, &p, "invalid column count"));
        }
        if cells.len() != rows[0].len() {
            return Err(fail(
                C::InvalidUsage,
                &p,
                "data table rows must be rectangular",
            ));
        }
        for (j, cell) in cells.iter().enumerate() {
            let p = format!("{p}[{j}]");
            match cell {
                DataTableCell::RichText(_) if i == 0 => {
                    return Err(fail(C::TypeMismatch, &p, "header cells must be raw"));
                }
                DataTableCell::RawText(v) if v.text().is_empty() => {
                    return Err(fail(
                        C::LengthExceeded,
                        &format!("{p}.text"),
                        "empty data table cell",
                    ));
                }
                DataTableCell::RawNumber(v) if v.text().is_empty() => {
                    return Err(fail(
                        C::LengthExceeded,
                        &format!("{p}.text"),
                        "empty data table cell",
                    ));
                }
                _ => {}
            }
        }
    }
    if value.extensions().contains_key("column_settings") {
        return Err(fail(
            C::InvalidUsage,
            &format!("{path}.column_settings"),
            "unsupported data table field",
        ));
    }
    if table_characters(rows) > limit("data_table.content.max_length") as usize {
        return Err(fail(
            C::LengthExceeded,
            &format!("{path}.rows"),
            "table text total exceeded",
        ));
    }
    Ok(())
}

fn table(v: &Value, path: &str) -> Result<(), ValidationError> {
    for (i, row) in array(&v["rows"]).iter().enumerate() {
        if array(row).len() > limit("table.columns.max_items") as usize {
            return Err(fail(
                C::LengthExceeded,
                &format!("{path}.rows[{i}]"),
                "invalid column count",
            ));
        }
    }
    Ok(())
}
fn chart(v: &Value, path: &str) -> Result<(), ValidationError> {
    let axis = array(&v["axis_config"]["categories"]);
    let series = array(&v["series"]);
    for (i, series_item) in series.iter().enumerate() {
        let points = array(&series_item["data"]);
        if points.len() != axis.len()
            || axis
                .iter()
                .any(|label| points.iter().filter(|p| &p["label"] == label).count() != 1)
        {
            return Err(fail(
                C::InvalidUsage,
                &format!("{path}.series[{i}].data"),
                "points must cover each category exactly once",
            ));
        }
        if series[..i].iter().any(|s| s["name"] == series_item["name"]) {
            return Err(fail(
                C::InvalidUsage,
                &format!("{path}.series"),
                "duplicate series name",
            ));
        }
    }
    Ok(())
}
