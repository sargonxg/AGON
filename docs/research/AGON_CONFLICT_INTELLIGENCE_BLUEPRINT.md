# **AGON: A Deep-Tech Blueprint for Large-Scale Conflict Intelligence**

## **Executive Summary and MVP Product Thesis**

The contemporary enterprise operates within dense webs of human interaction, contractual agreements, and organizational expectations. When these systems fail, the resulting friction manifests as legal disputes, workplace conflicts, and commercial deadlocks. Traditional software approaches to managing these disputes have bifurcated into two distinct categories. On one end are eDiscovery platforms, which are highly optimized for bulk document preservation, metadata search, and chain-of-custody tracking but offer little semantic understanding of the disputes themselves.1 On the other end are generative artificial intelligence chatbots that summarize text but introduce severe hallucination risks, obscure the evidentiary chain of custody, and inherently average out the very contradictions that define a conflict.2

The core product thesis of AGON represents a fundamental departure from both paradigms. Conflict intelligence requires a deterministic, auditable substrate that extracts, structures, and explains the hidden dynamics of a dispute without overriding human judgment. AGON is not an artificial intelligence summarization tool or a generic conversational agent. It is a deep-tech conflict primitive engine. It is designed to ingest unstructured, messy human text—such as deposition transcripts, email threads, human resources complaints, and negotiation logs—and computationally resolve this text into a queryable graph of actors, claims, commitments, contradictions, and evidentiary spans.

The Minimum Viable Product must rely on a rigid, evidence-backed typed workflow where every computational conclusion is anchored to a specific character offset in the source document. By treating conflict as an ontology of primitive objects rather than a continuous narrative, AGON enables investigators, mediators, and legal professionals to navigate disputes probabilistically and analytically. The architecture leverages Rust for high-performance, type-safe orchestration 4, PostgreSQL for relational graph persistence 6, and advanced language models strictly constrained to structured information extraction. The ultimate ambition is a general conflict-understanding substrate for human systems, but the immediate goal is to deliver a trusted analytical engine that exposes conflict structures quickly, accurately, and auditably.

## **Product Definition and First Market Wedge**

### **The AGON MVP Definition**

In its MVP form, AGON operates as an API-first analytical engine accompanied by an embedded web dashboard designed to process unstructured conflict narratives. The system accepts long-form text, parses it into semantically meaningful chunks, and applies multi-pass extraction pipelines to populate a relational graph of conflict primitives. The output consists of auditable primitives with source-backed evidence. Every conclusion rendered by the system traces back directly to source quotes and spans.

Crucially, AGON must explicitly avoid certain functionalities in the MVP to maintain trust and legal defensibility. It will not feature a generic open-ended conversational chatbot interface. It will not attempt to determine legal guilt, predict case outcomes, generate automated legal advice, or autonomously resolve disputes. Such features undermine user trust, run afoul of compliance frameworks regarding the unauthorized practice of law, and exceed the reliability thresholds of current language models without extensive, brittle guardrails.3 Instead, AGON surfaces structural tensions and leaves the ultimate judgment and strategic decision-making to human professionals.

### **Recommended Market Wedge: Enterprise HR and Workplace Investigations**

While commercial litigation and eDiscovery represent massive addressable markets, they are structurally saturated with established platforms focused primarily on document indexing.1 The strongest initial paid wedge for AGON is enterprise human resources workplace investigations and internal corporate mediation.

Workplace conflict management solutions are experiencing rapid growth, driven by stricter compliance reforms, the complexities of remote and hybrid work environments, and a rising zero-tolerance culture for workplace harassment.8 In 2025, workplace misconduct reached a near seven-year high, with over half of employees reporting having witnessed or experienced misconduct.10 Despite this volume, investigations are frequently bogged down by manual evidence reconciliation and fragmented data silos across emails, chat logs, and interview notes.11 Organizations face high urgency and possess a strong willingness to pay in this sector because poorly handled investigations lead to "nuclear verdicts," massive reputational damage, and high employee turnover.10

Enterprise investigation teams require rigorous adherence to structured methodologies—such as the standard "Plan, Investigate, Determine" framework—to ensure legal defensibility.10 AGON perfectly aligns with this need by providing an auditable, unbiased, and mathematically verifiable account of a dispute, serving as the ultimate analytical layer during the "Investigate" and "Determine" phases.

### **The Target Persona and User Experience**

The primary user persona with the highest urgency is the Senior Employee Relations Investigator or Internal Corporate Mediator. This professional operates under intense pressure to deliver impartial, fact-based reports that can withstand external legal scrutiny. They are frequently overwhelmed by the cognitive load of reading dozens of conflicting interview transcripts and email chains, attempting to manually map who knew what, when they knew it, and where individual testimonies contradict one another.

The "wow moment" for this persona must occur within two minutes of uploading a complex dossier—for example, a 50-page collection containing an initial complaint, a Slack export, and three conflicting interview transcripts. Upon processing, AGON automatically renders a Friction Matrix, a visual heat map showing tension points between actors, alongside a deterministic list of Contradiction Cards. When the investigator clicks on a contradiction, the user interface instantly snaps to the exact highlighted sentences in the source documents. The investigator immediately visualizes the structural fault lines of the case without having to manually cross-reference pages, drastically reducing time-to-insight while maintaining total evidentiary control.

### **UI and Reporting Surfaces**

To support this workflow, the AGON MVP requires a highly structured user interface comprising six core views. The "Overview" provides high-level session metadata and a summary of extracted primitive counts. The "Actors" panel lists canonicalized identities, their aliases, and their calculated roles within the dispute. The "Friction" panel visualizes the Friction Matrix, allowing users to select intersecting actor nodes to reveal underlying tensions. The "Contradictions" view serves as a triage queue for opposing claims, displaying side-by-side evidence spans. The "Evidence" view catalogs all verified spans, commitments, and obligations in a filterable table. Finally, the "Raw" view provides the original ingested text with interactive, color-coded highlighting overlaying the extracted primitives, ensuring the user never loses the context of the original document.

## **Core Conflict Ontology**

