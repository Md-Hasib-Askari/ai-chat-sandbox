"use client";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export default function GeminiPage() {
    useEffect(() => {
        invoke("open_gemini");
    }, []);

    return (
        <button
            className="absolute top-4 left-4 px-4 py-2 bg-gray-800 text-white rounded-lg"
            onClick={() => invoke("go_home")}
        >
            ← Back
        </button>
    );
}
