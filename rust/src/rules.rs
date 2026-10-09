//! Handwritten contextual rules. Typed values remain the public representation;
//! temporary wire views let aggregate rules follow nested Slack keys uniformly.
use crate::generated::{HOME_BLOCKS, ICONS, LIMITS, MESSAGE_BLOCKS, MODAL_BLOCKS};
use crate::{ErrorCategory as C, TaskCardBlock, ValidationError};
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
fn bound(key: &str) -> Option<i64> {
    LIMITS.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}
fn chars(v: &Value) -> usize {
    v.as_str().map_or(0, |s| s.chars().count())
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map_or(&[], Vec::as_slice)
}
fn length(v: &Value, key: &str, path: &str) -> Result<(), ValidationError> {
    if chars(v) > limit(key) as usize {
        return Err(fail(
            C::LengthExceeded,
            path,
            "text exceeds the field limit",
        ));
    }
    Ok(())
}
fn text_length(v: &Value, key: &str, path: &str) -> Result<(), ValidationError> {
    length(&v["text"], key, path)
}

pub(crate) fn limits(v: &Value, prefix: &str, path: &str) -> Result<(), ValidationError> {
    let (len, suffixes) = if let Some(items) = v.as_array() {
        (Some(items.len()), ["min_items", "max_items"])
    } else {
        (
            v.as_str()
                .or_else(|| v.get("text").and_then(Value::as_str))
                .map(|s| s.chars().count()),
            ["min_length", "max_length"],
        )
    };
    if let Some(len) = len {
        for (i, suffix) in suffixes.iter().enumerate() {
            if let Some(n) = bound(&format!("{prefix}.{suffix}")) {
                if (i == 0 && len < (n as usize)) || (i == 1 && len > (n as usize)) {
                    return Err(fail(
                        C::LengthExceeded,
                        path,
                        &format!("outside {prefix}.{suffix} ({n})"),
                    ));
                }
            }
        }
    }
    if let Some(value) = v.as_f64() {
        for suffix in ["min", "max", "exclusive_min", "exclusive_max"] {
            if let Some(n) = bound(&format!("{prefix}.{suffix}")) {
                let n = n as f64;
                let invalid = match suffix {
                    "min" => value < n,
                    "max" => value > n,
                    "exclusive_min" => value <= n,
                    _ => value >= n,
                };
                if invalid {
                    return Err(fail(
                        C::OutOfRange,
                        path,
                        &format!("outside {prefix}.{suffix}"),
                    ));
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn style(value: &Value, allowed: &[&str], path: &str) -> Result<(), ValidationError> {
    if let Some(map) = value.as_object() {
        for key in map.keys() {
            if !allowed.contains(&key.as_str()) {
                return Err(fail(
                    C::TypeMismatch,
                    &format!("{path}.{key}"),
                    "style is not allowed on this element",
                ));
            }
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
pub(crate) struct PlanTasks<'a>(pub(crate) &'a [TaskCardBlock]);
impl Serialize for PlanTasks<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for task in self.0 {
            let mut value = serde_json::to_value(task).map_err(serde::ser::Error::custom)?;
            if let Some(map) = value.as_object_mut() {
                map.remove("type");
            }
            seq.serialize_element(&value)?;
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
        "Attachment" => {
            if let Some(color) = v["color"].as_str() {
                let hex = color
                    .strip_prefix('#')
                    .is_some_and(|s| s.len() == 6 && s.bytes().all(|c| c.is_ascii_hexdigit()));
                if !hex && !["good", "warning", "danger"].contains(&color) {
                    return Err(fail(
                        C::TypeMismatch,
                        &at("color"),
                        "expected a semantic color or six-digit hex color",
                    ));
                }
            }
            surface(array(&v["blocks"]), "message", &at("blocks"))?;
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
            for (i, field) in array(&v["fields"]).iter().enumerate() {
                text_length(
                    field,
                    "section.fields.item_max_length",
                    &format!("{path}.fields[{i}].text"),
                )?;
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
        "TableBlock" | "DataTableBlock" => table(v, path, kind == "DataTableBlock")?,
        "AxisConfig" => {
            let categories = array(&v["categories"]);
            for (i, cat) in categories.iter().enumerate() {
                length(
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
        "MessagePayload" | "MessageResponse" | "WebhookMessage" => {
            surface(array(&v["blocks"]), "message", &at("blocks"))?;
            let (markdown, data_table) = totals(v);
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
        }
        "ModalView" | "HomeTabView" => {
            let modal = kind == "ModalView";
            surface(
                array(&v["blocks"]),
                if modal { "modal" } else { "home" },
                &at("blocks"),
            )?;
            if modal && !has("submit") && array(&v["blocks"]).iter().any(|b| b["type"] == "input") {
                return Err(fail(
                    C::MissingRequired,
                    &at("submit"),
                    "modal inputs require a submit label",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}
fn surface(blocks: &[Value], name: &str, path: &str) -> Result<(), ValidationError> {
    let allowed = match name {
        "modal" => MODAL_BLOCKS,
        "home" => HOME_BLOCKS,
        _ => MESSAGE_BLOCKS,
    };
    for (i, block) in blocks.iter().enumerate() {
        let path = format!("{path}[{i}]");
        if !allowed.contains(&block["type"].as_str().unwrap_or("")) {
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
fn standalone_task(value: &Value, path: &str) -> Result<(), ValidationError> {
    match value {
        Value::Object(map) => {
            if value["type"] == "task_card" && value["status"] == "pending" {
                return Err(fail(
                    C::TypeMismatch,
                    &format!("{path}.status"),
                    "pending tasks are plan-only",
                ));
            }
            for (key, value) in map {
                if key == "tasks" && map.get("type").and_then(Value::as_str) == Some("plan") {
                    continue;
                }
                standalone_task(value, &format!("{path}.{key}"))?;
            }
        }
        Value::Array(vs) => {
            for (i, v) in vs.iter().enumerate() {
                standalone_task(v, &format!("{path}[{i}]"))?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn characters(value: &Value) -> usize {
    match value {
        Value::Array(vs) => vs.iter().map(characters).sum(),
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| {
                if k == "text" && v.is_string() {
                    chars(v)
                } else {
                    characters(v)
                }
            })
            .sum(),
        _ => 0,
    }
}
fn totals(value: &Value) -> (usize, usize) {
    match value {
        Value::Object(map) if value["type"] == "markdown" => (chars(&map["text"]), 0),
        Value::Object(map) if value["type"] == "data_table" => (0, characters(&map["rows"])),
        Value::Object(map) => sum_totals(map.values()),
        Value::Array(vs) => sum_totals(vs.iter()),
        _ => (0, 0),
    }
}
fn sum_totals<'a>(vs: impl Iterator<Item = &'a Value>) -> (usize, usize) {
    vs.map(totals).fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}
fn table(v: &Value, path: &str, data: bool) -> Result<(), ValidationError> {
    let rows = array(&v["rows"]);
    let max = limit(if data {
        "data_table.columns.max_items"
    } else {
        "table.columns.max_items"
    }) as usize;
    for (i, row) in rows.iter().enumerate() {
        let cells = array(row);
        let p = format!("{path}.rows[{i}]");
        if cells.len() > max || (data && cells.is_empty()) {
            return Err(fail(C::LengthExceeded, &p, "invalid column count"));
        }
        if data && cells.len() != array(&rows[0]).len() {
            return Err(fail(
                C::InvalidUsage,
                &p,
                "data table rows must be rectangular",
            ));
        }
        if data {
            for (j, cell) in cells.iter().enumerate() {
                let p = format!("{p}[{j}]");
                if i == 0 && cell["type"] == "rich_text" {
                    return Err(fail(C::TypeMismatch, &p, "header cells must be raw"));
                }
                if (cell["type"] == "raw_text" || cell["type"] == "raw_number")
                    && cell["text"] == ""
                {
                    return Err(fail(
                        C::LengthExceeded,
                        &format!("{p}.text"),
                        "empty data table cell",
                    ));
                }
            }
        }
    }
    if data && v.get("column_settings").is_some() {
        return Err(fail(
            C::InvalidUsage,
            &format!("{path}.column_settings"),
            "unsupported data table field",
        ));
    }
    if data && characters(&v["rows"]) > limit("data_table.content.max_length") as usize {
        return Err(fail(
            C::LengthExceeded,
            &format!("{path}.rows"),
            "table text total exceeded",
        ));
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