To compute and analyze conflict, human language must be mapped to a highly constrained data structure. Drawing theoretical inspiration from the Argument Interchange Format and the Legal Knowledge Interchange Format, the AGON ontology separates informational nodes from relational and conflict scheme nodes.13 This separation ensures that raw claims are preserved independently of the system's evaluation of how those claims interact. The architecture assumes that EvidenceSpan and Provenance types already exist in the core framework and serve as the foundational building blocks for all subsequent extractions.

The following table defines the practical ontology for the MVP, detailing the fields, evidence requirements, generation strategies, and topological connections for each object.

| Object | Core Fields | Required Evidence | Generation Strategy | Source Quote / Connection Requirements |
| :---- | :---- | :---- | :---- | :---- |
| **Actor** | id, canonical\_name, role, type (Person/Org) | Inferred from document metadata, extracted via NER. | Deterministic normalization \+ Model-suggested conservative merging. | Must appear as a named entity or identifiable pronoun; connects to all actions. |
| **Alias** | id, actor\_id, surface\_form, context | Mention in text matching the entity. | Deterministic exact/fuzzy match against source. | Required exact string span; connects to canonical Actor. |
| **Claim** | id, actor\_id, statement, subject\_actor\_id | Validated EvidenceSpan supporting the assertion. | Model-generated summary constrained by schema. | Must map exactly to one or more contiguous spans; foundational I-node. |
| **Denial** | id, actor\_id, target\_claim\_id, statement | EvidenceSpan explicitly refuting a prior claim. | Model-extracted, deterministically linked to target. | Must contain negating language anchored to text; points to target Claim. |
| **Event** | id, description, timestamp, location | EvidenceSpan describing a temporal occurrence. | Model-extracted, strict ISO-8601 parsing. | Character offsets covering the event description; enables timeline sorting. |
| **Commitment** | id, promising\_actor\_id, receiving\_actor\_id, action, deadline | EvidenceSpan indicating promise or contract. | Model-extracted via intent parsing. | Spans showing promissory language ("I will", "agreed"); tracks obligations. |
| **Obligation** | id, actor\_id, duty\_description, source\_id | EvidenceSpan referencing policy, contract, or norm. | Model-extracted, legally agnostic interpretation. | Spans referencing rules or expectations; connects to Events for breach checks. |
| **Accusation** | id, accusing\_actor\_id, accused\_actor\_id, nature | EvidenceSpan containing hostile or blaming language. | Model-extracted via sentiment and intent analysis. | Spans directed at another actor's culpability; feeds Friction Matrix. |
| **Admission** | id, actor\_id, conceded\_claim\_id | EvidenceSpan acknowledging fault or contested fact. | Model-extracted, requiring human verification. | Explicit concessive language; resolves Contradictions. |
| **EvidenceSpan** | id, chunk\_id, start\_idx, end\_idx, exact\_text | The raw text bytes from the source chunk. | Deterministic string matching and offset calculation. | The atomic unit of AGON; all above objects possess an evidence\_span\_id. |
| **Contradiction** | id, type, claim\_a\_id, claim\_b\_id, resolution\_status | Two distinct EvidenceSpans that cannot logically co-exist. | Hybrid: LLM-suggested \+ Deterministic logical checks. | Dual quotes required; must be mutually exclusive; acts as an S-node. |
| **TimelineConflict** | id, event\_a\_id, event\_b\_id, temporal\_overlap | Conflicting timestamps or sequence descriptions. | Deterministic date mathematics. | Extracted temporal spans; highlights narrative impossibilities. |
| **RelationshipEdge** | id, source\_actor, target\_actor, weight, type | Aggregation of underlying interpersonal interactions. | Deterministic computation based on children objects. | Derived from nested EvidenceSpans; builds the overall property graph. |
| **PowerDynamic** | id, dominant\_actor, subordinate\_actor, basis | EvidenceSpan indicating hierarchy or leverage. | Model-inferred (requires low confidence flag). | Spans indicating authority ("my boss", "approved by"). |
| **Interest/Need** | id, actor\_id, underlying\_motivation | EvidenceSpan expressing desire or necessity. | Model-inferred (mediation specific). | Spans expressing underlying goals ("I just want", "we need"). |
| **EscalationSignal** | id, actor\_id, intensity\_score, trigger | EvidenceSpan containing threats or ultimatums. | Model-extracted via sentiment analysis. | Quotes of elevated emotional or adversarial tone. |
| **ResolutionOpp.** | id, actor\_id, proposed\_concession | EvidenceSpan offering compromise. | Model-extracted. | Quotes showing willingness to settle or mediate. |
| **FrictionPattern** | id, actor\_a, actor\_b, matrix\_score, factors | Aggregation of contradictions, denials, and accusations. | Deterministic rolling calculation. | Derived completely from foundational objects; powers the main UI. |
| **SourceDocument** | id, filename, hash, mime\_type, upload\_date | File bytes. | Deterministic ingestion processing. | The root container for all derived data. |
| **Chunk** | id, document\_id, start\_idx, end\_idx, text | Algorithmic semantic segmentation. | Deterministic. | The bounded text region processed by the language model. |

The topological design ensures that high-level abstractions, such as a FrictionPattern, are entirely derivative. They do not exist as independent hallucinations; they are mathematical roll-ups of discrete Contradiction, Accusation, and Denial nodes, which are themselves immutably bound to EvidenceSpans.

## **Evidence and Trust Architecture**

In legal, compliance, and human resources domains, the traditional paradigm of "seeing is believing" has been largely replaced by a "verify then trust" culture due to the proliferation of artificial intelligence hallucinations and synthetic data generation.2 An analytical system's output is only actionable if its provenance is both mathematically and procedurally proven.16 To satisfy these requirements, AGON implements a strict, multi-layered evidence architecture that heavily leverages the pre-existing EvidenceSpan and Provenance types in the Rust core.

The architecture enforces quote verification as a hard gate. When the language model extracts a Claim or an Accusation, the structured JSON output schema strictly requires an exact\_quote field. The Rust backend intercepts this output and executes a deterministic substring search of the quote against the original text of the specific Chunk. If the exact match succeeds, the start\_index and end\_index are recorded, and the Provenance status is marked as "Verified."

