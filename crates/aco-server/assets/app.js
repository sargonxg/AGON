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
    $('backend').textContent = info.backend + ' · ' + info.project;
  } catch (_) {}
})();

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
    setStatus(`${data.extraction.actors?.length || 0} actors · ${data.input_tokens}/${data.output_tokens} tokens · ${dt.toFixed(1)}s · ${data.model}`);
    render(data.extraction);
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
}

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
