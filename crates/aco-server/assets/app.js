const $ = (id) => document.getElementById(id);

const SAMPLES = {
  workplace: `AURORA PROGRAM INVESTIGATION PACKET — compiled 2026-04-18

Scope note:
People Team opened a review after the Aurora launch program missed the executive readout deadline and two senior contributors accused each other of misrepresenting ownership. The case file combines Slack, email, manager interviews, and rebuttals. The central question is not only who owned a deck, but whether ownership commitments are being converted, denied, or weaponized after handoffs.

Complaint from Sam Rivera, Product Marketing Lead, filed 2026-04-12:
Sam alleges that Alex Kim accepted ownership of the Q4 launch deck content on Monday, then denied the agreement after missing the Wednesday internal deadline. Sam says this is the third delivery ownership reversal in the quarter and that Alex has started telling colleagues Sam "sets people up to fail." Sam also alleges Mina Patel delayed design review because she relied on Alex's written commitment.

Slack export, #aurora-launch:
Sam (Mon 09:14): So we're agreed — you own the Q4 launch deck content, I handle design. Lock it in by Thursday?
Alex (Mon 09:47): Sounds good. I'll pick it up after the Jenkins pitch.
Mina (Mon 10:02): Great. I will hold design review until Alex sends content.
Priya (Mon 10:16): Please make sure exec narrative and customer proof are in one source of truth by Wed EOD.
Sam (Wed 17:02): Where are we on deck content? Designer needs it tomorrow morning.
Alex (Wed 17:31): What deck content? I thought you were doing the whole thing and I'd review.
Sam (Wed 17:45): No — Monday we agreed I do design, you do content. I have the messages.
Alex (Thu 09:02): I never said I'd own it. Just help. You're putting this on me last minute and now blaming me for not delivering something I never agreed to.
Mina (Thu 09:08): I paused design because I understood Alex had content. I am losing another day here.
Sam (Thu 09:14): Alex, this is the third time this quarter. We had it in writing. I have screenshots.
Alex (Thu 09:30): Whatever. Send what you have, I'll finish it tonight. But we need to talk about how you set me up to fail.

Email from Mina Patel to Sam, Alex, and Priya, Wed 18:10:
"The design review was moved because content was not ready. I understood from Monday's thread that Alex owned copy/content and Sam owned layout. If that changed, I was not told. I need one named owner because design cannot keep restarting from partial drafts."

Manager interview, Priya Shah, 2026-04-13:
Priya says Alex reported feeling "publicly blamed" by Sam. Priya also says Sam escalated to her only after the missed Wednesday checkpoint. Priya denies telling Alex that Sam was responsible for the full deck. Priya confirms she wrote "exec narrative and customer proof" in Slack but says she did not assign that work to either Alex or Sam directly. Priya says the team has begun copying her on routine handoffs, which she reads as a sign of collapsing trust.

Interview with Mina Patel, 2026-04-14:
Mina says she relied on the Monday exchange and put two designers on a different account because she expected content by Thursday. Mina says the dispute is affecting downstream capacity. Mina does not believe Sam invented the commitment, but she also says Alex often uses "I'll pick it up" to mean review, not ownership. Mina says the phrase "you own" was Sam's phrase, while "sounds good" was Alex's acceptance.

Alex response, 2026-04-15:
"I agreed to help after Jenkins, not to own the entire content package. Sam routinely converts casual help into formal ownership after the fact. Mina's note reflects her assumption, not my commitment. I said I would finish it Thursday night to stop the escalation, not because I accepted blame. Priya had asked for one source of truth, not for me to become the owner. Sam escalated after sitting on an empty deck for two days."

Sam rebuttal, 2026-04-16:
"Alex's distinction between help and ownership is new. The phrase 'you own the Q4 launch deck content' was explicit. Alex said 'sounds good.' Mina also relied on that. This is not public blame; it is accountability for a written commitment. Alex now says I sat on an empty deck, but the working file shows layout and outline were complete on Tuesday at 15:20."

Document note:
The working file history shows Sam created sections on Tuesday at 15:20, Mina entered comments Thursday at 11:05, and Alex added final narrative Thursday at 21:44. The final deck shipped Friday morning with an apology from Priya to Sales for the delay.`,

  legal: `VENDOR EXCEPTION CASE FILE — deposition, email, finance notes, and Slack, March 2026

Issue:
The company granted a non-standard vendor exception that bypassed the ordinary finance queue. The witness, Nick Hart, testified that finance had approved the exception before he saw it. Emails and Slack messages suggest a different sequence. Legal wants a contradiction map, not a prose summary.

Deposition of Nick Hart, VP Operations, 2026-03-18:
Q: Did you approve the vendor exception before finance reviewed it?
A: No. Finance had already signed off before I saw the request.
Q: Who told Legal the exception was approved?
A: I believe Finance did. I would not have represented approval if it had not happened.
Q: Did you ask Finance to hold review?
A: I do not recall asking Finance to hold anything. My practice is to let Finance run its process.
Q: Did you tell Lena Ortiz on March 14 that approval was already complete?
A: I do not remember that conversation. If I said it, I would have been referring to preliminary comfort, not formal approval.

Email from Nick Hart to Finance, 2026-03-14 09:22:
"Please hold the review until I confirm with Lena. We may need to route this outside the standard queue. Do not issue approval until I come back with the commercial context."

Finance reply, 2026-03-14 11:03:
"Understood. No approval has been issued yet. We will wait for your confirmation before routing."

Slack, Lena Ortiz to Legal Ops, 2026-03-14 13:48:
"Nick told me it was approved this morning and that Legal could paper it later. He said Finance was aligned but did not want to own the exception because the vendor is strategic."

Legal Ops note, 2026-03-14 16:10:
Lena asked whether Legal could prepare an exception letter without a finance approval ID. Legal Ops answered that the process requires approval first, unless the CFO grants an override.

CFO calendar note, 2026-03-15 08:30:
"Nick/Lena vendor exception — commercial pressure, no paper trail yet, need clean story before Monday."

Finance memorandum, 2026-03-16:
Finance states that approval was issued on March 16 at 12:04 after CFO review. The memo says Finance did not approve the exception on March 14 and did not authorize Legal to paper the exception before approval.

Nick Hart errata draft, unsent, 2026-03-19:
"My deposition answer should have distinguished formal approval from commercial alignment. I understood Finance was likely to approve, but formal approval may have been issued later."

Legal review note:
The contradiction may turn on whether Hart used "approved" to mean formal approval, preliminary comfort, or commercial alignment. The source chain also raises a process-control question: Hart told Finance to hold review, then allegedly told Lena that Finance was aligned.`,

  board: `BOARD GOVERNANCE DISPUTE — minutes, correspondence, and director objections

Background:
The board is disputing whether a strategic pivot toward commercial customers was validly approved at an October retreat, whether a later Q1 vote retroactively ratified the pivot, and whether the 72-hour board-pack rule is binding. Director Park argues process was manufactured after the fact. The Chair argues the board had practical consensus and Park is using process to relitigate strategy.

October Retreat Notes, 2025-10-19:
The Chair summarized three strategic options: enterprise commercial expansion, public-sector consolidation, and hybrid managed services. No formal resolution was read into the record. Director Ito stated that commercial expansion had "the clearest operating leverage." Director Park warned that the company lacked sales capacity for a commercial pivot and asked that any decision be deferred until the December budget session. The Chair closed by saying, "Management has enough directional support to model the commercial path."

Email from Director Park to the Chair, 2025-10-21:
"I want to be explicit that I did not vote for a commercial pivot at the retreat. I agreed management could model it. Please do not describe that as authorization."

Chair reply, 2025-10-22:
"Understood. I will describe it as directional support unless and until we take a formal vote."

BOARD MINUTES — Q1, March 18, 2026:
The Chair opened by confirming that the strategic pivot toward commercial customers had been approved unanimously at the October retreat. Director Park objected that the retreat had produced no formal resolution on customer segment. Director Ito moved to ratify the pivot retroactively; motion seconded by Director Chen. Director Park objected: "We cannot retroactively manufacture a vote we did not take." The Chair tabled the motion pending review of the October notes.

Board Secretary draft note, March 20, 2026:
"Minutes should avoid saying unanimous approval if October record shows directional support only. Recommendation: state that management interpreted the retreat as authorization and that at least one director disputes that interpretation."

BOARD MINUTES — Q2, June 12, 2026:
The Chair reported that Q1 results were ahead of plan. Director Ito noted the agenda would be distributed 72 hours in advance, consistent with the new governance protocol. Director Park stated she had received the pack the morning of the meeting and asked whether the 72-hour rule applied uniformly. The Chair responded that the rule was directional, not binding. Director Park noted for the record that she objected to a directional reading of a written protocol.

Email from Director Chen to the Chair, June 13, 2026:
"Park is technically right on the packet timing. We keep calling written procedures directional when they become inconvenient. That creates avoidable process risk."

Chair memo to Governance Committee, June 15, 2026:
"The commercial pivot has operational momentum and should not be held hostage to semantics. Park's objections are procedurally framed but substantively strategic. We need a clean ratification vote, but the company should not imply that management acted without board support."

Director Park memo, June 16, 2026:
"This is not semantics. The record shows a repeated pattern: directional discussion is later treated as authorization, written protocol is later treated as optional, and dissent is later characterized as strategy disagreement. That pattern undermines board governance."`,

  commercial: `ENTERPRISE RENEWAL NEGOTIATION — pricing, expansion, procurement authority, and face-saving

Context:
Atlas Cloud is negotiating renewal and expansion with Northstar Retail. The vendor needs a visible Q4 expansion logo. The buyer has a board-imposed per-seat ceiling and a procurement team worried about losing authority. The dispute looks like price, but the hidden friction is governance, precedent, and internal cover.

Call transcript, 2026-04-03:
Vendor AE Maya: We cannot move below list price on the expansion seats. It is a matter of principle at this stage of the quarter.
Buyer CIO Daniel: Then we will have to shelve the rollout. We committed internally to a per-seat cost ceiling and I cannot go above it.
Maya: A pilot could be structured differently. Smaller footprint, executive visibility, expansion option.
Daniel: Our board wants credible traction in this category before the next funding round. A tiny pilot reads as hesitation.
Maya: Let me be candid. If we discount this deal, every Q4 deal gets pulled in for the same treatment. That is our actual concern.
Daniel: And if we go above ceiling, procurement's authority is undermined for the next two cycles. Same shape of problem.
Maya: So we both need cover. What if the pilot price is at list but the expansion clause renegotiates from a lower base?
Daniel: That could work if procurement can say it preserved the ceiling.

Email from Northstar Procurement, 2026-04-04:
"The proposed list-price pilot is not acceptable if the commercial intent is already full rollout. We cannot endorse a structure that disguises expansion economics as a pilot. The board ceiling applies to the effective rollout price, not only to the first invoice."

Internal Slack, Atlas Sales, 2026-04-04:
Maya: Northstar procurement is blocking the pilot idea. They think it is a disguised discount.
VP Sales: We need the expansion in Q4. Do not put a discount in the order form. Use service credits if needed.
Maya: Credits may still count as economics.
VP Sales: They count only if procurement notices. Keep Legal out until we have business alignment.

Email from Atlas Legal, 2026-04-05:
"Please do not use side credits to alter effective price without reflecting them in the commercial summary. If credits are contingent on expansion, they are part of the economics and must be reviewed."

Northstar CFO note, 2026-04-06:
"Daniel can accept a phased rollout if procurement can show the effective price does not breach the board ceiling. We need vendor paper that does not embarrass the board process."

Follow-up call, 2026-04-07:
Daniel: Legal caught the service credit issue on your side. We need something cleaner.
Maya: We can do list price for pilot, board-approved expansion discount triggered only after security milestone, and a public reference commitment from Northstar.
Daniel: Reference commitment has value to you, not to procurement.
Maya: Correct. It gives us internal cover to justify lower expansion economics without calling it a pure discount.
Daniel: Send the paper. If procurement can explain it as milestone-based and the board ceiling is preserved, I can support it.

Open question:
Both sides deny trying to evade process, but both are searching for language that preserves authority while changing economics. The friction is less adversarial than structural: each party needs a defensible story for a different internal audience.`
};

