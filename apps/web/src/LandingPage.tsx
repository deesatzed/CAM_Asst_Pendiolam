const statusCards = [
  {
    gate: "Gate 0",
    label: "Fixture boundary",
    detail: "Synthetic source corpus frozen with digests and adversarial cases.",
  },
  {
    gate: "Gate 1",
    label: "Schema and authority",
    detail: "Typed objects, canonical identities, ledger, Merkle checkpoints, and exact grants.",
  },
  {
    gate: "Gate 2",
    label: "Packet Studio",
    detail: "Local import, fail-closed review, immutable publication, receipts, diffs, and reconstruction.",
  },
];

const featureCards = [
  {
    number: "01",
    title: "Freeze the source",
    detail: "Capture provenance, byte length, and a digest before interpretation can drift.",
  },
  {
    number: "02",
    title: "Type the record",
    detail: "Keep claims, evidence, counterevidence, uncertainty, values, action options, and minority reports distinct.",
  },
  {
    number: "03",
    title: "Block unsafe publication",
    detail: "Missing support and missing material counterevidence are review findings, not invisible caveats.",
  },
  {
    number: "04",
    title: "Rebuild what was published",
    detail: "Content-addressed objects, manifests, receipts, and deterministic diffs make change inspectable.",
  },
  {
    number: "05",
    title: "Keep authority explicit",
    detail: "A receipt proves integrity and process history. It never pretends to prove truth or neutrality.",
  },
  {
    number: "06",
    title: "Work locally first",
    detail: "The Gate 2 flow needs no account, hosted database, telemetry, or model provider.",
  },
];

const methods = [
  "A typed epistemic grammar for shared records",
  "Reconstruction as an integrity precondition for later reasoning",
  "Counterevidence and uncertainty as mandatory visibility",
  "Evidence gates that convert roadmap claims into falsifiable proof",
];

export function LandingPage() {
  return (
    <main className="landing-shell">
      <header className="landing-nav">
        <a className="landing-brand" href="/">
          COMMON REALITY
        </a>
        <nav aria-label="Product navigation">
          <a href="#features">Features</a>
          <a href="#methods">Methods</a>
          <a className="button button--small" href="#studio">
            Open Packet Studio
          </a>
        </nav>
      </header>

      <section className="landing-hero" aria-labelledby="landing-title">
        <div className="landing-hero__copy">
          <p className="landing-eyebrow">Evidence over persuasion</p>
          <h1 id="landing-title">
            Build the shared record before asking people to agree.
          </h1>
          <p className="landing-hero__lede">
            Common Reality is a local-first workspace for evidence-bound Shared
            Reality Packets. It separates what was observed, what was inferred,
            what is valued, what is proposed, and what remains unknown.
          </p>
          <div className="landing-actions">
            <a className="button" href="#studio">
              Open Packet Studio
              <span aria-hidden="true">↗</span>
            </a>
            <a className="text-link" href="#features">
              See the feature map
              <span aria-hidden="true">↓</span>
            </a>
          </div>
          <p className="landing-boundary">
            Gate 2 verified with synthetic local data. The system is an
            integrity and review substrate, not a truth oracle.
          </p>
        </div>
        <div className="landing-hero__signal" aria-label="Packet integrity signal">
          <div className="signal-ring">
            <span className="signal-ring__core" />
          </div>
          <div>
            <p className="landing-eyebrow">PACKET STATE</p>
            <p className="signal-title">Source frozen</p>
            <p className="signal-detail">
              Provenance captured
              <br />
              Review remains explicit
            </p>
          </div>
          <div className="signal-lines" aria-hidden="true">
            <span />
            <span />
            <span />
          </div>
        </div>
      </section>

      <section className="status-grid" aria-label="Verified gate status">
        {statusCards.map((card) => (
          <article className="status-card" key={card.gate}>
            <p className="landing-eyebrow">{card.gate}</p>
            <h2>{card.label}</h2>
            <p>{card.detail}</p>
          </article>
        ))}
        <article className="status-card status-card--deferred">
          <p className="landing-eyebrow">NEXT</p>
          <h2>Gates 3–10 deferred</h2>
          <p>
            Lens, Bridge, Disagreement Map, human pilot, memory, federation,
            and Chorus each have separate proof gates.
          </p>
        </article>
      </section>

      <section className="landing-section landing-section--split" id="features">
        <div className="section-intro">
          <p className="landing-eyebrow">THE PROBLEM</p>
          <h2>Most records lose the disagreement before the meeting ends.</h2>
        </div>
        <div className="section-copy">
          <p>
            A polished paragraph can hide a changed source, an unsupported
            claim, a missing caveat, or a minority concern that was never
            resolved. Common Reality treats those as data-integrity problems,
            not merely writing problems.
          </p>
          <p>
            The product makes the record inspectable enough for a person to
            review, correct, compare, reconstruct, and deliberately export.
          </p>
        </div>
      </section>

      <section className="feature-grid" aria-label="Common Reality features">
        {featureCards.map((feature) => (
          <article className="feature-card" key={feature.number}>
            <p className="feature-number">{feature.number}</p>
            <h2>{feature.title}</h2>
            <p>{feature.detail}</p>
          </article>
        ))}
      </section>

      <section className="landing-section landing-section--methods" id="methods">
        <div className="section-intro">
          <p className="landing-eyebrow">WHY THIS APPROACH</p>
          <h2>Make uncertainty legible without turning it into a score.</h2>
        </div>
        <div className="method-list">
          {methods.map((method, index) => (
            <div className="method-row" key={method}>
              <span className="method-index">0{index + 1}</span>
              <p>{method}</p>
            </div>
          ))}
        </div>
      </section>

      <section className="comparison-panel" aria-labelledby="difference-title">
        <div>
          <p className="landing-eyebrow">THE DIFFERENCE</p>
          <h2 id="difference-title">
            The product boundary lives in the data model, not the marketing
            copy.
          </h2>
        </div>
        <div className="comparison-list">
          <p>
            <span>Conventional default</span>
            One mutable narrative, one confidence signal, one current version.
          </p>
          <p>
            <span>Common Reality</span>
            Typed objects, fail-closed review, immutable versions, and
            reconstructable evidence.
          </p>
          <p>
            <span>Explicit non-claim</span>
            Integrity is not truth. A clearer Lens is not more authority.
          </p>
        </div>
      </section>

      <section className="roadmap-section" aria-labelledby="roadmap-title">
        <div>
          <p className="landing-eyebrow">LATER SCOPE</p>
          <h2 id="roadmap-title">A roadmap with proof obligations.</h2>
        </div>
        <p>
          Later gates add declared Lenses, participant-reviewed Bridges,
          disagreement maps, human pilots, explainable memory, typed
          AI-to-AI boundaries, group-agent lineage, and federation. Each item
          remains deferred until its own adversarial evidence passes.
        </p>
        <a className="text-link" href="#studio">
          Inspect the verified slice
          <span aria-hidden="true">↗</span>
        </a>
      </section>

      <footer className="landing-footer">
        <div>
          <p className="landing-eyebrow">COMMON REALITY</p>
          <p className="landing-footer__statement">
            A better substrate for human judgment, audit, collaboration, and
            correction.
          </p>
        </div>
        <div className="landing-footer__links">
          <a href="#studio">Open Packet Studio</a>
          <a href="https://github.com/deesatzed/CAM_Asst_Pendiolam.git">
            Source repository
          </a>
        </div>
      </footer>
    </main>
  );
}
