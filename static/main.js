const $ = (s, r = document) => r.querySelector(s);
const $$ = (s, r = document) => [...r.querySelectorAll(s)];

// Machine à écrire
(() => {
  const el = $('#typed'); if (!el) return;
  const words = el.dataset.words.split('|'); let w = 0, i = 0, del = false;
  const tick = () => {
    const word = words[w];
    el.textContent = word.slice(0, i);
    if (!del && i === word.length) { del = true; return setTimeout(tick, 1400); }
    if (del && i === 0) { del = false; w = (w + 1) % words.length; }
    i += del ? -1 : 1;
    setTimeout(tick, del ? 40 : 80);
  };
  tick();
})();

// Apparition au scroll, barres de compétences, compteurs
const io = new IntersectionObserver(es => es.forEach(e => {
  if (!e.isIntersecting) return;
  const t = e.target; t.classList.add('in'); io.unobserve(t);
  $$('.bar i', t).forEach(b => b.style.width = b.dataset.w + '%');
  $$('[data-count]', t).forEach(c => {
    const n = +c.dataset.count; let v = 0;
    const step = () => { v += Math.max(1, n / 40); c.textContent = Math.min(n, Math.round(v)); if (v < n) requestAnimationFrame(step); };
    step();
  });
  $$('.tl, .card', t).forEach((x, k) => { x.style.transitionDelay = (k * 80) + 'ms'; x.classList.add('in'); });
}), { threshold: .12 });
$$('.reveal').forEach(el => io.observe(el));

// Barre de progression
addEventListener('scroll', () => {
  const h = document.documentElement;
  $('#progress').style.width = (h.scrollTop / (h.scrollHeight - h.clientHeight) * 100) + '%';
}, { passive: true });

// Filtres par technologie
(() => {
  const cards = $$('.card'), box = $('#filters');
  const tags = [...new Set(cards.flatMap(c => c.dataset.tags.split(',').filter(Boolean)))].sort();
  tags.forEach(t => { const b = document.createElement('button'); b.dataset.f = t; b.textContent = t; box.append(b); });
  box.addEventListener('click', e => {
    const b = e.target.closest('button'); if (!b) return;
    $$('button', box).forEach(x => x.classList.toggle('on', x === b));
    cards.forEach(c => c.classList.toggle('hide', b.dataset.f !== 'all' && !c.dataset.tags.split(',').includes(b.dataset.f)));
  });
})();
