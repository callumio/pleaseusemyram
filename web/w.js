import init, { PleaseUseMyRam } from './pkg/pleaseusemyram.js';

let keep;
onmessage = async ({ data: n }) => {
  try {
    await init();
    keep = new PleaseUseMyRam(n);
    postMessage({ ok: keep.len() });
  } catch (e) { postMessage({ err: String(e) }); }
};