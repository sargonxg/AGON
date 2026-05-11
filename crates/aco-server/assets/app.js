const $ = (id) => document.getElementById(id);

const SAMPLE = `Sam (Mon 09:14): So we're agreed — you own the Q4 launch deck content, I handle design. Lock it in by Thursday?
Alex (Mon 09:47): Sounds good. I'll pick it up after the Jenkins pitch.
Sam (Wed 17:02): Where are we on the deck content? Designer needs it tomorrow morning.
Alex (Wed 17:31): What deck content? I thought you were doing the whole thing and I'd review.
Sam (Wed 17:45): No — Monday we agreed I do design, you do content. I have the messages.
Alex (Thu 09:02): I never said I'd own it. Just help. You're putting this on me last minute and now blaming me for not delivering something I never agreed to. Honestly, this kind of communication breakdown is exactly why nothing on this team works.
Sam (Thu 09:14): Alex, this is the third time this quarter. We had it in writing. I have screenshots.
Alex (Thu 09:30): Whatever. Send what you have, I'll finish it tonight. But we need to talk about how you set me up to fail.`;

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

$('sample').addEventListener('click', () => { $('text').value = SAMPLE; });

$('run').addEventListener('click', async () => {
  const text = $('text').value.trim();
  if (!text) { setStatus('paste some text first', true); return; }
  const btn = $('run');
  btn.disabled = true;
  setStatus('perceiving…');
  const t0 = performance.now();
  try {
    const r = await fetch('/api/perceive', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ text })
    });
    if (!r.ok) {
      const err = await r.text();
      throw new Error(err || r.statusText);
    }
    const data = await r.json();
    const dt = (performance.now() - t0) / 1000;
    setStatus(`${data.extraction.actors?.length || 0} actors · ${data.input_tokens}/${data.output_tokens} tokens · ${dt.toFixed(1)}s · ${data.model}${data.persisted ? ' · 💾 saved' : ''}`);
    render(data.extraction);
    loadHistory();
  } catch (e) {
    setStatus('error: ' + e.message, true);
  } finally {
    btn.disabled = false;
  }
});

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
