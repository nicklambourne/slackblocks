import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeReferencePages } from "./reference_pages.mjs";

const directory = path.dirname(fileURLToPath(import.meta.url));
const reference = JSON.parse(execFileSync("php", [path.join(directory, "php_reference_adapter.php")], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 }));
const domains = [
  { slug: "blocks", title: "Blocks", position: 1, package: "block" },
  { slug: "elements", title: "Elements", position: 2, package: "element" },
  { slug: "objects", title: "Composition Objects", position: 3, package: "object" },
  { slug: "payloads", title: "Payloads and Views", position: 4, package: "payload" },
  { slug: "components", title: "Components and Utilities", position: 5, package: "component" },
  { slug: "core", title: "Core Types", position: 6, package: "core" },
  { slug: "errors", title: "Validation and Errors", position: 7, package: "error" },
];
const count = await writeReferencePages({
  reference, domains,
  domainFor: (type) => domains.find((domain) => domain.package === type.package),
  language: "php", fence: "php", preserveMemberNames: true,
  outputRoot: path.resolve(directory, "../docs/reference/php"),
  introduction: () => "Generated from actual PHP reflection and PHPDoc. Names live in the `Slackblocks` namespace. Use named arguments and `declare(strict_types=1)`; values validate before construction returns.\n\n",
  index: `---\nsidebar_position: 0\n---\n\n# PHP API reference\n\nPHP 8.2+ on 64-bit systems, with readonly classes, named arguments and backed enums. The first release is being prepared for 2.7.0; see [installation](/usage/installation). Core serialization has no HTTP or framework dependencies.\n\n${domains.map((d) => `- [${d.title}](/reference/php/${d.slug})`).join("\n")}\n`,
});
console.log(`Generated ${count} PHP public types from reflection and PHPDoc.`);
