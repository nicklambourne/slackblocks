import { describe, expect, it } from "vitest";
import * as s from "../src/index.js";

const grid = () => s.LayoutBlock().label("Grid").rows(s.LayoutRow().items(s.LayoutItem().block(s.DividerBlock()))).build();

describe("native layout construction", () => {
  it("preserves omission and explicit false/empty, and detaches builder edits", () => {
    const builder = s.LayoutBlock().label("Grid").rows(s.LayoutRow().items(s.LayoutItem().block(s.DividerBlock())));
    const before = builder.build();
    const after = builder.isLabelVisible(false).fallback([]).gridLines("none").build();
    expect(before).not.toHaveProperty("fallback");
    expect(before).not.toHaveProperty("is_label_visible");
    expect(after).toMatchObject({fallback: [], is_label_visible:false, grid_lines:"none"});
    expect(before.rows).toEqual(after.rows);
  });
  it("accepts the surface boundaries and replacement fallback", () => {
    expect(s.message({channel:"C123",blocks:Array.from({length:50},grid)}).blocks).toHaveLength(50);
    expect(s.homeTab({blocks:Array.from({length:100},grid)}).blocks).toHaveLength(100);
    s.message({channel:"C123", blocks:[s.layoutBlock({label:"Grid",rows:[s.layoutRow({items:[s.layoutItem({block:s.dividerBlock()})]})],fallback:Array.from({length:51},()=>s.dividerBlock())})]});
    s.message({channel:"C123",blocks:Array.from({length:50},()=>s.containerBlock({title:"Existing",childBlocks:Array.from({length:10},()=>s.dividerBlock())}))});
  });
  it("rejects row and item extension fields through checked input", () => {
    expect(()=>s.layoutRow({items:[s.layoutItem({block:s.dividerBlock()})], horizontal_alignment:"center"} as never)).toThrow(s.TypeMismatchError);
    expect(()=>s.layoutItem({block:s.dividerBlock(), width:"full"} as never)).toThrow(s.TypeMismatchError);
  });
});

// These functions are type-checked, never executed. A stale role makes tsc fail.
function rejectedRoles() {
  // @ts-expect-error a layout cannot be its own direct child
  s.layoutItem({block:grid()});
  // @ts-expect-error markdown blocks are not layout children
  s.layoutItem({block:s.markdownBlock({text:"No"})});
  // @ts-expect-error actions are not stable fallback blocks
  s.layoutBlock({label:"Grid",rows:[],fallback:[s.actionsBlock([s.button({text:"Go"})])]});
  // @ts-expect-error container full width is not a layout item width
  s.layoutItem({block:s.dividerBlock(),width:"full"});
}
void rejectedRoles;

describe("layout surface error paths", () => {
  it.each(["message", "home"] as const)("retains nested paths on %s", (surface) => {
    const bad = s.alertBlock({text: "Modal only"});
    const nested = s.layoutBlock({label: "Grid", rows: [
      s.layoutRow({items: [s.layoutItem({block: s.dividerBlock()})]}),
      s.layoutRow({items: [s.layoutItem({block: s.dividerBlock()}), s.layoutItem({block: bad})]}),
    ]});
    const cases = [
      {blocks: [s.dividerBlock(), nested], path: "blocks[1].rows[1].items[1].block.type"},
      {blocks: [s.dividerBlock(), s.containerBlock({title: "Group", childBlocks: [s.dividerBlock(), nested]})], path: "blocks[1].child_blocks[1].rows[1].items[1].block.type"},
      {blocks: [grid(), bad], path: "blocks[1].type"},
    ];
    for (const {blocks, path} of cases) {
      for (const fluent of [false, true]) {
        const build = () => surface === "message"
          ? fluent ? s.Message().channel("C123").blocks(blocks).build() : s.message({channel: "C123", blocks})
          : fluent ? s.HomeTab().blocks(blocks).build() : s.homeTab({blocks});
        expect(build).toThrow(s.TypeMismatchError);
        try { build(); } catch (error) {
          expect((error as s.TypeMismatchError).path).toMatch(new RegExp(`(?:^|\\.)${path.replace(/[.[\]]/g, "\\$&")}$`));
          expect((error as Error).message).toContain(`not supported on ${surface} surfaces`);
        }
      }
    }
  });
  it("retains fallback paths on modal surfaces", () => {
    const blocks = [s.dividerBlock(), s.layoutBlock({label: "Grid", rows: [s.layoutRow({items: [s.layoutItem({block: s.dividerBlock()})]})], fallback: [s.dividerBlock(), s.tableBlock({rows: [[s.rawText("Cell")]]})]})];
    for (const build of [() => s.modal({title: "Test", blocks}), () => s.Modal().title("Test").blocks(blocks).build()]) {
      expect(build).toThrow(s.TypeMismatchError);
      try { build(); } catch (error) {
        expect((error as s.TypeMismatchError).path).toMatch(/(?:^|\.)blocks\[1\]\.fallback\[1\]\.type$/);
      }
    }
  });
});


describe("chart width allocation", () => {
  const widths = [undefined, "narrow", "standard", "wide"] as const;
  const weight = (width: typeof widths[number]) => width === "narrow" ? 1 : width === "wide" ? 3 : 2;
  for (const count of [1, 2, 3]) {
    const combinations = widths.flatMap(a => count === 1 ? [[a]] : widths.flatMap(b => count === 2 ? [[a, b]] : widths.map(c => [a, b, c])));
    for (const allocation of combinations) {
      for (let chartPosition = 0; chartPosition < count; chartPosition++) {
        it(`requires a strict majority: ${allocation.map(w => w ?? "omitted").join("/")}, chart ${chartPosition}`, () => {
          const chart = s.dataVisualizationBlock({title: "Chart", chart: s.pieChart([s.chartSegment({label: "One", value: 1})])});
          const items = allocation.map((width, index) => s.layoutItem({block: index === chartPosition ? chart : s.dividerBlock(), ...(width === undefined ? {} : {width})}));
          const chartWeight = weight(allocation[chartPosition]);
          const totalWeight = allocation.reduce((total, width) => total + weight(width), 0);
          if (chartWeight > totalWeight - chartWeight) {
            const row = s.layoutRow({items});
            expect(row.items.map(item => item.width)).toEqual(allocation);
          } else {
            expect(() => s.layoutRow({items})).toThrow(s.InvalidUsageError);
          }
        });
      }
    }
  }
});
