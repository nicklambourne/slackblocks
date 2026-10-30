import React from "react";
import CodeBlock from "@theme/CodeBlock";
import { useLanguage } from "../LanguageContext";
import styles from "./styles.module.css";

type Json = string | number | boolean | null | Json[] | { [key: string]: Json };
const object = (value: Json | undefined): Record<string, Json> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? value : {};
const list = (value: Json | undefined): Json[] => Array.isArray(value) ? value : [];
const text = (value: Json | undefined): string => {
  const content = typeof value === "string" ? value : object(value).text;
  return typeof content === "string" ? content : "";
};

function Inline({ value }: { value: Json | undefined }) {
  const content = text(value).replace(/:white_check_mark:/g, "✅").replace(/:warning:/g, "⚠️").replace(/:hand:/g, "✋").replace(/:calendar:/g, "📅").replace(/:mag:/g, "🔎");
  return <>{content.split(/(\*[^*]+\*|`[^`]+`)/g).map((part, index) =>
    part.startsWith("*") ? <strong key={index}>{part.slice(1, -1)}</strong> :
      part.startsWith("`") ? <code key={index}>{part.slice(1, -1)}</code> : part)}</>;
}

function Rich({ value }: { value: Json }) {
  const item = object(value);
  if (Array.isArray(item.elements)) return <>{item.elements.map((child, index) => <Rich key={index} value={child} />)}</>;
  let result: React.ReactNode = text(item.text);
  const style = object(item.style);
  if (style.bold) result = <strong>{result}</strong>;
  if (style.italic) result = <em>{result}</em>;
  if (style.code) result = <code>{result}</code>;
  if (item.type === "link") result = <a href={text(item.url)}>{result || text(item.url)}</a>;
  return <>{result}</>;
}

function Blocks({ values }: { values: Json[] }) {
  return <>{values.map((value, index) => <Block key={index} value={value} />)}</>;
}

function Block({ value }: { value: Json }) {
  const block = object(value);
  switch (block.type) {
    case "header": return <h3 className={styles.header}><Inline value={block.text} /></h3>;
    case "divider": return <hr />;
    case "section": return <div className={styles.section}>
      {block.text && <div><Inline value={block.text} /></div>}
      {block.fields && <div className={styles.fields}>{list(block.fields).map((field, index) => <div key={index}><Inline value={field} /></div>)}</div>}
    </div>;
    case "context": return <div className={styles.context}>{list(block.elements).map((element, index) => <span key={index}><Inline value={element} /></span>)}</div>;
    case "actions": return <div className={styles.actions}>{list(block.elements).map((element, index) => <Block key={index} value={element} />)}</div>;
    case "button":
    case "workflow_button": return <button type="button" disabled className={styles.button} data-style={text(block.style)}><Inline value={block.text} /></button>;
    case "rich_text": return <div className={styles.section}><Rich value={value} /></div>;
    case "alert": return <div className={styles.alert}><Inline value={block.text} /></div>;
    case "input": return <label className={styles.input}><Inline value={block.label} /><input disabled placeholder={text(object(block.element).placeholder)} /></label>;
    case "table": return <table className={styles.table}><tbody>{list(block.rows).map((row, index) => <tr key={index}>{list(row).map((cell, column) => <td key={column}>{object(cell).type === "raw_text" ? text(cell) : <Rich value={cell} />}</td>)}</tr>)}</tbody></table>;
    case "layout": return <div className={styles.layout}>
      {block.is_label_visible === true && <div className={styles.layoutLabel}><strong>{text(block.label)}</strong></div>}
      <div className={styles.layoutGrid} data-grid={text(block.grid_lines) || "none"}>
        {list(block.rows).map((row, rowIndex) => {
          const items = list(object(row).items);
          const columns = items.map((item) => object(item).width === "narrow" ? "1fr" : object(item).width === "wide" ? "3fr" : "2fr").join(" ");
          return <div key={rowIndex} className={styles.layoutRow} style={{ "--layout-columns": columns } as React.CSSProperties}>
            {items.map((item, index) => {
              const cell = object(item);
              const alignment = text(cell.horizontal_alignment);
              return <div key={index} className={styles.layoutCell} data-alignment={alignment || "fill"}>
                <div><Block value={cell.block} /></div>
              </div>;
            })}
          </div>;
        })}
      </div>
      {block.fallback !== undefined && <details className={styles.layoutFallback}><summary>Linear fallback</summary>
        {list(block.fallback).length ? <Blocks values={list(block.fallback)} /> : <p>No fallback content.</p>}
      </details>}
    </div>;
    case "container": return <details className={styles.container} open={!block.default_collapsed}><summary><Inline value={block.title} /></summary><Blocks values={list(block.child_blocks)} /></details>;
    default: throw new Error(`Recipe preview has no renderer for ${String(block.type)}`);
  }
}

function Message({ value }: { value: Json }) {
  const payload = object(value);
  if (payload.type && payload.type !== "modal") return <Block value={value} />;
  const modal = payload.type === "modal";
  return <div className={modal ? styles.modal : undefined}>
    {modal && <h3 className={styles.header}><Inline value={payload.title} /></h3>}
    {payload.thread_ts && <div className={styles.context}>Reply in thread · {text(payload.thread_ts)}</div>}
    {payload.response_type === "ephemeral" && <div className={styles.context}>Only visible to you</div>}
    {!payload.blocks && payload.text && <div><Inline value={payload.text} /></div>}
    <Blocks values={list(payload.blocks)} />
    {list(payload.attachments).map((attachment, index) => {
      const item = object(attachment);
      const color = text(item.color);
      return <div key={index} className={styles.attachment} style={{ borderColor: ({good: "#16845b", warning: "#d49b18", danger: "#c9374c"} as Record<string, string>)[color] ?? color }}><Blocks values={list(item.blocks)} /></div>;
    })}
    {modal && <div className={styles.actions}>{payload.close && <button disabled className={styles.button}><Inline value={payload.close} /></button>}<button disabled className={styles.button} data-style="primary"><Inline value={payload.submit ?? "Close"} /></button></div>}
  </div>;
}

export default function RecipeOutput({ recipe, outputs }: { recipe: string; outputs: Record<string, Json> }) {
  const { language } = useLanguage();
  const value = outputs[language];
  if (value === undefined) throw new Error(`Missing ${language} output for ${recipe}`);
  const preview = recipe === "builder-preview" ? JSON.parse(decodeURIComponent(new URL(String(value)).hash.slice(1))) as Json : value;
  return <section aria-label="Recipe output" className={styles.output}>
    <h2>Output</h2>
    <p className={styles.caption}>Local preview of the selected language’s tested example. Slack client appearance may differ; controls here do not send requests.</p>
    <div className={styles.preview}>
      {recipe === "validation-errors" ? <div role="status" className={styles.alert}><strong>Validation handled</strong><br /><code>length-exceeded</code> · the oversized button label was rejected before sending.</div> : <Message value={preview} />}
      {recipe === "metadata" && <><div className={styles.context}>Metadata accompanies the message and is not shown in Slack.</div><CodeBlock language="json">{JSON.stringify(object(value).metadata, null, 2)}</CodeBlock></>}
    </div>
    <details><summary>{recipe === "builder-preview" ? "Generated Builder URL" : "Serialized output"}</summary><CodeBlock language="json">{JSON.stringify(value, null, 2)}</CodeBlock></details>
  </section>;
}
