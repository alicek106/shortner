use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

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
    label {
      display: block;
      font-size: 0.85rem;
      font-weight: 600;
      color: #374151;
      margin-bottom: 6px;
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
    button {
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
    button:hover { opacity: 0.9; }
    button:active { transform: scale(0.98); }
    #msg {
      margin-top: 20px;
      padding: 12px 14px;
      border-radius: 8px;
      font-size: 0.9rem;
      display: none;
    }
    #msg.success { background: #ecfdf5; color: #065f46; border: 1px solid #6ee7b7; }
    #msg.error   { background: #fef2f2; color: #991b1b; border: 1px solid #fca5a5; }
  </style>
</head>
<body>
  <div class="card">
    <h1>🔗 URL Shortener</h1>
    <p class="subtitle">Map any URL to a short path instantly.</p>

    <label for="new_url">Destination URL</label>
    <input type="text" id="new_url" placeholder="https://example.com/very/long/url" />

    <label for="old_url">Short path (e.g. <code>/go</code>)</label>
    <input type="text" id="old_url" placeholder="/my-link" />

    <button onclick="create()">Create</button>
    <div id="msg"></div>
  </div>

  <script>
    async function create() {
      const old_url = document.getElementById('old_url').value.trim();
      const new_url = document.getElementById('new_url').value.trim();
      const msg = document.getElementById('msg');
      msg.style.display = 'none';

      if (!old_url || !new_url) {
        show(msg, 'error', 'Both fields are required.');
        return;
      }

      try {
        const res = await fetch('/new', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ old_url, new_url }),
        });
        const data = await res.json();
        if (res.ok) {
          show(msg, 'success', `✓ Created: ${old_url} → ${new_url}`);
          document.getElementById('old_url').value = '';
          document.getElementById('new_url').value = '';
        } else {
          show(msg, 'error', data.error || 'Something went wrong.');
        }
      } catch (e) {
        show(msg, 'error', 'Network error: ' + e.message);
      }
    }

    function show(el, type, text) {
      el.className = type;
      el.textContent = text;
      el.style.display = 'block';
    }

    document.addEventListener('keydown', e => {
      if (e.key === 'Enter') create();
    });
  </script>
</body>
</html>"#,
    )
}
