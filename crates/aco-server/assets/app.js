const $ = (id) => document.getElementById(id);

const SAMPLES = {
  workplace: `Sam (Mon 09:14): So we're agreed — you own the Q4 launch deck content, I handle design. Lock it in by Thursday?
Alex (Mon 09:47): Sounds good. I'll pick it up after the Jenkins pitch.
Sam (Wed 17:02): Where are we on the deck content? Designer needs it tomorrow morning.
Alex (Wed 17:31): What deck content? I thought you were doing the whole thing and I'd review.
Sam (Wed 17:45): No — Monday we agreed I do design, you do content. I have the messages.
Alex (Thu 09:02): I never said I'd own it. Just help. You're putting this on me last minute and now blaming me for not delivering something I never agreed to. Honestly, this kind of communication breakdown is exactly why nothing on this team works.
Sam (Thu 09:14): Alex, this is the third time this quarter. We had it in writing. I have screenshots.
Alex (Thu 09:30): Whatever. Send what you have, I'll finish it tonight. But we need to talk about how you set me up to fail.`,

  coparenting: `Jordan (Fri 18:22): Just confirming — I have Mia from Wednesday to Sunday next week, you have her the following week.
Riley (Fri 18:40): That works. Pickup at 4:30 like usual.
Riley (Tue 09:11): Hey, I'm running 20 min late tomorrow. Can you keep her till 5?
Jordan (Tue 09:14): No, I have a meeting at 5:15 I told you about three weeks ago.
Riley (Tue 09:16): You always do this. Make it impossible.
Jordan (Tue 09:18): I'm not making anything impossible. I have a meeting. Reschedule with someone else this once.
Riley (Tue 09:25): Forget it. I'll come at 4:30 like I should have said in the first place. But this is exactly why the kids think you're rigid.
Jordan (Tue 09:30): Don't put what the kids think on me. We can disagree without dragging them in.
Riley (Wed 16:55): At the door — Mia ready?
Jordan (Wed 16:56): Yes. She has her math book and the green sweater she wanted.`,

  board: `BOARD MINUTES — Q1 (March 18, 2025)
The Chair opened by confirming the strategic pivot toward commercial customers had been approved unanimously at the October retreat.
Director Park observed that the retreat had produced no formal resolution on customer segment.
Director Ito moved to ratify the pivot retroactively; motion seconded by Director Chen.
Director Park objected: "We cannot retroactively manufacture a vote we did not take."
The Chair tabled the motion pending review of the October minutes.

BOARD MINUTES — Q2 (June 12, 2025)
The Chair reported that Q1 results were ahead of plan.
Director Ito noted the agenda would be distributed 72 hours in advance, consistent with the new governance protocol.
Director Park stated she had received the pack the morning of, and asked whether the 72-hour rule applied uniformly.
The Chair responded that the rule was directional, not binding.
Director Park noted for the record that she objected to a directional reading of a written protocol.`,

  commercial: `Vendor: We cannot move below the list price. It's a matter of principle at this stage of the quarter.
Buyer: Then we will have to shelve the rollout. We committed internally to a per-seat cost ceiling and I cannot go above it.
Vendor: A pilot could be structured differently. Smaller footprint, executive visibility, an expansion option.
Buyer: Our board wants to see credible traction in this category before the next funding round.
Vendor: Let me be candid — if we discount this deal, every Q4 deal gets pulled in for the same treatment. That's our actual concern.
Buyer: And if we go above ceiling, our procurement team's authority is undermined for the next two cycles. Same shape of problem.
Vendor: So we both need cover. What if the pilot price is at list but the expansion clause renegotiates from a lower base?
Buyer: That could work. Get me the paper by end of week.`,

  diplomatic: `Press briefing, Foreign Ministry of State A (2026-02-03):
The Spokesperson reiterated that State A will not accept any unilateral demarcation of the maritime boundary.
A question on whether consultations with State B had resumed was declined.
The Spokesperson added that cooperation with State C on fisheries remains "unaffected by recent tensions".

Cable, Mission of State B (2026-02-05):
State B's Deputy Permanent Representative stated in the corridor that "technical contacts have continued throughout" and characterized State A's posture as "rhetorical".
The DPR declined to confirm a date for the next political-level meeting.

Op-ed, retired State A diplomat (2026-02-07):
The maritime claim is settled in international law. Any negotiation that proceeds from a contrary premise legitimises the contrary premise. Therefore there is no negotiation to be had.`,

  hr: `HR INTAKE NOTE — formal complaint filed by C. Reyes against M. Patel (manager), filed 2026-04-02.
Reyes alleges: (i) Patel removed her from the Aurora project on March 14 after she raised concerns about timeline feasibility on March 11; (ii) Patel told colleagues in a 1:1 that Reyes "isn't a team player"; (iii) Patel cancelled her promotion review without notice.
Patel response (interview 2026-04-09): "I removed Reyes from Aurora because she had begun to escalate to my skip-level without notifying me. The timeline feedback was not the trigger. The 'not a team player' comment was made in confidence and reflects a pattern of bypass behaviour. I postponed not cancelled her review; the postponement was discussed."
Reyes rebuttal (interview 2026-04-12): "I escalated to skip-level because Patel had not responded to two written concerns about Aurora's scope. The skip-level escalation happened on March 13, the day before removal. The 'pattern of bypass' framing reverses cause and effect."
HR observation: Records show two unread emails from Reyes to Patel dated March 6 and March 9 (subjects: 'Aurora — scope risk').`
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

  $('actors').innerHTML = (x.actors || []).map(a => `
    <div class="item">
      <span class="lbl">${esc(a.label || a.id)}</span>
      ${a.kind ? `<span class="tag">${esc(a.kind)}</span>` : ''}
      ${a.aliases?.length ? `<div class="meta">aka: ${a.aliases.map(esc).join(', ')}</div>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('claims').innerHTML = (x.claims || []).map(c => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, c.actor_id))}:</span>
      ${esc(c.text)}
      ${c.polarity ? `<span class="tag">${esc(c.polarity)}</span>` : ''}
      ${c.evidence ? `<span class="ev">"${esc(c.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('events').innerHTML = (x.events || []).map(e => `
    <div class="item">
      <span class="lbl">${esc(e.label)}</span>
      ${e.when ? `<span class="meta">· ${esc(e.when)}</span>` : ''}
      ${e.evidence ? `<span class="ev">"${esc(e.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('commitments').innerHTML = (x.commitments || []).map(c => `
    <div class="item">
      <span class="lbl">${esc(actorName(x, c.by_actor))} →</span> ${esc(c.subject)}
      ${c.deadline ? `<span class="meta">· due ${esc(c.deadline)}</span>` : ''}
      <span class="tag ${statusTag(c.status)}">${esc(c.status)}</span>
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
      ${p.evidence ? `<span class="ev">"${esc(p.evidence)}"</span>` : ''}
    </div>
  `).join('') || '<div class="meta">none</div>';

  $('contras').innerHTML = (x.contradictions || []).map(c => {
    const a = (x.claims || []).find(cl => cl.id === c.claim_a);
    const b = (x.claims || []).find(cl => cl.id === c.claim_b);
    return `
    <div class="item">
      <span class="tag ${c.materiality}">${esc(c.materiality)}</span>
      <div style="margin-top:6px"><strong>A:</strong> ${esc(a?.text || c.claim_a)}</div>
      <div style="margin-top:4px"><strong>B:</strong> ${esc(b?.text || c.claim_b)}</div>
      ${c.rationale ? `<div class="meta" style="margin-top:6px">${esc(c.rationale)}</div>` : ''}
    </div>`;
  }).join('') || '<div class="meta">none</div>';

  $('raw').textContent = JSON.stringify(x, null, 2);
  drawGraph(x);
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
