import { describe, expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { App } from "../src/App";
import {
  blockedGate2Model,
  gate2Model,
  requiredTypedKinds,
  reviewSummary,
} from "../src/studioModel";

describe("Packet Studio inspection surface", () => {
  it("renders the local source-frozen packet and every typed section", () => {
    const html = renderToStaticMarkup(<App model={gate2Model} />);

    expect(html).toContain("Source frozen");
    expect(html).toContain("Synthetic source corpus");
    for (const kind of requiredTypedKinds) {
      expect(html).toContain(kind);
    }
    expect(html).toContain("Claim editor");
    expect(html).toContain("Counterevidence editor");
    expect(html).toContain("Uncertainty editor");
    expect(html).toContain("Action option editor");
    expect(html).toContain("Local integrity receipt");
    expect(html).toContain("Byte-for-byte reconstruction");
    expect(html).toContain("Deterministic diff");
  });

  it("renders blocked review reasons without implying a truth score", () => {
    const html = renderToStaticMarkup(<App model={blockedGate2Model} />);
    const summary = reviewSummary(blockedGate2Model);

    expect(summary.publishable).toBe(false);
    expect(html).toContain("Publication blocked");
    expect(html).toContain("unsupported_claim_marked_supported");
    expect(html).toContain("missing_material_counterevidence");
    expect(html).not.toContain("truth score");
    expect(html).not.toContain("objective neutrality");
  });
});
