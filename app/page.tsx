"use client";

import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  return (
    <div className="h-screen flex flex-col items-center justify-center gap-4 bg-black text-white">
      <button
        onClick={() => invoke("navigate_to", { id: "chatgpt", url: "https://chat.openai.com/" })}
        className="px-6 py-3 bg-green-600 rounded-lg"
      >
        Open ChatGPT
      </button>

      <button
        onClick={() => invoke("navigate_to", { id: "gemini", url: "https://gemini.google.com/" })}
        className="px-6 py-3 bg-blue-600 rounded-lg"
      >
        Open Gemini
      </button>

      <button
        onClick={() => invoke("navigate_to", { id: "claude", url: "https://claude.ai/" })}
        className="px-6 py-3 bg-blue-600 rounded-lg"
      >
        Open Claude
      </button>

      <button
        onClick={() => invoke("navigate_to", { id: "grok", url: "https://grok.com/" })}
        className="px-6 py-3 bg-blue-600 rounded-lg"
      >
        Open Grok
      </button>

      <button
        onClick={() => invoke("navigate_to", { id: "deepseek", url: "https://chat.deepseek.com/" })}
        className="px-6 py-3 bg-blue-600 rounded-lg"
      >
        Open DeepSeek
      </button>
    </div>
  );
}
