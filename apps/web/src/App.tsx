import type { ReactNode } from "react";
import { gate2Model, reviewSummary, type StudioModel } from "./studioModel";

export type AppProps = { model?: StudioModel };

export function App({ model = gate2Model }: AppProps) {
  const summary = reviewSummary(model);
  return (
    <main className="app-shell">
      <header className="hero">
        <div>
          <p className="eyebrow">COMMON REALITY / GATE 2</p>
          <h1>Packet Studio</h1>
          <p className="lede">
            A local inspection surface for evidence-bound packet versions. It
            exposes status and provenance; it does not decide truth.
          </p>
        </div>
        <div className="status-chip" data-status={model.packet.status}>
          {model.packet.status === "published"
            ? "Published / local receipt"
            : "Source frozen / review required"}
        </div>
      </header>

      <section className="panel source-panel" aria-labelledby="source-heading">
        <SectionHeading eyebrow="01 / provenance" id="source-heading">
          Source import
        </SectionHeading>
        <div className="two-column">
          <div>
            <h3>{model.source.title}</h3>
            <p>{model.source.limitations.join(" ")}</p>
          </div>
          <dl className="facts">
            <Fact label="State" value="Source frozen" />
            <Fact label="Origin" value={model.source.origin} />
            <Fact label="Type" value={model.source.sourceType} />
            <Fact label="Retrieved" value={model.source.retrievedAt} />
            <Fact label="Bytes" value={String(model.source.byteLength)} />
            <Fact label="Digest" value={model.source.digest} mono />
          </dl>
        </div>
      </section>

      <section className="panel" aria-labelledby="packet-heading">
        <SectionHeading eyebrow="02 / identity" id="packet-heading">
          Packet outline and version
        </SectionHeading>
        <div className="two-column">
          <div>
            <h3>{model.packet.question}</h3>
            <p>{model.packet.scope}</p>
          </div>
          <dl className="facts">
            <Fact label="Packet" value={model.packet.id} mono />
            <Fact label="Version" value={String(model.packet.version)} />
            <Fact label="Status" value={model.packet.status} />
          </dl>
        </div>
      </section>

      <section className="panel" aria-labelledby="typed-heading">
        <SectionHeading eyebrow="03 / typed editing" id="typed-heading">
          Claim, evidence, and decision objects
        </SectionHeading>
        <div className="object-grid">
          {model.sections.map((section) => (
            <article className="object-card" key={section.kind}>
              <div className="object-card__topline">
                <span className="tag">{section.kind}</span>
                <span className="muted">{section.count} object</span>
              </div>
              <h3>{section.status}</h3>
              <p>{section.detail}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="panel" aria-labelledby="editor-heading">
        <SectionHeading eyebrow="03a / local draft" id="editor-heading">
          Packet editors
        </SectionHeading>
        <form
          className="editor-grid"
          onSubmit={(event) => event.preventDefault()}
        >
          <label>
            Claim editor
            <textarea
              defaultValue="A policy pilot should distinguish permitted assistance from undisclosed substitution."
              rows={3}
            />
          </label>
          <label>
            Evidence editor
            <textarea
              defaultValue="Disclosure can preserve independent mastery checks."
              rows={3}
            />
          </label>
          <label>
            Counterevidence editor
            <textarea
              defaultValue="Uniform disclosure can burden legitimate accessibility use without an appeal path."
              rows={3}
            />
          </label>
          <label>
            Uncertainty editor
            <textarea
              defaultValue="Assessment and accessibility contexts differ."
              rows={3}
            />
          </label>
          <label>
            Interpretation editor
            <textarea
              defaultValue="Disclosure may need context-specific rules and an appeal path."
              rows={3}
            />
          </label>
          <label>
            Value position editor
            <textarea
              defaultValue="Policy should not penalize legitimate accessibility support."
              rows={3}
            />
          </label>
          <label>
            Action option editor
            <textarea
              defaultValue="Run a reversible course-level disclosure and appeal pilot."
              rows={3}
            />
          </label>
          <label>
            Minority report editor
            <textarea
              defaultValue="Students using assistive technology need a safe appeal path."
              rows={3}
            />
          </label>
          <button type="submit">Keep local draft</button>
        </form>
        <p className="muted">
          This form is a local draft surface. It does not publish, call a
          model, send telemetry, or commit a person to an action.
        </p>
      </section>

      <section
        className={"panel review-panel " + (summary.publishable ? "is-clear" : "is-blocked")}
        aria-labelledby="review-heading"
      >
        <SectionHeading eyebrow="04 / deterministic review" id="review-heading">
          {summary.publishable ? "Review clear for publication" : "Publication blocked"}
        </SectionHeading>
        {summary.publishable ? (
          <p className="callout">
            No blocking findings. Publication still records process integrity,
            not truth or neutrality.
          </p>
        ) : (
          <div className="failure-list">
            <p className="callout">
              The packet cannot publish until each blocking finding is resolved
              and reviewed again.
            </p>
            <ul>
              {summary.blockers.map((finding) => (
                <li key={finding.code}>
                  <strong>{finding.code}</strong>
                  <span>{finding.message}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
        <p className="muted">Checked locally at {model.review.checkedAt}.</p>
      </section>

      <section className="panel" aria-labelledby="receipt-heading">
        <SectionHeading eyebrow="05 / publication" id="receipt-heading">
          Local integrity receipt
        </SectionHeading>
        <div className="two-column">
          <dl className="facts">
            <Fact label="Receipt" value={model.receipt.status} />
            <Fact label="Packet content ID" value={model.receipt.packetContentId} mono />
            <Fact label="Manifest digest" value={model.receipt.manifestDigest} mono />
            <Fact label="Objects" value={String(model.receipt.objectCount)} />
          </dl>
          <div>
            <p className="callout">
              {model.receipt.integrity}. Raw source bytes included:{" "}
              {model.receipt.rawSourceBytesIncluded ? "yes" : "no"}.
            </p>
            <p className="muted">
              A receipt proves local integrity and process history, not source
              correctness or universal truth.
            </p>
          </div>
        </div>
      </section>

      <section className="panel" aria-labelledby="reconstruction-heading">
        <SectionHeading eyebrow="06 / integrity" id="reconstruction-heading">
          Byte-for-byte reconstruction
        </SectionHeading>
        <div className={"integrity-state " + model.reconstruction.status}>
          <span className="integrity-dot" aria-hidden="true" />
          <div>
            <h3>{model.reconstruction.status}</h3>
            <p>{model.reconstruction.detail}</p>
          </div>
        </div>
      </section>

      <section className="panel" aria-labelledby="diff-heading">
        <SectionHeading eyebrow="07 / change review" id="diff-heading">
          Deterministic diff
        </SectionHeading>
        <p className="muted">
          Version {model.diff.fromVersion} → {model.diff.toVersion}; additions{" "}
          {model.diff.additions}, removals {model.diff.removals}.
        </p>
        <ul className="change-list">
          {model.diff.changedFields.map((field) => (
            <li key={field}>{field}</li>
          ))}
        </ul>
      </section>

      <footer className="footer-note">
        <strong>Synthetic, local, and bounded.</strong> This surface does not
        claim human-pilot success, model quality, neutrality, democratic
        legitimacy, external witnessing, or Gate 3/4 behavior.
      </footer>
    </main>
  );
}

function SectionHeading({
  eyebrow,
  id,
  children,
}: {
  eyebrow: string;
  id: string;
  children: ReactNode;
}) {
  return (
    <div className="section-heading">
      <p className="eyebrow">{eyebrow}</p>
      <h2 id={id}>{children}</h2>
    </div>
  );
}

function Fact({
  label,
  value,
  mono = false,
}: {
  label: string;
  value: string;
  mono?: boolean;
}) {
  return (
    <div className="fact">
      <dt>{label}</dt>
      <dd className={mono ? "mono" : undefined}>{value}</dd>
    </div>
  );
}