(async function init() {
  try {
    const r = await fetch('/api/info');
    const info = await r.json();
    $('backend').textContent = info.backend + ' · ' + info.project + (info.db ? ' · 💾' : ' · stateless');
  } catch (_) {}
  loadHistory();
})();

document.getElementById('refresh-history')?.addEventListener('click', loadHistory);
document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => activateTab(tab.dataset.tab));
});

function activateTab(id) {
  document.querySelectorAll('.tab').forEach(t => t.classList.toggle('active', t.dataset.tab === id));
  document.querySelectorAll('.tab-pane').forEach(p => p.classList.toggle('active', p.id === id));
}

async function loadHistory() {
  try {
    const r = await fetch('/api/sessions');
    const data = await r.json();
    const el = $('history');
    if (!data.db) { el.innerHTML = '<div class="meta">db not configured (stateless)</div>'; return; }
    const list = data.sessions || [];
    if (!list.length) { el.innerHTML = '<div class="meta">no sessions yet — run one above</div>'; return; }
    el.innerHTML = list.map(s => `
      <div class="h-card" data-id="${s.id}">
        <div class="top">
          <span class="score ${s.friction_score >= 65 ? 'heat-high' : s.friction_score >= 35 ? 'heat-mid' : 'heat-low'}">${s.friction_score}</span>
          <span class="ts">${new Date(s.created_at).toLocaleString()}</span>
        </div>
        <div class="sum">${esc(s.summary || '(no summary)').slice(0, 140)}${(s.summary || '').length > 140 ? '…' : ''}</div>
        <div class="counts">${s.n_actors}a · ${s.n_claims}c · ${s.n_events}e · ${s.n_patterns}p · ${s.n_contradictions}x · ${s.input_tokens}/${s.output_tokens}tok · ${(s.elapsed_ms / 1000).toFixed(1)}s</div>
      </div>
    `).join('');
    el.querySelectorAll('.h-card').forEach(card => {
      card.addEventListener('click', async () => {
        const id = card.dataset.id;
        const r = await fetch('/api/sessions/' + id);
        if (!r.ok) return;
        const sess = await r.json();
        $('text').value = sess.source_text;
        render(sess.extraction);
        window.scrollTo({ top: 0, behavior: 'smooth' });
      });
    });
  } catch (e) {
    console.error(e);
  }
}

