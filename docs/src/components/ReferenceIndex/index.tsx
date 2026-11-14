import React, { useId, useState } from "react";

type Entry = {name: string; href: string};

export default function ReferenceIndex({entries}: {entries: Entry[]}) {
  const id = useId();
  const [query, setQuery] = useState("");
  const matches = entries.filter(({name}) => name.toLowerCase().includes(query.toLowerCase()));
  return (
    <details className="reference-index">
      <summary>Find a native type or function ({entries.length})</summary>
      <label htmlFor={id}>Filter this page’s API</label>
      <input id={id} type="search" value={query} onChange={event => setQuery(event.target.value)} />
      <p role="status">{matches.length} matching entries</p>
      <ul style={{maxHeight: "20rem", overflow: "auto"}}>
        {matches.map(({name, href}) => <li key={`${name}:${href}`}><a href={href}>{name}</a></li>)}
      </ul>
    </details>
  );
}