However, language models frequently apply minor formatting alterations, drop punctuation, or slightly paraphrase text during extraction. To handle span recovery when a quote is slightly altered, AGON employs a fuzzy-matching fallback mechanism. Utilizing algorithms such as Levenshtein distance or semantic word-overlap mapped to character arrays 5, the system attempts to align the suggested quote with the source text. If the alignment meets a rigorous similarity threshold (e.g., greater than 95%), the system extracts the true character offsets from the source and permanently binds them to the EvidenceSpan, discarding the model's altered string.

To maintain auditability, data states are explicitly delineated. "Verified" indicates a mathematically proven character offset match. "Model-Suggested" indicates a fuzzy match that requires human confirmation. "Inferred" is reserved for high-level abstractions, such as a PowerDynamic, where the system derives a conclusion from tone rather than explicit statements. "Unresolved" marks extractions that failed alignment and are quarantined from the graph. Legal and human resources users expect human-in-the-loop workflows; thus, the user interface presents all "Model-Suggested" and "Inferred" primitives with a visual toggle, requiring the investigator to explicitly accept or reject the data before it influences the Friction Matrix.

## **Large Text Ingestion Strategy**

Processing extensive, contradictory narratives requires an ingestion pipeline that preserves linguistic context while preventing context-window overflow and memory exhaustion. The ingestion strategy is designed to handle the specific idiosyncrasies of conflict texts, such as email threads and deposition transcripts.

The pipeline begins with document parsing and deduplication. Ingested files are parsed, stripped of complex formatting, and immediately hashed using SHA-256 at the document level. This source hashing prevents duplicate processing and establishes the foundational chain of custody.17 Incremental reprocessing is natively supported; because chunks are securely hashed, adding a supplementary document to an existing case session only triggers extraction for the novel text, seamlessly updating the existing relational graph with new edges.

Chunking strategy is critical for conflict narratives. Standard token-count chunking is insufficient because severing a sentence that separates an accusation from a denial destroys the relational context. AGON utilizes semantic chunking with strict sentence boundary awareness 5, ensuring that conversational turns in transcripts or contiguous email blocks remain intact within a single chunk.

For transcripts and email threads, deterministic regular expressions and structural parsing initially extract speaker labels and metadata. The text is subsequently segmented into speaker-attributed chunks. A significant challenge in transcript ingestion is the prevalence of Automatic Speech Recognition (ASR) errors. Research indicates that attribution models are resilient to word-level ASR transcription errors, provided the underlying semantic intent remains intact.18 The fuzzy-matching span recovery mechanism described previously mitigates the impact of minor transcription anomalies during evidence linking.

When handling contradictory documents, AGON does not attempt premature reconciliation during ingestion. If a plaintiff's complaint states a specific sequence of events and a defendant's email thread provides a conflicting timeline, both are extracted as valid Claims and Events, tagged with their respective document provenance. The reconciliation and highlighting of these discrepancies occur downstream in the contradiction detection phase, preserving the integrity of the conflicting narratives.

## **Actor Canonicalization**

Entity resolution in unstructured text is a notoriously complex problem. In a single dossier, an individual might be referred to as "Mr. Smith," "John," "the plaintiff," or "he." In high-stakes legal and internal investigation contexts, a false positive merge—combining two different individuals into a single actor profile—is a catastrophic failure that corrupts the entire conflict graph. Conversely, a false negative—keeping them separate—is a minor inconvenience easily rectified by human intervention.

AGON adopts an Evidence Graph approach coupled with an evidence-gated state-update policy known as a Commit Gate.19 This represents a highly conservative MVP strategy prioritizing precision over recall.

The pipeline operates in tiers. Tier-0 executes deterministic normalization. Explicit identifiers such as email addresses, employee identification numbers, and full names are normalized—honorifics are stripped, strings are lowercased—and deterministically merged. Tier-1 utilizes the Commit Gate. All extracted actor fragments, including ambiguous pronouns and role labels, are initially maintained in isolated sub-graphs to prevent cross-entity contamination.19

The system merges these entities automatically only when a strict threshold of quasi-identifiers is met. For example, a merge is committed if the fragments share a last name, a matching organizational role, and temporal proximity in the text. Ambiguous pronouns are resolved using dictionary-based anaphora resolution strictly within the bounds of the immediate semantic chunk.19

To represent uncertainty, if the language model suggests that "the manager" likely refers to "Jane Doe" based on contextual clues, this alias is stored in the actor\_aliases table with a corresponding confidence score. The user interface surfaces this as an "Unconfirmed Alias," prompting manual merge or split actions from the investigator. Vector embeddings are explicitly avoided for automated deterministic merging in the MVP, as they introduce unpredictable semantic overlap risks. Embeddings are only utilized as a background process to cluster potential aliases for human review, adhering to the principle that automation should assist, not override, identity resolution in sensitive contexts.

## **Contradiction and Friction Detection**

Detecting the exact points where human narratives collide is the core value proposition of the AGON system. Traditional retrieval-augmented generation systems fail in this regard because their objective functions are designed to synthesize and average out opposing views into a coherent summary.3 AGON, conversely, is designed to isolate and highlight these fractures using a practical hybrid system combining language model extraction with deterministic logic.

The following defines the contradiction types and detection methods implemented in the MVP:

