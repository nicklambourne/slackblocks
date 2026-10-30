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
    expect(s.message({channel:"C123",blocks:Array.from({length:25},grid)}).blocks).toHaveLength(25);
    expect(s.homeTab({blocks:Array.from({length:50},grid)}).blocks).toHaveLength(50);
    s.message({channel:"C123", blocks:[s.layoutBlock({label:"Grid",rows:[s.layoutRow({items:[s.layoutItem({block:s.dividerBlock()})]})],fallback:Array.from({length:50},()=>s.dividerBlock())})]});
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
