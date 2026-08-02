export const requiredTypedKinds = [
  "source",
  "claim",
  "evidence",
  "counterevidence",
  "uncertainty",
  "interpretation",
  "value",
  "action",
  "minority_report",
] as const;

export type TypedKind = (typeof requiredTypedKinds)[number];

export type ReviewFinding = {
  code: string;
  severity: "blocker" | "warning";
  message: string;
};

export type StudioModel = {
  packet: {
    id: string;
    version: number;
    status: "source_frozen" | "published";
    question: string;
    scope: string;
  };
  source: {
    title: string;
    origin: string;
    sourceType: string;
    retrievedAt: string;
    digest: string;
    byteLength: number;
    limitations: string[];
  };
  sections: Array<{
    kind: TypedKind;
    count: number;
    status: string;
    detail: string;
  }>;
  review: {
    findings: ReviewFinding[];
    checkedAt: string;
  };
  receipt: {
    status: string;
    packetContentId: string;
    manifestDigest: string;
    objectCount: number;
    integrity: string;
    rawSourceBytesIncluded: boolean;
  };
  reconstruction: {
    status: "verified" | "blocked";
    detail: string;
  };
  diff: {
    fromVersion: number;
    toVersion: number;
    additions: number;
    removals: number;
    changedFields: string[];
  };
};

export function reviewSummary(model: StudioModel): {
  publishable: boolean;
  blockers: ReviewFinding[];
} {
  const blockers = model.review.findings.filter(
    (finding) => finding.severity === "blocker",
  );
  return { publishable: blockers.length === 0, blockers };
}

export const gate2Model: StudioModel = {
  packet: {
    id: "campus-ai-v1",
    version: 1,
    status: "published",
    question:
      "What should a university permit, require, disclose, teach, and prohibit regarding generative AI?",
    scope: "Synthetic Campus AI Constitution pilot fixture.",
  },
  source: {
    title: "Synthetic source corpus",
    origin: "fixture",
    sourceType: "synthetic",
    retrievedAt: "2026-08-01T00:00:00Z",
    digest:
      "sha256:a51372e6aedbb5c7bc4b920f12e955cc38c4b583666c2b32113ab3a38e660d38",
    byteLength: 536,
    limitations: ["Synthetic local fixture; not a real institutional source."],
  },
  sections: requiredTypedKinds.map((kind) => ({
    kind,
    count: 1,
    status: "typed and linked",
    detail: detailFor(kind),
  })),
  review: {
    findings: [],
    checkedAt: "2026-08-02T00:00:00Z",
  },
  receipt: {
    status: "published",
    packetContentId:
      "sha256:44b82a7f98df10847ecb016538b9eba539d0a4b92db66010a9fa9f6d9be30270",
    manifestDigest:
      "sha256:84bcfdd478a1d3731bf6eef5e742c7531de5552626d7b511f70337512e738ede",
    objectCount: 13,
    integrity: "local-only-integrity-receipt",
    rawSourceBytesIncluded: false,
  },
  reconstruction: {
    status: "verified",
    detail: "Published draft reconstructs byte-for-byte from manifest/object IDs.",
  },
  diff: {
    fromVersion: 1,
    toVersion: 2,
    additions: 0,
    removals: 0,
    changedFields: ["claim:claim-disclosure-boundary.proposition"],
  },
};

export const blockedGate2Model: StudioModel = {
  ...gate2Model,
  packet: { ...gate2Model.packet, status: "source_frozen" },
  review: {
    checkedAt: "2026-08-02T00:00:00Z",
    findings: [
      {
        code: "unsupported_claim_marked_supported",
        severity: "blocker",
        message: "An unsupported claim cannot be published with status supported.",
      },
      {
        code: "missing_material_counterevidence",
        severity: "blocker",
        message: "Material counterevidence must remain visible before publication.",
      },
    ],
  },
  receipt: {
    ...gate2Model.receipt,
    status: "not issued",
  },
  reconstruction: {
    status: "blocked",
    detail: "No receipt is issued while review has blocking findings.",
  },
};

function detailFor(kind: TypedKind): string {
  switch (kind) {
    case "source":
      return "origin, digest, retrieval time, type, and limitations";
    case "claim":
      return "proposition, support status, evidence links, and materiality";
    case "evidence":
      return "source references, direction, summary, and confidence";
    case "counterevidence":
      return "material challenge references and visible disclosure";
    case "uncertainty":
      return "level, reasons, and claim linkage";
    case "interpretation":
      return "assumptions, counterarguments, and distinguishing tests";
    case "value":
      return "holder scope and tensions";
    case "action":
      return "risks, reversibility, success measures, and stop conditions";
    case "minority_report":
      return "position, evidence references, and unresolved risk";
  }
}