| Contradiction Type | Detection Method | Required Data | Example | Confidence Strategy & Failure Modes |
| :---- | :---- | :---- | :---- | :---- |
| **Accusation/Denial Pairs** | LLM relational extraction \+ deterministic actor matching. | Claim \+ Accusation referencing the Claim. | *A: "He stole the files." B: "I never took any files."* | High confidence if actors match. Failure mode: Sarcasm or rhetorical questions. Mitigation: Require explicit negative keywords in the denial span. |
| **Mutually Exclusive Factual Claims** | LLM embedding proximity \+ opposing sentiment/intent classification. | Two Claims regarding the identical subject matter. | *A: "The meeting occurred on Tuesday." B: "The meeting happened on Friday."* | Medium confidence. Failure mode: Context shifts (referring to two different meetings). Mitigation: Both claims must share tight temporal or spatial metadata. |
| **Broken Commitments** | Deterministic rule: Commitment deadline \< Event timestamp (or absence thereof). | Commitment \+ sequence of Events. | *A promises a report by Friday. B's email proves the report arrived Monday.* | High confidence. Failure mode: Informally waived deadlines not documented. Mitigation: Expose dates clearly for human verification. |
| **Obligation Disputes** | LLM-extracted rule mapped to conflicting action event. | Obligation \+ Event. | *Policy requires 2 approvals. Event logs show 1 approval.* | Medium confidence. Failure mode: Documented exceptions to policy. Mitigation: Human review strictly required for policy interpretation. |
| **Timeline Inconsistencies** | Deterministic date mathematics. | Multiple Events with overlapping or impossible sequences. | *Actor claims to be in New York at 2 PM and London at 4 PM.* | High confidence. Failure mode: Unstated timezone misalignments. Mitigation: Normalize all extracted dates to UTC before computation. |
| **Conflicting Responsibility Claims** | LLM intent extraction. | Two actors claiming exclusive ownership of a decision. | *A: "I authorized the payment." B: "I was the sole authorizer."* | Low confidence. Requires human-in-the-loop review to determine if joint responsibility was possible. |

Every detected conflict generates an Evidence-backed Contradiction Card within the user interface. This card presents the opposing claims side-by-side, visually highlighting the divergent EvidenceSpans and providing direct hyperlinked access to the source documents, ensuring total transparency.

## **Friction Matrix**

To quantify interpersonal tension within an organization or dispute, AGON generates the Friction Matrix. This feature translates subjective textual hostility into an objective, mathematical abstraction of the relationship graph.21 The objective is to move beyond the subjective "vibes" of a conflict and provide an investigator with a defensible metric of actor-versus-actor tension.

The Friction Matrix calculates a continuous tension score for every pair of actors. To avoid the trap of fake precision—where neural network weights produce an unexplainable, black-box score—the AGON tension metric is strictly additive, based entirely on transparent, auditable events.

The scoring algorithm assigns weighted values to specific primitives. A direct contradiction between two actors adds significant weight to their friction score. Explicit accusations carry the highest weight, as they represent direct adversarial actions. Broken commitments and extracted escalation signals (such as legal threats or intense emotional language) provide moderate score increases. Furthermore, if a PowerDynamic edge exists between the actors—for example, a manager and a direct report—a multiplier is applied to the escalation signals, reflecting the higher institutional risk associated with hierarchical friction.22

Explainability is paramount. The matrix is displayed as a visual heat map. When an investigator selects a high-tension cell intersecting two actors, a side-panel explains the score in plain language, avoiding mathematical jargon. For example, the system will state: *"Tension Score: High. This score is driven by 4 distinct contradictions regarding the Q3 timeline, 2 broken commitments, and 1 direct accusation of policy violation."* Every cited reason functions as a hyperlink, connecting directly back to the source-backed reasons and their underlying EvidenceSpans.

## **Deep Capabilities Roadmap**

To maintain engineering velocity and focus on the MVP, advanced capabilities must be strictly triaged. The following roadmap ranks post-MVP features by their usefulness, technical difficulty, trust risk, and commercial value.

### **Ranked Backlog (Post-MVP)**