document.getElementById('sample-select')?.addEventListener('change', (e) => {
  const k = e.target.value;
  if (k && SAMPLES[k]) $('text').value = SAMPLES[k];
});

$('run').addEventListener('click', async () => {
  const text = $('text').value.trim();
  if (!text) { setStatus('paste some text first', true); return; }
  const btn = $('run');
  btn.disabled = true;
  $('report-link')?.classList.add('hidden');
  setStatus('streaming…');
  resetPipeline();
  const t0 = performance.now();
  try {
    const model = document.getElementById('model-select')?.value || 'flash';
    const res = await fetch('/api/perceive/stream', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ text, model })
    });
    if (!res.ok) { throw new Error(await res.text() || res.statusText); }
    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let buf = '';
    let done = false;
    while (!done) {
      const { value, done: d } = await reader.read();
      done = d;
      if (value) buf += decoder.decode(value, { stream: true });
      let i;
      while ((i = buf.indexOf('\n\n')) >= 0) {
        const frame = buf.slice(0, i); buf = buf.slice(i + 2);
        handleSseFrame(frame);
        $('pipe-total').textContent = ((performance.now() - t0) / 1000).toFixed(2) + ' s';
      }
    }
  } catch (e) {
    setStatus('error: ' + e.message, true);
    addStep('error', e.message, 'error');
  } finally {
    btn.disabled = false;
  }
});

function resetPipeline() {
  $('pipeline').classList.remove('hidden');
  $('pipe-steps').innerHTML = '';
  $('pipe-total').textContent = '0 ms';
}

function handleSseFrame(frame) {
  let evt = 'message', data = '';
  for (const line of frame.split('\n')) {
    if (line.startsWith('event:')) evt = line.slice(6).trim();
    else if (line.startsWith('data:')) data += line.slice(5).trim();
  }
  if (!data) return;
  let payload;
  try { payload = JSON.parse(data); } catch (_) { return; }

  if (evt === 'stage') {
    markPriorDone();
    const meta = formatMeta(payload);
    addStep(payload.stage, payload.msg || '', 'active', meta, payload.elapsed_ms);
  } else if (evt === 'result') {
    markPriorDone();
    addStep('result', 'world model built', 'done', '', payload.elapsed_ms);
    setStatus(`${payload.extraction.actors?.length || 0} actors · ${payload.input_tokens}/${payload.output_tokens} tokens · ${(payload.elapsed_ms/1000).toFixed(1)}s · ${payload.model}${payload.persisted ? ' · 💾' : ''}`);
    setReportLink(payload.session_id);
    render(payload.extraction);
    renderHeatmap(payload.friction_matrix, payload.extraction);
    renderPreCanon(payload.pre_canonical);
    loadHistory();
  } else if (evt === 'error') {
    markPriorDone();
    addStep('error', payload.error, 'error');
    setStatus('error: ' + payload.error, true);
  } else if (evt === 'warn') {
    addStep('warn', payload.warn, 'error');
  }
}

function setReportLink(sessionId) {
  const link = $('report-link');
  if (!link) return;
  if (!sessionId) {
    link.classList.add('hidden');
    link.removeAttribute('href');
    return;
  }
  link.href = `/api/sessions/${sessionId}/report.md`;
  link.classList.remove('hidden');
}

function markPriorDone() {
  const last = $('pipe-steps').lastElementChild;
  if (last && last.classList.contains('active')) {
    last.classList.remove('active');
    last.classList.add('done');
  }
}

