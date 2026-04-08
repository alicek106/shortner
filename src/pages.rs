use axum::response::IntoResponse;

// warning!!
// AI-generated HTML code
pub async fn root_handler() -> impl IntoResponse {
    axum::response::Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>URL Shortener</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      min-height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    }
    .card {
      background: #fff;
      border-radius: 16px;
      padding: 40px 48px;
      width: 100%;
      max-width: 480px;
      box-shadow: 0 20px 60px rgba(0,0,0,0.2);
    }
    h1 {
      font-size: 1.6rem;
      font-weight: 700;
      color: #1a1a2e;
      margin-bottom: 8px;
    }
    p.subtitle {
      font-size: 0.9rem;
      color: #6b7280;
      margin-bottom: 32px;
    }
    .label-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 6px;
    }
    label {
      font-size: 0.85rem;
      font-weight: 600;
      color: #374151;
    }
    .checkbox-label {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 0.8rem;
      font-weight: 500;
      color: #6b7280;
      cursor: pointer;
    }
    .checkbox-label input[type="checkbox"] {
      width: 15px;
      height: 15px;
      accent-color: #667eea;
      cursor: pointer;
    }
    input[type="text"] {
      width: 100%;
      padding: 12px 14px;
      border: 1.5px solid #e5e7eb;
      border-radius: 8px;
      font-size: 0.95rem;
      color: #111827;
      transition: border-color 0.2s;
      margin-bottom: 20px;
      outline: none;
    }
    input[type="text"]:focus {
      border-color: #667eea;
      box-shadow: 0 0 0 3px rgba(102,126,234,0.15);
    }
    input[type="text"]:disabled {
      background: #f3f4f6;
      color: #9ca3af;
      cursor: not-allowed;
    }
    button.submit-btn {
      width: 100%;
      padding: 13px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: #fff;
      border: none;
      border-radius: 8px;
      font-size: 1rem;
      font-weight: 600;
      cursor: pointer;
      transition: opacity 0.2s, transform 0.1s;
      margin-top: 4px;
    }
    button.submit-btn:hover { opacity: 0.9; }
    button.submit-btn:active { transform: scale(0.98); }
    #msg {
      margin-top: 20px;
      padding: 12px 14px;
      border-radius: 8px;
      font-size: 0.9rem;
      display: none;
    }
    #msg.error { background: #fef2f2; color: #991b1b; border: 1px solid #fca5a5; }
    .result-box {
      margin-top: 20px;
      padding: 12px 14px;
      border-radius: 8px;
      background: #ecfdf5;
      border: 1px solid #6ee7b7;
      display: none;
      align-items: center;
      gap: 10px;
    }
    .result-box a {
      flex: 1;
      font-size: 0.9rem;
      color: #065f46;
      font-weight: 500;
      word-break: break-all;
      text-decoration: none;
    }
    .result-box a:hover { text-decoration: underline; }
    .copy-btn {
      flex-shrink: 0;
      background: none;
      border: none;
      cursor: pointer;
      padding: 4px;
      border-radius: 6px;
      color: #065f46;
      transition: background 0.15s;
      display: flex;
      align-items: center;
    }
    .copy-btn:hover { background: #d1fae5; }
    .copy-btn.copied { color: #047857; }
  </style>
</head>
<body>
  <div class="card">
    <h1>🔗 URL Shortener</h1>
    <p class="subtitle">Map any URL to a short path instantly.</p>

    <label for="dest_url" style="display:block; margin-bottom:6px;">Destination URL</label>
    <input type="text" id="dest_url" placeholder="https://example.com/very/long/url" />

    <div class="label-row">
      <label for="short_path">Short Path</label>
      <label class="checkbox-label">
        <input type="checkbox" id="auto_path" checked onchange="toggleAutoPath()" />
        무작위 생성
      </label>
    </div>
    <input type="text" id="short_path" placeholder="/my-link" disabled />

    <button class="submit-btn" onclick="create()">Create</button>

    <div id="msg" class="error"></div>
    <div class="result-box" id="result-box">
      <a id="result-url" href="\#" target="_blank"></a>
      <button class="copy-btn" id="copy-btn" onclick="copyUrl()" title="복사">
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"
             fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
        </svg>
      </button>
    </div>
  </div>

  <script>
    function toggleAutoPath() {
      const cb = document.getElementById('auto_path');
      const input = document.getElementById('short_path');
      input.disabled = cb.checked;
      if (cb.checked) input.value = '';
    }

    async function create() {
      const dest_url = document.getElementById('dest_url').value.trim();
      const auto = document.getElementById('auto_path').checked;
      const short_path = document.getElementById('short_path').value.trim();
      const msg = document.getElementById('msg');
      const resultBox = document.getElementById('result-box');

      msg.style.display = 'none';
      resultBox.style.display = 'none';

      if (!dest_url) {
        show(msg, 'Destination URL을 입력해주세요.');
        return;
      }
      if (!auto && !short_path) {
        show(msg, 'Short path를 입력하거나 무작위 생성을 선택해주세요.');
        return;
      }

      const body = { dest_url };
      if (!auto) body.short_path = short_path;

      try {
        const res = await fetch('/new', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        });
        const data = await res.json();
        if (res.ok) {
          const fullUrl = `${location.origin}${data.short_url}`;
          const link = document.getElementById('result-url');
          link.textContent = fullUrl;
          link.href = fullUrl;
          resultBox.style.display = 'flex';
          document.getElementById('dest_url').value = '';
          document.getElementById('short_path').value = '';
        } else {
          show(msg, data.error || 'Something went wrong.');
        }
      } catch (e) {
        show(msg, 'Network error: ' + e.message);
      }
    }

    function show(el, text) {
      el.textContent = text;
      el.style.display = 'block';
    }

    function copyUrl() {
      const url = document.getElementById('result-url').textContent;
      navigator.clipboard.writeText(url).then(() => {
        const btn = document.getElementById('copy-btn');
        btn.classList.add('copied');
        btn.title = '복사됨!';
        setTimeout(() => {
          btn.classList.remove('copied');
          btn.title = '복사';
        }, 1500);
      });
    }

    document.addEventListener('keydown', e => {
      if (e.key === 'Enter') create();
    });
  </script>
</body>
</html>"#,
    )
}