1. **Multi-Document Contradiction Graphs (High Value, Medium Difficulty, Low Risk):** Expanding the contradiction engine to trace complex event timelines and logical inconsistencies across dozens of intersecting documents, providing a macro-view of massive litigation dossiers.  
2. **Human-in-the-Loop Review Queues (High Value, Low Difficulty, Low Risk):** Developing a gamified, high-speed user interface allowing investigators to rapidly bulk-approve or reject EvidenceSpans and model-suggested actor merges, dramatically accelerating workflow efficiency.  
3. **Commitment Tracking (Medium Value, Medium Difficulty, Low Risk):** Implementing a state-machine architecture to track the lifecycle of a promise from inception through fulfillment, modification, or breach across chronological document sets.  
4. **GraphRAG (Medium Value, High Difficulty, Medium Risk):** Utilizing the deterministically constructed property graph to constrain and augment contextual question-answering, allowing users to query the graph naturally while mitigating hallucination risks.  
5. **Temporal Reasoning (Medium Value, High Difficulty, Low Risk):** Integrating advanced interval algebra (e.g., Allen's interval algebra) to mathematically infer implicit event sequences when explicit timestamps are absent.

### **"Do Not Build Yet" List**

Certain capabilities present unacceptable trust and liability risks for the MVP phase and are explicitly restricted:

* **Predictive Settlement Path Generation / Mediation Strategy Support:** Deploying artificial intelligence to offer settlement advice or predict legal outcomes carries massive liability risks and crosses ethical boundaries.3 The system must remain an analytical tool, not an autonomous legal advisor.  
* **Automated Legal Issue Spotting:** Categorizing actions as specific torts or labor law violations approaches the unauthorized practice of law and risks severe compliance backlash from corporate legal departments.7  
* **Causal Graphs:** While academic causal loop diagrams are highly effective for systemic policy analysis 23, generating them autonomously from unstructured text introduces excessive ungrounded inference, corrupting the deterministic nature of the MVP.  
* **Vector Search / pgvector:** Over-indexing on pure semantic vector search dilutes the deterministic, typed primitive workflow that differentiates AGON from standard, easily commoditized RAG chatbots.

## **Technical Architecture**

AGON is architected to be highly performant, type-safe, and capable of executing complex graph traversals over relational data. The architecture relies on a Rust-first backend, leveraging PostgreSQL as a relational graph store, and is deployed via Cloud Run.

The core logic is divided into specialized Rust crates. The ingestion module handles document parsing, optical character recognition, and semantic chunking, utilizing zero-copy memory optimization to efficiently process large text buffers.5 The extraction module interfaces with external language models (Vertex/Gemini). It utilizes rigorous schemas via crates like instructors or struct-llm to enforce strict JSON outputs, managing asynchronous batch processing and implementing exponential retry backoff limits to handle API rate limiting.4 The verification module serves as the deterministic engine, executing the substring cross-referencing and fuzzy matching required to establish character offsets and instantiate EvidenceSpans. Finally, the graph module computes the Friction Matrix and resolves actor canonicalization via the Commit Gate logic.

While dedicated graph databases (such as Neo4j) excel at deep, multi-hop traversals, they introduce significant operational complexity and integration friction. For the AGON MVP, where traversals rarely exceed three degrees of separation (e.g., Actor to Claim to Contradiction to Actor), PostgreSQL is highly efficient and significantly easier to maintain.6 Graph nodes are stored in standard relational tables to enforce schema rigidity, while dynamic properties and edge metadata are stored in JSONB columns utilizing GIN indexes for rapid querying.6

The application programming interface exposes RESTful endpoints for document upload and session management, while utilizing GraphQL for flexible, complex graph queries by the frontend dashboard. To manage the latency inherent in large-scale language model extraction, the pipeline implements Server-Sent Events (SSE) to stream extraction progress to the user interface in real-time, preventing gateway timeouts on Cloud Run.25 To maintain backward compatibility with existing simple APIs, the newly typed internal structs implement TryFrom traits, gracefully mapping older, less structured payload shapes into the rigorous new ontology.

## **Data Model**

The PostgreSQL database schema is designed to enforce referential integrity and utilize source-hash uniqueness to prevent data duplication. The foundational tables include:

* **sessions**: id (UUID, Primary Key), name, created\_at, status.  
* **documents**: id, session\_id (Foreign Key), file\_hash (UNIQUE constraint to prevent duplicate ingestion), filename, content\_text, mime\_type.  
* **chunks**: id, document\_id (FK), start\_idx, end\_idx, text\_content, speaker\_label. A composite unique constraint on (document\_id, start\_idx, end\_idx) ensures chunk integrity.  
* **evidence\_spans**: id, chunk\_id (FK), start\_idx, end\_idx, exact\_text, provenance\_status (Enum: Verified, Suggested, Inferred).  
* **actors**: id, session\_id (FK), canonical\_name, role, type (Person, Organization).  
* **actor\_aliases**: id, actor\_id (FK), surface\_form, confidence\_score.  
* **claims**: id, actor\_id (FK), evidence\_span\_id (FK), statement\_summary.  
* **events**: id, session\_id (FK), evidence\_span\_id (FK), normalized\_timestamp (Timestamp with Timezone), description.  
* **commitments**: id, promising\_actor\_id (FK), receiving\_actor\_id (FK), evidence\_span\_id (FK), action\_description, deadline\_timestamp, status.  
* **contradictions**: id, type, claim\_a\_id (FK), claim\_b\_id (FK), resolution\_state (Open, Resolved).  
* **graph\_edges**: id, source\_node\_type, source\_node\_id, target\_node\_type, target\_node\_id, edge\_type (e.g., "denies", "supports", "accuses"), weight.  
* **patterns**: id, session\_id (FK), actor\_a\_id (FK), actor\_b\_id (FK), friction\_score, metadata (JSONB).  
* **reports**: id, session\_id (FK), generated\_summary, created\_at.

Comprehensive B-Tree indexes are applied to all foreign keys to ensure rapid traversal.

## **Evaluation and Benchmarks**

Testing a deterministic conflict engine requires evaluation methodologies that extend beyond standard retrieval-augmented generation metrics. Evaluation relies heavily on "Golden Conflict Examples." These are hand-crafted, ground-truth dossiers representing diverse, high-stakes contexts, including human resources complaints, commercial contract negotiations, and legal deposition samples.

Drawing inspiration from emerging legal contradiction datasets like CLAUSE and multilingual hallucination frameworks like Poly-FEVER 28, AGON will employ strict, multi-dimensional metrics. Actor extraction is evaluated via Recall and Precision, with a critical modification: false positive merges in canonicalization are penalized five times more heavily than false negatives, reflecting the conservative merge strategy.

Evidence Span correctness is measured via Exact Match (EM) percentage, calculating the ratio of extracted claims where the character offsets map perfectly to the human-annotated ground truth string in the source document. Contradiction quality is evaluated using a rigorous Confusion Matrix, tracking True Positives (correctly identifying a conflict) against False Positives, ensuring the system does not hallucinate conflicts where none exist.

Friction Matrix utility is evaluated qualitatively via a human evaluator rubric. Employee relations professionals and mediators will grade the generated matrix on a 1-to-5 scale, answering a primary heuristic question: *"Did this matrix accurately surface the primary interpersonal tension that drove the dispute, without introducing distracting or irrelevant noise?"*

## **MVP Implementation Plan**

The MVP development is structured as an aggressive, staged four-week implementation plan, moving from foundational persistence to a hardened, demo-ready application.

**Week 1: Typed Persistence and Evidence-Backed Extraction**

* *Deliverables:* PostgreSQL schema deployed; typed language model pipelines integrated; basic EvidenceSpan verification engine functional.  
* *Engineering Tasks:* Establish Cloud Run continuous integration pipelines. Create Rust traits for Vertex/Gemini API interactions. Implement the exact-match and fuzzy-match character offset algorithms.  
* *Tests:* Unit tests for semantic chunking logic and strict JSON schema enforcement.  
* *Demo Acceptance Criteria:* A user can upload a single text document, the system extracts claims, and the user can view verified character spans correctly indexed in the database.  
* *Risks:* Model formatting inconsistencies breaking JSON deserialization. Mitigated by struct-llm auto-repair capabilities.

**Week 2: Actor Canonicalization and Contradiction MVP**

* *Deliverables:* Evidence Graph and Commit Gate operational; language model-assisted contradiction detection functional.  
* *Engineering Tasks:* Build the Tier-0 and Tier-1 actor merging logic. Develop prompt chains for comparing Claims and deterministically outputting Contradictions.  
* *Tests:* Inject known alias sets into test data to verify the conservative merging policy. Test the contradiction engine against a subset of the Poly-FEVER dataset.29  
* *Demo Acceptance Criteria:* Uploading a transcript containing "Mr. Davis" and "John" results in conservative linking pending human review. The system successfully highlights two mutually exclusive claims.  
* *Risks:* Premature actor merging corrupting the graph. Mitigated by strict adherence to the Commit Gate thresholds.

**Week 3: Friction Matrix and Report Surfaces**

* *Deliverables:* Friction calculation engine online; web dashboard user interface displaying the Matrix, Actors, and Raw Evidence views.  
* *Engineering Tasks:* Implement the deterministic tension\_score algorithms. Build RESTful endpoints to serve the matrix JSON data. Create the frontend visual heat map components.  
* *Tests:* Verify matrix mathematics (e.g., ensuring 2 contradictions \+ 1 accusation equals the exact expected mathematical score).  
* *Demo Acceptance Criteria:* End-to-end processing of a three-document dossier results in a fully populated dashboard and an interactive, clickable Friction Matrix.  
* *Risks:* Frontend rendering performance issues with large graphs. Mitigated by paginating edge data.

**Week 4: Golden Examples, Evaluation, and Demo Hardening**

* *Deliverables:* Finalized benchmark report; hardened APIs; resolved edge cases.  
* *Engineering Tasks:* Run the 5 Golden Conflict Examples through the pipeline. Tune model temperatures and system prompts to eliminate residual hallucinations. Harden the streaming API against timeout limits.  
* *Tests:* Execute the human-in-the-loop rubric scoring sessions with domain experts.  
* *Demo Acceptance Criteria:* Flawless, sub-two-minute execution of the 5 targeted demo scenarios for external stakeholders.  
* *Risks:* Demonstration failure due to unpredicted input formatting. Mitigated by locking down demo datasets and thorough pre-computation caching.

## **Competitive and Adjacent Landscape**

The market landscape comprises several distinct adjacencies, none of which currently solve the exact problem AGON targets.

eDiscovery platforms (such as Relativity or Everlaw) are highly optimized for scale, enabling the searchability and chain-of-custody tracking of millions of documents during litigation.30 However, these platforms find documents; they do not extract structured relational intelligence from within the texts. They require armies of human reviewers to map the actual conflicts.

Human Resources Investigation Tools (such as HR Acuity) provide excellent case management, standardized reporting templates, and workflow guardrails.10 Yet, they function primarily as manual systems of record. Investigators must read transcripts and manually input findings. AGON differentiates itself by functioning as an automated extraction engine that could easily integrate into these platforms via API, accelerating the investigative phase.

Legal AI Assistants focus primarily on legal research, memo drafting, and open-ended generative question-answering. AGON differs radically by generating a deterministic, auditable ontology of facts rather than generating new narrative text, thereby avoiding the hallucination risks inherent in open-ended legal RAG systems.3

## **Critical Risks and Mitigations**

| Risk Factor | Description | Proposed Mitigation |
| :---- | :---- | :---- |
| **Hallucinated Claims** | Language models may invent claims or events not present in the source text. | The deterministic quote verification engine acts as a hard gate. If a claim lacks a verifiable EvidenceSpan, it is immediately dropped from the graph. |
| **Misattributed Actors** | Falsely merging identities (e.g., confusing two different "Johns") corrupts the conflict graph. | The Evidence Graph and Commit Gate strictly enforce conservative merging.19 It is mathematically preferable to maintain unmerged aliases pending human review than to automate a false positive merge. |
| **False Contradictions** | The system may flag two statements as contradictory when broader context allows both to be true. | Models only *suggest* contradictions; the UI explicitly flags them for human review. Prompts are heavily engineered to demand strict mutual exclusivity. |
| **Privacy and Security** | Ingesting sensitive HR and legal data risks severe data leaks or compliance breaches. | Enforce zero-retention policies on external model provider APIs. Implement strict row-level security within the PostgreSQL database to isolate tenant data. |
| **Bias in Interpretation** | Models may disproportionately assign friction to informal or colloquial language while ignoring polite but hostile corporate speak. | Base the Friction Matrix purely on structural object counts (number of contradictions, broken commitments) rather than relying heavily on subjective semantic sentiment analysis. |
| **Overstated Legal Conclusions** | The system inadvertently generates outputs that qualify as automated legal advice. | Strictly limit the ontology to factual, objective primitives (claims, events, dates). Exclude legal classification fields (e.g., "Tort", "Breach of Contract") from the database schema. |

## **5 Concrete Demo Scenarios**

**Scenario 1: The Timeline Disconnect (HR Investigation)**

* *Input:* Two HR interview transcripts and one email thread regarding a disputed employee termination.  
* *Processing:* AGON parses the text, isolates the actors, and extracts a sequence of events.  
* *Expected Output:* The system surfaces a TimelineConflict. Actor A claims the performance review occurred on March 1st. Actor B's email proves they were locked out of corporate systems on February 28th. The Friction Matrix glows red between A and B, driven exclusively by this temporal impossibility. Clicking the red node displays the precise chronological mismatch.

**Scenario 2: The Broken Commitment (Commercial Dispute)**

* *Input:* A 20-message email thread between a software vendor and a corporate client.  
* *Processing:* The intent parser identifies promissory language and maps temporal extraction to subsequent events.  
* *Expected Output:* AGON extracts a Commitment node ("I will deliver the source code by Friday"). It subsequently extracts an Event node ("Code not received as of Monday morning"). The UI links these two nodes into a "Broken Commitment" warning, citing the specific email timestamps and driving up the tension score.

**Scenario 3: The Accusation and Denial (Workplace Mediation)**

* *Input:* A formal written grievance document and an exported Slack channel history.  
* *Processing:* The system identifies hostile sentiment in the grievance and maps it to a specific denial statement in the chat log.  
* *Expected Output:* AGON maps an Accusation from the complaint ("Sarah yelled at me in the meeting") and a Denial from the Slack export ("I never raised my voice, ask anyone"). The UI presents a Contradiction Card, flagging the dispute for the mediator to address directly.

**Scenario 4: Actor Canonicalization Validation (Legal Deposition)**

* *Input:* A 100-page deposition transcript where the primary subject is referred to interchangeably as "Mr. Robertson", "David", and "the defendant".  
* *Processing:* The ingestion pipeline runs the text through the Commit Gate, searching for quasi-identifiers.  
* *Expected Output:* AGON's Commit Gate successfully identifies the explicit linkage between "David Robertson" and "the defendant" early in the transcript, confidently merging them into a single canonical Actor node. Subsequent claims made by "David" are correctly attributed to the canonical actor without duplicate nodes forming.

**Scenario 5: The Policy Obligation (Compliance Audit)**

* *Input:* An excerpt from the corporate employee handbook and a manager's internal email.  
* *Processing:* AGON extracts the normative rule from the handbook and checks extracted events against this rule.  
* *Expected Output:* AGON extracts an Obligation from the handbook ("All expenses over $500 require VP approval"). It simultaneously extracts an Admission from the manager's email ("I approved the $800 flight without asking the VP"). AGON immediately flags an Obligation Dispute, providing a direct link between the policy text and the admission.

## **10 Example Source Text Snippets and Extracted Primitives**

The following table demonstrates how AGON processes raw, unstructured snippets into deterministic primitives.

| Source Text Snippet | Extracted Primitives | Analytical Reasoning |
| :---- | :---- | :---- |
| *"I clearly told Mark on Tuesday that the server migration was delayed. He is lying if he says otherwise."* | **Actor:** Mark, Sender. **Claim:** Migration was delayed. **Accusation:** Mark is lying. **Event:** Communication on Tuesday. | The system extracts multiple overlapping nodes from a single chunk. The explicit phrase "lying" triggers an Accusation node, binding the sender to Mark with high friction. |
| *"Q: Did you authorize the payment? A: Absolutely not, I wasn't even in the office that week."* | **Actor:** Interviewer, Interviewee. **Denial:** Did not authorize payment. **Claim:** Was not in office. | The negative phrasing "Absolutely not" maps the Denial to the interviewer's question, while the justification forms an independent Claim that can be chronologically verified. |
| *"Can you promise to have the Q3 report done by 5 PM? Yes, I'm on it."* | **Actor:** User 1, User 2\. **Commitment:** Deliver Q3 report by 5 PM. | The intent parser recognizes "promise" and the affirmative "Yes," establishing a formal Commitment node with a strict temporal deadline. |
| *"According to company policy 4.2, contractors cannot access the main database."* | **Actor:** Company. **Obligation:** Contractors cannot access database. | The language "According to policy" creates an Obligation node that serves as a benchmark for evaluating subsequent actor behaviors. |
| *"Jane Doe, the Director of Marketing, has final say over the budget."* | **Actor:** Jane Doe (Alias: Director of Marketing). **PowerDynamic:** Final say over budget. | The system merges the name and the title, while the phrase "final say" establishes a hierarchical PowerDynamic edge, which acts as a multiplier in the Friction Matrix. |
| *"He said he sent the contract on the 4th. I checked my inbox, and nothing arrived until the 6th."* | **Claim 1 (3rd Party):** Sent on the 4th. **Claim 2:** Arrived on the 6th. **Contradiction:** Delivery timeline. | The system extracts the embedded third-party claim and contrasts it with the primary speaker's claim, generating an immediate Contradiction node. |
| *"I admit I lost my temper, but only because David kept interrupting me."* | **Actor:** Sender, David. **Admission:** Lost temper. **Accusation:** David interrupted. | The explicit phrase "I admit" creates a highly valuable Admission node, while the subsequent justification generates an Accusation edge against David. |
| *"We need the indemnity clause removed. It's a dealbreaker for our board."* | **Actor:** Negotiator, Board. **Interest/Need:** Remove clause. **EscalationSignal:** Dealbreaker. | In a mediation context, "need" maps to an underlying Interest, while "dealbreaker" is classified as an EscalationSignal, indicating a hard negotiation boundary. |
| *"My manager, Mr. Smith, continually assigns me the worst shifts as retaliation for my HR complaint."* | **Actor:** Complainant, Mr. Smith (Alias: Manager). **Accusation:** Retaliatory scheduling. | The explicit connection of an action ("assigns shifts") to a hostile motive ("retaliation") establishes a severe Accusation node and a high-friction RelationshipEdge. |
| *"Let's just split the difference at $50k and walk away today."* | **Actor:** Speaker. **ResolutionOpportunity:** Settle for $50k today. | The collaborative phrasing "split the difference" triggers a ResolutionOpportunity node, highlighting a potential off-ramp for mediators to exploit. |

#### **Works cited**

1. Legal Document Management Systems vs. eDiscovery Platforms \- Nextpoint, accessed May 11, 2026, [https://www.nextpoint.com/ediscovery-blog/legal-document-management-systems-vs-ediscovery-platforms/](https://www.nextpoint.com/ediscovery-blog/legal-document-management-systems-vs-ediscovery-platforms/)  
2. AI generated audit evidence: Rethinking trust and authenticity in the digital age, accessed May 11, 2026, [https://www.wolterskluwer.com/en/expert-insights/ai-generated-audit-evidence-rethinking-trust-authenticity](https://www.wolterskluwer.com/en/expert-insights/ai-generated-audit-evidence-rethinking-trust-authenticity)  
3. Hallucination‐Free? Assessing the Reliability of Leading AI Legal Research Tools \- Daniel E. Ho, accessed May 11, 2026, [https://dho.stanford.edu/wp-content/uploads/Legal\_RAG\_Hallucinations.pdf](https://dho.stanford.edu/wp-content/uploads/Legal_RAG_Hallucinations.pdf)  
4. struct-llm \- crates.io: Rust Package Registry, accessed May 11, 2026, [https://crates.io/crates/struct-llm](https://crates.io/crates/struct-llm)  
5. langextract-rust \- Lib.rs, accessed May 11, 2026, [https://lib.rs/crates/langextract-rust](https://lib.rs/crates/langextract-rust)  
6. Using PostgreSQL as a Graph Database: A Simple Approach for Beginners, accessed May 11, 2026, [https://kushankurdas.medium.com/using-postgresql-as-a-graph-database-a-simple-approach-for-beginners-c76d3bc9e82c](https://kushankurdas.medium.com/using-postgresql-as-a-graph-database-a-simple-approach-for-beginners-c76d3bc9e82c)  
7. The Legal Playbook for AI in HR: Five Practical Steps to Help Mitigate Your Risk, accessed May 11, 2026, [https://www.theemployerreport.com/2024/11/the-legal-playbook-for-ai-in-hr-five-practical-steps-to-help-mitigate-your-risk/](https://www.theemployerreport.com/2024/11/the-legal-playbook-for-ai-in-hr-five-practical-steps-to-help-mitigate-your-risk/)  
8. North America Workplace Conflict Management Solution Market Size, 2026-2033, accessed May 11, 2026, [https://www.coherentmarketinsights.com/industry-reports/north-america-workplace-conflict-management-solution-market](https://www.coherentmarketinsights.com/industry-reports/north-america-workplace-conflict-management-solution-market)  
9. Conflict Resolution Solutions Market Report 2026 \- Research and Markets, accessed May 11, 2026, [https://www.researchandmarkets.com/reports/5972857/conflict-resolution-solutions-market-report](https://www.researchandmarkets.com/reports/5972857/conflict-resolution-solutions-market-report)  
10. Best Workplace Investigation Software for Enterprises in 2026, accessed May 11, 2026, [https://www.hracuity.com/blog/best-investigation-software-for-enterprises-2026/](https://www.hracuity.com/blog/best-investigation-software-for-enterprises-2026/)  
11. Entity Resolution in Financial Crime: A Practical Guide to Uncover Hidden Risks \- DataWalk, accessed May 11, 2026, [https://datawalk.com/entity-resolution-in-financial-crime-a-practical-guide-to-uncover-hidden-risks/](https://datawalk.com/entity-resolution-in-financial-crime-a-practical-guide-to-uncover-hidden-risks/)  
12. Best Workplace Investigations Platform for 2026 | HR Acuity, accessed May 11, 2026, [https://www.hracuity.com/blog/best-workplace-investigations-platform-in-2026/](https://www.hracuity.com/blog/best-workplace-investigations-platform-in-2026/)  
13. Argument Interchange Format \- Wikipedia, accessed May 11, 2026, [https://en.wikipedia.org/wiki/Argument\_Interchange\_Format](https://en.wikipedia.org/wiki/Argument_Interchange_Format)  
14. AIF : Dialogue in the Argument Interchange Format \- Dr Simon Wells, accessed May 11, 2026, [https://www.simonwells.org/assets/papers/reed\_2008\_aif.plus.pdf](https://www.simonwells.org/assets/papers/reed_2008_aif.plus.pdf)  
15. The LKIF Core Ontology of Basic Legal Concepts \- CEUR-WS.org, accessed May 11, 2026, [https://ceur-ws.org/Vol-321/paper3.pdf](https://ceur-ws.org/Vol-321/paper3.pdf)  
16. An auditable and source-verified framework for clinical AI decision support: integrating retrieval-augmented generation with data provenance \- PMC, accessed May 11, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC12913532/](https://pmc.ncbi.nlm.nih.gov/articles/PMC12913532/)  
17. What Is Entity Resolution? How It Works & Why It Matters \- Senzing, accessed May 11, 2026, [https://senzing.com/what-is-entity-resolution/](https://senzing.com/what-is-entity-resolution/)  
18. The Impact of Automatic Speech Transcription on Speaker Attribution \- ACL Anthology, accessed May 11, 2026, [https://aclanthology.org/2025.tacl-1.72/](https://aclanthology.org/2025.tacl-1.72/)  
19. Early Detection of Re-Identification Risk in Multi-Turn ... \- Preprints.org, accessed May 11, 2026, [https://www.preprints.org/manuscript/202603.0209/v1/download](https://www.preprints.org/manuscript/202603.0209/v1/download)  
20. LegalWiz: A Multi-Agent Generation Framework for Contradiction Detection in Legal Documents \- arXiv, accessed May 11, 2026, [https://arxiv.org/html/2510.03418v2](https://arxiv.org/html/2510.03418v2)  
21. Theory and Practice of Coarse-Grained Molecular Dynamics of Biologically Important Systems \- MDPI, accessed May 11, 2026, [https://www.mdpi.com/2218-273X/11/9/1347](https://www.mdpi.com/2218-273X/11/9/1347)  
22. AI-Powered Coaching for Conflict Resolution | Personos Blog, accessed May 11, 2026, [https://www.personos.ai/post/ai-powered-coaching-for-conflict-resolution](https://www.personos.ai/post/ai-powered-coaching-for-conflict-resolution)  
23. Full article: Feedback density and causal complexity of simulation model structure, accessed May 11, 2026, [https://www.tandfonline.com/doi/full/10.1080/17477778.2021.1982653](https://www.tandfonline.com/doi/full/10.1080/17477778.2021.1982653)  
24. A Systems Thinking Approach to Algorithmic Fairness \- arXiv, accessed May 11, 2026, [https://arxiv.org/html/2412.16641v6](https://arxiv.org/html/2412.16641v6)  
25. instructors \- Rust \- Docs.rs, accessed May 11, 2026, [https://docs.rs/instructors](https://docs.rs/instructors)  
26. llm-toolkit \- crates.io: Rust Package Registry, accessed May 11, 2026, [https://crates.io/crates/llm-toolkit/0.43.2](https://crates.io/crates/llm-toolkit/0.43.2)  
27. Relational Databases vs. Graph Databases: What's the Difference? \- DataWalk, accessed May 11, 2026, [https://datawalk.com/relational-vs-graph-databases-difference/](https://datawalk.com/relational-vs-graph-databases-difference/)  
28. CLAUSE: A Discrepancy Benchmark for Auditing LLMs Legal Reasoning, accessed May 11, 2026, [https://clause-legal.github.io/](https://clause-legal.github.io/)  
29. Poly-FEVER: A Multilingual Fact Verification Benchmark for Hallucination Detection in Large Language Models \- arXiv, accessed May 11, 2026, [https://arxiv.org/html/2503.16541v2](https://arxiv.org/html/2503.16541v2)  
30. What is eDiscovery? \- OpenText, accessed May 11, 2026, [https://www.opentext.com/what-is/ediscovery](https://www.opentext.com/what-is/ediscovery)