function addStep(name, msg, cls, meta, ms) {
  const li = document.createElement('li');
  if (cls) li.classList.add(cls);
  const span = (c, t) => `<span class="${c}">${esc(t)}</span>`;
  li.innerHTML =
    span('name', name) +
    `<span class="meta">${esc(msg || '')}${meta ? ' · ' + meta : ''}</span>` +
    `<span class="ms">${ms != null ? ms + ' ms' : ''}</span>`;
  $('pipe-steps').appendChild(li);
}

function formatMeta(p) {
  const bits = [];
  if (p.chars != null) bits.push(`${p.chars} chars · ${p.lines} lines`);
  if (p.input_tokens != null) bits.push(`${p.input_tokens} in / ${p.output_tokens} out`);
  if (p.tokens_per_sec != null && p.tokens_per_sec > 0) bits.push(`${p.tokens_per_sec.toFixed(0)} tok/s`);
  if (p.model) bits.push(p.model);
  if (p.backend) bits.push(p.backend);
  if (p.friction_score != null) bits.push(`friction ${p.friction_score}`);
  if (p.n_actors != null) bits.push(`${p.n_actors}a/${p.n_claims}c/${p.n_events}e/${p.n_patterns}p/${p.n_contradictions}x`);
  if (p.session_id) bits.push(`id ${String(p.session_id).slice(0,8)}…`);
  return bits.join(' · ');
}

function setStatus(msg, err) {
  const el = $('status');
  el.textContent = msg;
  el.className = 'status' + (err ? ' err' : '');
}

