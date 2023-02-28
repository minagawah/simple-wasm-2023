import './main.css';
import('../pkg/index.js').catch(console.error);

(async () => {
  try {
    const wasm = await import('../pkg/index.js');
    console.log('[index] Ready!');
  } catch (err) {
    console.error(err);
  }
})();
