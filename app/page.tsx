"use client";

import { invoke } from "@tauri-apps/api/core";

// New: central app config array
const apps = [
  { id: "youtube", label: "YouTube", url: "https://www.youtube.com/", icon: "▶", accent: "from-pink-500 to-red-500" },
  { id: "chatgpt", label: "ChatGPT", url: "https://chat.openai.com/", icon: "💬", accent: "from-emerald-500 to-teal-500" },
  { id: "gemini", label: "Gemini", url: "https://gemini.google.com/", icon: "✨", accent: "from-sky-500 to-indigo-500" },
  { id: "claude", label: "Claude", url: "https://claude.ai/", icon: "🌤", accent: "from-amber-500 to-orange-600" },
  { id: "grok", label: "Grok", url: "https://grok.com/", icon: "🧠", accent: "from-fuchsia-500 to-purple-600" },
  { id: "deepseek", label: "DeepSeek", url: "https://chat.deepseek.com/", icon: "🔎", accent: "from-cyan-500 to-blue-600" },
  { id: "canva", label: "Canva", url: "https://www.canva.com/", icon: "🎨", accent: "from-green-500 to-lime-600" },
];

export default function Home() {
  return (
    <div className="min-h-screen relative flex items-center justify-center bg-[radial-gradient(circle_at_top_left,#0f172a,#020617)] text-white overflow-hidden">
      {/* Ambient glows */}
      <div className="absolute inset-0 -z-10 pointer-events-none">
        <div className="absolute top-10 left-1/2 -translate-x-1/2 w-72 h-72 bg-cyan-500/20 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-10 right-10 w-60 h-60 bg-fuchsia-500/10 rounded-full blur-3xl" />
      </div>

      <div className="w-full max-w-4xl px-6 md:px-10">
        <div className="space-y-8 backdrop-blur-sm bg-white/5 border border-white/10 rounded-2xl p-8 shadow-[0_0_0_1px_rgba(255,255,255,0.05),0_8px_40px_-10px_rgba(0,0,0,0.6)]">
          <header className="space-y-2">
            <h1 className="text-3xl md:text-4xl font-semibold tracking-tight bg-gradient-to-r from-white to-slate-300 bg-clip-text text-transparent">
              AI Portals
            </h1>
            <p className="text-sm text-slate-400">
              Launch an external window for your chosen service.
            </p>
          </header>

          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {apps.map(app => (
              <button
                key={app.id}
                onClick={() => invoke("navigate_to", { id: app.id, url: app.url })}
                className="group relative overflow-hidden rounded-xl border border-white/10 bg-white/[0.04] px-5 py-5 text-left transition
                           hover:bg-white/[0.08] focus:outline-none focus:ring-2 focus:ring-cyan-400/60 active:scale-[0.98]"
              >
                {/* Animated gradient frame */}
                <span
                  className={`pointer-events-none absolute inset-0 opacity-0 group-hover:opacity-100 transition
                              bg-gradient-to-br ${app.accent} mix-blend-overlay`}
                />
                {/* Shine sweep */}
                <span className="pointer-events-none absolute -inset-10 bg-gradient-to-r from-transparent via-white/10 to-transparent rotate-12 translate-x-[-60%] group-hover:translate-x-[60%] transition-all duration-700" />
                <span className="relative z-10 flex items-center gap-3">
                  <span className="text-lg">{app.icon}</span>
                  <span className="font-medium">{app.label}</span>
                </span>
              </button>
            ))}
          </div>

          <footer className="pt-2">
            <p className="text-xs text-center text-slate-500">
              Windows open externally via Tauri integration.
            </p>
          </footer>
        </div>
      </div>
    </div>
  );
}