function render(x) {
  $('results').classList.remove('hidden');
  activateTab('overview');
  const friction = x.friction_score ?? 0;
  const fe = $('friction');
  fe.textContent = friction;
  fe.className = 'big ' + (friction >= 65 ? 'heat-high' : friction >= 35 ? 'heat-mid' : 'heat-low');

  $('n-actors').textContent = (x.actors || []).length;
  $('n-claims').textContent = (x.claims || []).length;
  $('n-events').textContent = (x.events || []).length;
  $('n-patterns').textContent = (x.patterns || []).length;
  $('n-contras').textContent = (x.contradictions || []).length;
  $('summary').textContent = x.summary || '(no summary)';
  renderCaseTelemetry(x);
  renderChannelBars(x);
  renderLens(x);

  $('actors').innerHTML = (x.actors || []).map(a => `
    <div class="item">
      <span class="lbl">${esc(a.label || a.id)}</span>
      ${a.kind ? `<span class="tag">${esc(a.kind)}</span>` : ''}
      ${a.role ? `<span class="tag">${esc(a.role)}</span>` : ''}
      ${a.aliases?.length ? `<div class="meta">aka: ${a.aliases.map(esc).join(', ')}</div>` : ''}
      ${evidenceBadge(a.evidence)}
      ${a.evidence ? `<span class="ev">"${esc(a.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('claims').innerHTML = (x.claims || []).map(c => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, c.actor_id))}:</span>
      ${esc(c.text)}
      ${c.polarity ? `<span class="tag">${esc(c.polarity)}</span>` : ''}
      ${c.subject_actor_id ? `<span class="tag">re: ${esc(actorName(x, c.subject_actor_id))}</span>` : ''}
      ${evidenceBadge(c.evidence)}
      ${c.evidence ? `<span class="ev">"${esc(c.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('events').innerHTML = (x.events || []).map(e => `
    <div class="item">
      <span class="lbl">${esc(e.label)}</span>
      ${e.when ? `<span class="meta">· ${esc(e.when)}</span>` : ''}
      ${evidenceBadge(e.evidence)}
      ${e.evidence ? `<span class="ev">"${esc(e.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('commitments').innerHTML = (x.commitments || []).map(c => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, c.by_actor))}${c.to_actor ? ' → ' + esc(actorName(x, c.to_actor)) : ' →'}</span> ${esc(c.subject)}
      ${c.deadline ? `<span class="meta">· due ${esc(c.deadline)}</span>` : ''}
      <span class="tag ${statusTag(c.status)}">${esc(c.status)}</span>
      ${evidenceBadge(c.evidence)}
      ${c.evidence ? `<span class="ev">"${esc(c.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('interests').innerHTML = (x.interests || []).map(i => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, i.actor_id))}:</span>
      ${esc(i.interest)}
      ${i.rationale ? `<div class="meta">${esc(i.rationale)}</div>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('patterns').innerHTML = (x.patterns || []).map(p => `
    <div class="item">
      <span class="lbl">${esc(p.kind)}</span>
      <span class="tag ${heat(p.confidence)}">${(p.confidence ?? 0).toFixed(2)}</span>
      ${actorName(x, p.actor_id) ? `<span class="meta">· ${esc(actorName(x, p.actor_id))}</span>` : ''}
      ${evidenceBadge(p.evidence)}
      ${p.evidence ? `<span class="ev">"${esc(p.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('contras').innerHTML = (x.contradictions || []).map(c => {
    const a = (x.claims || []).find(cl => cl.id === c.claim_a);
    const b = (x.claims || []).find(cl => cl.id === c.claim_b);
    return `
    <div class="item contradiction-card">
      <span class="tag ${c.materiality}">${esc(c.materiality)}</span>
      ${c.source ? `<span class="tag">${esc(c.source)}</span>` : '<span class="tag">model_suggested</span>'}
      ${c.confidence != null ? `<span class="tag ${heat(c.confidence)}">${Number(c.confidence).toFixed(2)}</span>` : ''}
      <div class="claim-pair">
        <div><strong>A · ${esc(actorName(x, a?.actor_id) || 'unknown')}</strong><p>${esc(a?.text || c.claim_a)}</p>${a?.evidence ? `<span class="ev">"${esc(a.evidence)}"</span>` : ''}</div>
        <div><strong>B · ${esc(actorName(x, b?.actor_id) || 'unknown')}</strong><p>${esc(b?.text || c.claim_b)}</p>${b?.evidence ? `<span class="ev">"${esc(b.evidence)}"</span>` : ''}</div>
      </div>
      ${c.rationale ? `<div class="meta" style="margin-top:6px">${esc(c.rationale)}</div>` : ''}
    </div>`;
  }).join('') || '<div class="meta">none</div>';

  renderRelationships(x);
  renderPowerDynamics(x);
  renderEscalation(x);
  renderResolution(x);
  renderEvidenceLedger(x);
  renderEvidenceCoverage(x);
  renderTimeline(x);
  renderInferenceStack(x);
  renderActorLanes(x);
  drawRelationshipMap(x);

  $('raw').textContent = JSON.stringify(x, null, 2);
  drawGraph(x);
}

function renderLens(x) {
  const el = $('lens');
  if (!el) return;
  const p = x.document_profile || {};
  const neural = x.neural_signals || {};
  const gates = x.quality_gates || [];
  const questions = x.review_questions || [];
  const notes = p.reading_notes || [];
  const signals = neural.signals || [];
  el.innerHTML = `
    <div class="lens-grid">
      <div><span class="meta">format</span><strong>${esc(p.format || 'unknown')}</strong></div>
      <div><span class="meta">density</span><strong>${Number(p.conflict_density || 0).toFixed(2)}</strong></div>
      <div><span class="meta">segments</span><strong>${(p.segments || []).length}</strong></div>
      <div><span class="meta">neural signals</span><strong>${signals.length}</strong></div>
    </div>
    <div class="lens-section">
      ${(notes.length ? notes : ['No pre-reading notes emitted']).map(n => `<div class="meta">• ${esc(n)}</div>`).join('')}
    </div>
    <div class="lens-section">
      <h4>Quality gates</h4>
      ${gates.map(g => `<span class="tag ${esc(g.status || 'review')}">${esc(g.label || g.id)} · ${Number(g.score || 0).toFixed(2)}</span>`).join('') || '<span class="meta">none</span>'}
    </div>
    <div class="lens-section">
      <h4>Review questions</h4>
      ${(questions.length ? questions : ['What additional source text would reduce uncertainty?']).map(q => `<div class="item compact">${esc(q)}</div>`).join('')}
    </div>
  `;
}

function renderCaseTelemetry(x) {
  const el = $('case-telemetry');
  if (!el) return;
  const p = x.document_profile || {};
  const counts = [
    ['actors', (x.actors || []).length],
    ['claims', (x.claims || []).length],
    ['events', (x.events || []).length],
    ['commitments', (x.commitments || []).length],
    ['relationships', (x.relationships || []).length],
    ['evidence quotes', countEvidenceQuotes(x)]
  ];
  el.innerHTML = `
    <div class="telemetry-grid">
      ${counts.map(([k, v]) => `<div><span>${esc(k)}</span><strong>${v}</strong></div>`).join('')}
    </div>
    <div class="mini-read">
      <span class="tag">${esc(p.format || 'unknown')}</span>
      <span class="tag ${Number(p.conflict_density || 0) > 0.45 ? 'high' : Number(p.conflict_density || 0) > 0.2 ? 'med' : 'low'}">density ${Number(p.conflict_density || 0).toFixed(2)}</span>
      <span class="tag">${(p.temporal_markers || []).length} temporal markers</span>
      <span class="tag">${(p.modality_markers || []).length} modality markers</span>
    </div>
  `;
}

function renderChannelBars(x) {
  const el = $('channel-bars');
  if (!el) return;
  const channels = [
    ['claim pressure', (x.claims || []).length],
    ['contradiction', (x.contradictions || []).length * 3],
    ['commitment risk', (x.commitments || []).filter(c => ['contested', 'broken', 'unclear'].includes(String(c.status || '').toLowerCase())).length * 2],
    ['relationship pressure', (x.relationships || []).length],
    ['escalation', (x.escalation_signals || []).reduce((s, e) => s + Number(e.intensity || 1), 0)],
    ['unresolved evidence', unresolvedEvidenceRows(x).length * 2]
  ];
  const max = Math.max(1, ...channels.map(([, v]) => v));
  el.innerHTML = channels.map(([label, value]) => {
    const pct = Math.max(4, Math.round((value / max) * 100));
    return `
      <div class="bar-row">
        <span>${esc(label)}</span>
        <div><i style="width:${pct}%"></i></div>
        <b>${Number(value).toFixed(value % 1 ? 1 : 0)}</b>
      </div>
    `;
  }).join('');
}

function renderTimeline(x) {
  const el = $('timeline');
  if (!el) return;
  const events = [
    ...(x.events || []).map(e => ({ kind: 'event', when: e.when || '', label: e.label || e.id, evidence: e.evidence })),
    ...(x.commitments || []).filter(c => c.deadline).map(c => ({ kind: 'commitment', when: c.deadline, label: c.subject, evidence: c.evidence })),
  ];
  events.sort((a, b) => String(a.when).localeCompare(String(b.when)));
  el.innerHTML = events.length ? events.slice(0, 14).map(e => `
    <div class="timeline-row">
      <span class="when">${esc(e.when || 'undated')}</span>
      <div>
        <span class="tag">${esc(e.kind)}</span>
        <strong>${esc(e.label)}</strong>
        ${e.evidence ? `<em>"${esc(e.evidence)}"</em>` : ''}
      </div>
    </div>
  `).join('') : '<div class="meta">no dated events or deadlines extracted</div>';
}

function renderInferenceStack(x) {
  const el = $('inference-stack');
  if (!el) return;
  const findings = x.inferences || [];
  const gates = x.quality_gates || [];
  const neural = x.neural_signals?.signals || [];
  const questions = x.review_questions || [];
  el.innerHTML = `
    <div class="stack-metrics">
      <div><span>findings</span><strong>${findings.length}</strong></div>
      <div><span>gates</span><strong>${gates.length}</strong></div>
      <div><span>signals</span><strong>${neural.length}</strong></div>
      <div><span>questions</span><strong>${questions.length}</strong></div>
    </div>
    ${findings.slice(0, 6).map(f => `
      <div class="item compact">
        <span class="tag ${heat(Number(f.confidence || f.score || 0))}">${esc(f.kind || 'finding')}</span>
        <span class="lbl">${esc(f.label || f.summary || 'inference')}</span>
        <div class="meta">${esc(f.rationale || f.explanation || '')}</div>
      </div>
    `).join('') || '<div class="meta">no deterministic inference findings emitted for this case</div>'}
  `;
}

function renderActorLanes(x) {
  const el = $('actor-lanes');
  if (!el) return;
  const actors = x.actors || [];
  el.innerHTML = actors.length ? actors.map(a => {
    const claims = (x.claims || []).filter(c => c.actor_id === a.id);
    const commitments = (x.commitments || []).filter(c => c.by_actor === a.id || c.to_actor === a.id);
    const rels = (x.relationships || []).filter(r => r.from_actor === a.id || r.to_actor === a.id);
    const escalations = (x.escalation_signals || []).filter(e => e.actor_id === a.id);
    return `
      <div class="lane">
        <div class="lane-head">
          <strong>${esc(a.label || a.id)}</strong>
          ${a.role ? `<span class="tag">${esc(a.role)}</span>` : ''}
        </div>
        <div class="lane-counts">
          <span>${claims.length} claims</span>
          <span>${commitments.length} commitments</span>
          <span>${rels.length} relationships</span>
          <span>${escalations.length} escalations</span>
        </div>
        ${claims.slice(0, 3).map(c => `<div class="mini-claim">${esc(trunc(c.text, 130))}</div>`).join('')}
      </div>
    `;
  }).join('') : '<div class="meta">no actors extracted</div>';
}

function drawRelationshipMap(x) {
  const svg = $('relationship-map');
  if (!svg) return;
  svg.innerHTML = '';
  const actors = x.actors || [];
  if (!actors.length) return;
  const W = 1200, H = 360, cx = W / 2, cy = H / 2, r = Math.min(W, H) * 0.36;
  const pos = {};
  actors.forEach((a, i) => {
    const ang = (i / actors.length) * Math.PI * 2 - Math.PI / 2;
    pos[a.id] = { x: cx + Math.cos(ang) * r, y: cy + Math.sin(ang) * r, label: a.label || a.id };
  });
  const rels = x.relationships || [];
  const contraPairs = (x.contradictions || []).map(c => {
    const ca = (x.claims || []).find(cl => cl.id === c.claim_a);
    const cb = (x.claims || []).find(cl => cl.id === c.claim_b);
    return ca?.actor_id && cb?.actor_id ? { from_actor: ca.actor_id, to_actor: cb.actor_id, type: 'contradiction', weight: 3 } : null;
  }).filter(Boolean);
  for (const rel of [...rels, ...contraPairs]) {
    const a = pos[rel.from_actor], b = pos[rel.to_actor];
    if (!a || !b || rel.from_actor === rel.to_actor) continue;
    const line = svgEl('line', { x1: a.x, y1: a.y, x2: b.x, y2: b.y });
    line.setAttribute('class', `rel-line ${String(rel.type || '').toLowerCase().includes('contrad') ? 'contradict' : ''}`);
    line.setAttribute('stroke-width', Math.min(5, Math.max(1.5, Number(rel.weight || 1.5))));
    svg.appendChild(line);
  }
  for (const a of actors) {
    const p = pos[a.id];
    const c = svgEl('circle', { cx: p.x, cy: p.y, r: 18 });
    c.setAttribute('class', 'rel-node');
    svg.appendChild(c);
    const t = svgEl('text', { x: p.x, y: p.y + 36, 'text-anchor': 'middle' });
    t.textContent = trunc(p.label, 22);
    svg.appendChild(t);
  }
}

function renderEvidenceCoverage(x) {
  const el = $('evidence-coverage');
  if (!el) return;
  const rows = allEvidenceRows(x);
  const verified = rows.filter(r => r.verified).length;
  const unresolved = rows.length - verified;
  const pct = rows.length ? Math.round((verified / rows.length) * 100) : 0;
  const unresolvedRows = rows.filter(r => !r.verified).slice(0, 8);
  el.innerHTML = `
    <div class="coverage-ring" style="--pct:${pct}">
      <strong>${pct}%</strong><span>verified</span>
    </div>
    <div class="coverage-detail">
      <span class="tag low">${verified} verified</span>
      <span class="tag material">${unresolved} unresolved</span>
      ${unresolvedRows.map(r => `<div class="meta">unresolved ${esc(r.kind)} · ${esc(trunc(r.label, 120))}</div>`).join('')}
    </div>
  `;
}

// Simple force-directed graph. No CDN.
function drawGraph(x) {
  const svg = document.getElementById('graph');
  svg.innerHTML = '';
  const W = 1200, H = 480;
  const nodes = [];
  const links = [];

  (x.actors || []).forEach(a => nodes.push({ id: a.id, label: a.label || a.id, kind: 'actor' }));
  (x.claims || []).forEach(c => {
    nodes.push({ id: c.id, label: trunc(c.text, 28), kind: 'claim' });
    if (c.actor_id) links.push({ s: c.actor_id, t: c.id, type: 'ASSERTED' });
  });
  (x.events || []).forEach(e => nodes.push({ id: e.id, label: trunc(e.label, 28), kind: 'event' }));
  (x.commitments || []).forEach(c => {
    nodes.push({ id: c.id, label: trunc(c.subject, 28), kind: 'commitment' });
    if (c.by_actor) links.push({ s: c.by_actor, t: c.id, type: 'COMMITS_TO' });
  });
  (x.interests || []).forEach((i, idx) => {
    const id = 'int_' + idx;
    nodes.push({ id, label: trunc(i.interest, 28), kind: 'interest' });
    if (i.actor_id) links.push({ s: i.actor_id, t: id, type: 'HOLDS_INTEREST' });
  });
  (x.contradictions || []).forEach(c => links.push({ s: c.claim_a, t: c.claim_b, type: 'CONTRADICTS' }));

  if (!nodes.length) return;

  // Init positions on a circle
  const cx = W / 2, cy = H / 2, R = Math.min(W, H) * 0.4;
  nodes.forEach((n, i) => {
    const a = (i / nodes.length) * Math.PI * 2;
    n.x = cx + Math.cos(a) * R;
    n.y = cy + Math.sin(a) * R;
    n.vx = 0; n.vy = 0;
  });
  const byId = Object.fromEntries(nodes.map(n => [n.id, n]));
  const linksResolved = links.filter(l => byId[l.s] && byId[l.t]);

  // Run physics: charge repulsion + spring links + center gravity.
  for (let step = 0; step < 220; step++) {
    for (let i = 0; i < nodes.length; i++) for (let j = i + 1; j < nodes.length; j++) {
      const a = nodes[i], b = nodes[j];
      const dx = b.x - a.x, dy = b.y - a.y;
      const d2 = dx * dx + dy * dy + 1;
      const f = 5000 / d2;
      const fx = (dx / Math.sqrt(d2)) * f;
      const fy = (dy / Math.sqrt(d2)) * f;
      a.vx -= fx; a.vy -= fy; b.vx += fx; b.vy += fy;
    }
    for (const l of linksResolved) {
      const a = byId[l.s], b = byId[l.t];
      const dx = b.x - a.x, dy = b.y - a.y;
      const d = Math.sqrt(dx * dx + dy * dy) || 1;
      const f = (d - 110) * 0.04;
      const fx = (dx / d) * f, fy = (dy / d) * f;
      a.vx += fx; a.vy += fy; b.vx -= fx; b.vy -= fy;
    }
    for (const n of nodes) {
      n.vx += (cx - n.x) * 0.005;
      n.vy += (cy - n.y) * 0.005;
      n.x += n.vx * 0.5; n.y += n.vy * 0.5;
      n.vx *= 0.7; n.vy *= 0.7;
      n.x = Math.max(40, Math.min(W - 40, n.x));
      n.y = Math.max(30, Math.min(H - 30, n.y));
    }
  }

  // Draw links
  for (const l of linksResolved) {
    const a = byId[l.s], b = byId[l.t];
    const line = svgEl('line', { x1: a.x, y1: a.y, x2: b.x, y2: b.y });
    line.setAttribute('class', 'link ' + l.type);
    svg.appendChild(line);
  }
  // Draw nodes
  for (const n of nodes) {
    const r = n.kind === 'actor' ? 10 : 6;
    const c = svgEl('circle', { cx: n.x, cy: n.y, r });
    c.setAttribute('class', 'node-' + n.kind);
    svg.appendChild(c);
    const t = svgEl('text', { x: n.x + r + 3, y: n.y + 3 });
    t.textContent = n.label;
    svg.appendChild(t);
  }
}
function renderHeatmap(m, x) {
  const el = document.getElementById('heatmap');
  if (!el) return;
  const actors = (x.actors || []);
  if (!actors.length) { el.innerHTML = '<div class="meta">no actors</div>'; return; }
  const byId = Object.fromEntries(actors.map(a => [a.id, a.label || a.id]));
  const pairWeight = {};
  for (const p of (m?.pairs || [])) {
    const k = [p.a, p.b].sort().join('|');
    pairWeight[k] = p.weight;
  }
  const ids = actors.map(a => a.id);
  let html = '<table><thead><tr><th></th>';
  for (const id of ids) html += `<th>${esc(trunc(byId[id], 12))}</th>`;
  html += '</tr></thead><tbody>';
  for (const a of ids) {
    html += `<tr><th>${esc(trunc(byId[a], 12))}</th>`;
    for (const b of ids) {
      if (a === b) { html += '<td class="cell" style="background:var(--line);">—</td>'; continue; }
      const w = pairWeight[[a, b].sort().join('|')] || 0;
      const intensity = Math.min(1, w / 5);
      const bg = `rgba(248, 113, 113, ${intensity * 0.85})`;
      const fg = intensity > 0.4 ? '#fff' : 'var(--muted)';
      html += `<td class="cell" style="background:${bg}; color:${fg};">${w.toFixed(1)}</td>`;
    }
    html += '</tr>';
  }
  html += '</tbody></table>';
  el.innerHTML = html;
  renderFrictionDrivers(m);
}

function renderFrictionDrivers(m) {
  const el = $('friction-drivers');
  if (!el) return;
  const pairs = [...(m?.pairs || [])].sort((a, b) => (b.weight || 0) - (a.weight || 0));
  el.innerHTML = pairs.length ? pairs.map(p => `
    <div class="item">
      <span class="lbl">${esc(p.a_label)} ↔ ${esc(p.b_label)}</span>
      <span class="tag ${p.weight >= 3 ? 'high' : p.weight >= 1 ? 'med' : 'low'}">${Number(p.weight || 0).toFixed(1)}</span>
      <div class="meta">${(p.reasons || []).slice(0, 6).map(esc).join(' · ') || 'derived pressure'}</div>
    </div>
  `).join('') : '<div class="meta">no pair pressure yet</div>';
}

function renderPreCanon(pc) {
  const el = document.getElementById('precanon');
  if (!el || !pc) return;
  const speakers = (pc.speakers || []).map(s => `<span class="spk">${esc(s)}</span>`).join('');
  el.innerHTML = `
    <div class="row"><span class="k">format</span><span>${esc(pc.format_hint)}</span></div>
    <div class="row"><span class="k">turns</span><span>${pc.n_turns} / lines ${pc.n_lines} / chars ${pc.n_chars}</span></div>
    <div class="row"><span class="k">speakers</span><span>${speakers || '<span class="meta">(none — prose)</span>'}</span></div>
  `;
}

function renderRelationships(x) {
  const el = $('relationships');
  if (!el) return;
  el.innerHTML = (x.relationships || []).map(r => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, r.from_actor))} → ${esc(actorName(x, r.to_actor))}</span>
      <span class="tag">${esc(r.type || 'relationship')}</span>
      ${r.weight != null ? `<span class="tag ${Number(r.weight) >= 2 ? 'high' : 'med'}">${Number(r.weight).toFixed(1)}</span>` : ''}
      ${evidenceBadge(r.evidence)}
      ${r.evidence ? `<span class="ev">"${esc(r.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none extracted</div>';
}

function renderPowerDynamics(x) {
  const el = $('power-dynamics');
  if (!el) return;
  el.innerHTML = (x.power_dynamics || []).map(p => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, p.dominant_actor))} over ${esc(actorName(x, p.subordinate_actor))}</span>
      ${p.confidence != null ? `<span class="tag ${heat(p.confidence)}">${Number(p.confidence).toFixed(2)}</span>` : ''}
      <div>${esc(p.basis || '')}</div>
      ${evidenceBadge(p.evidence)}
      ${p.evidence ? `<span class="ev">"${esc(p.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none extracted</div>';
}

function renderEscalation(x) {
  const el = $('escalation');
  if (!el) return;
  el.innerHTML = (x.escalation_signals || []).map(e => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, e.actor_id))}</span>
      <span class="tag ${Number(e.intensity || 0) >= 4 ? 'high' : 'med'}">intensity ${esc(e.intensity || '?')}</span>
      <div>${esc(e.trigger || '')}</div>
      ${evidenceBadge(e.evidence)}
      ${e.evidence ? `<span class="ev">"${esc(e.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none extracted</div>';
}

function renderResolution(x) {
  const el = $('resolution');
  if (!el) return;
  el.innerHTML = (x.resolution_opportunities || []).map(o => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, o.actor_id))}</span>
      <div>${esc(o.opening || '')}</div>
      ${evidenceBadge(o.evidence)}
      ${o.evidence ? `<span class="ev">"${esc(o.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none extracted</div>';
}

function renderEvidenceLedger(x) {
  const source = $('text').value || '';
  const rows = Array.isArray(x.evidence_audit) ? x.evidence_audit.map(r => ({
    kind: r.kind,
    label: r.label,
    quote: r.quote,
    verified: r.status === 'verified'
  })) : [];
  const push = (kind, label, quote) => {
    if (!quote) return;
    const verified = source.includes(quote);
    rows.push({ kind, label, quote, verified });
  };
  if (rows.length) {
    $('evidence-ledger').innerHTML = rows.map(r => `
      <div class="item evidence-row">
        <span class="tag">${esc(r.kind)}</span>
        <span class="tag ${r.verified ? 'low' : 'material'}">${r.verified ? 'verified' : 'unresolved'}</span>
        <div class="meta">${esc(trunc(r.label, 100))}</div>
        <span class="ev">"${esc(r.quote)}"</span>
      </div>
    `).join('');
    return;
  }
  (x.actors || []).forEach(a => push('actor', a.label || a.id, a.evidence));
  (x.claims || []).forEach(c => push('claim', c.text, c.evidence));
  (x.events || []).forEach(e => push('event', e.label, e.evidence));
  (x.commitments || []).forEach(c => push('commitment', c.subject, c.evidence));
  (x.patterns || []).forEach(p => push('pattern', p.kind, p.evidence));
  (x.relationships || []).forEach(r => push('relationship', r.type, r.evidence));
  (x.power_dynamics || []).forEach(p => push('power', p.basis, p.evidence));
  (x.escalation_signals || []).forEach(e => push('escalation', e.trigger, e.evidence));
  (x.resolution_opportunities || []).forEach(o => push('resolution', o.opening, o.evidence));
  $('evidence-ledger').innerHTML = rows.length ? rows.map(r => `
    <div class="item evidence-row">
      <span class="tag">${esc(r.kind)}</span>
      <span class="tag ${r.verified ? 'low' : 'material'}">${r.verified ? 'verified' : 'unresolved'}</span>
      <div class="meta">${esc(trunc(r.label, 100))}</div>
      <span class="ev">"${esc(r.quote)}"</span>
    </div>
  `).join('') : '<div class="meta">no evidence quotes returned</div>';
}

function allEvidenceRows(x) {
  const source = $('text').value || '';
  const rows = [];
  if (Array.isArray(x.evidence_audit)) {
    return x.evidence_audit.map(r => ({
      kind: r.kind || 'evidence',
      label: r.label || r.id || '',
      quote: r.quote || '',
      verified: r.status === 'verified'
    }));
  }
  const push = (kind, label, quote) => {
    if (!quote) return;
    rows.push({ kind, label: label || '', quote, verified: source.includes(quote) });
  };
  (x.actors || []).forEach(a => push('actor', a.label || a.id, a.evidence));
  (x.claims || []).forEach(c => push('claim', c.text, c.evidence));
  (x.events || []).forEach(e => push('event', e.label, e.evidence));
  (x.commitments || []).forEach(c => push('commitment', c.subject, c.evidence));
  (x.patterns || []).forEach(p => push('pattern', p.kind, p.evidence));
  (x.relationships || []).forEach(r => push('relationship', r.type, r.evidence));
  (x.power_dynamics || []).forEach(p => push('power', p.basis, p.evidence));
  (x.escalation_signals || []).forEach(e => push('escalation', e.trigger, e.evidence));
  (x.resolution_opportunities || []).forEach(o => push('resolution', o.opening, o.evidence));
  return rows;
}

function countEvidenceQuotes(x) {
  return allEvidenceRows(x).length;
}

function unresolvedEvidenceRows(x) {
  return allEvidenceRows(x).filter(r => !r.verified);
}

function evidenceBadge(quote) {
  if (!quote) return '';
  const verified = ($('text').value || '').includes(quote);
  return `<span class="tag ${verified ? 'low' : 'material'}">${verified ? 'verified' : 'unresolved'}</span>`;
}

function svgEl(name, attrs) {
  const el = document.createElementNS('http://www.w3.org/2000/svg', name);
  for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
  return el;
}
function trunc(s, n) { s = String(s || ''); return s.length > n ? s.slice(0, n - 1) + '…' : s; }

function statusTag(s) {
  switch ((s || '').toLowerCase()) {
    case 'contested': case 'broken': return 'material';
    case 'fulfilled': return 'low';
    case 'accepted': return 'low';
    default: return '';
  }
}
function heat(c) {
  if (c == null) return '';
  return c >= 0.7 ? 'high' : c >= 0.4 ? 'med' : 'low';
}
function actorName(x, id) {
  if (!id) return '';
  const a = (x.actors || []).find(a => a.id === id);
  return a ? (a.label || a.id) : id;
}
function esc(s) {
  return String(s ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
}
