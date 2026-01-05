/**
 * mdbook-htmx Cloudflare Worker
 *
 * Handles static asset serving with HTMX-aware routing.
 *
 * Phase 1: Static file serving
 * Phase 2: HTMX fragment routing
 * Phase 3: Authentication middleware (D1)
 * Phase 4: Search endpoint (KV)
 * Phase 5: Analytics and R2 assets
 */

import { handleSearch } from "./search";

export interface Env {
  ASSETS: Fetcher;
  ENVIRONMENT: string;
  // Phase 3: D1 Database
  // DB: D1Database;
  // Phase 4: KV Namespace for search index
  SEARCH_INDEX?: KVNamespace;
  // Phase 5: R2 Bucket
  // ASSETS_BUCKET: R2Bucket;
}

export default {
  async fetch(
    request: Request,
    env: Env,
    ctx: ExecutionContext
  ): Promise<Response> {
    const url = new URL(request.url);

    // Health check endpoint
    if (url.pathname === "/health") {
      return new Response("healthy", {
        headers: { "Content-Type": "text/plain" },
      });
    }

    // Phase 4: Search endpoint
    if (url.pathname === "/search") {
      return handleSearch(request, env, url);
    }

    // Phase 2: HTMX fragment routing
    const isHtmxRequest = request.headers.get("HX-Request") === "true";

    if (isHtmxRequest) {
      // Serve fragment instead of full page
      const fragmentPath = `/fragments${url.pathname}`;
      const fragmentUrl = new URL(fragmentPath, url.origin);

      try {
        const response = await env.ASSETS.fetch(
          new Request(fragmentUrl, request)
        );

        if (response.ok) {
          // Add HTMX-specific headers
          const headers = new Headers(response.headers);
          headers.set("Vary", "HX-Request");
          headers.set("Cache-Control", "private, max-age=0");

          return new Response(response.body, {
            status: response.status,
            headers,
          });
        }
      } catch {
        // Fragment not found, fall through to full page
      }
    }

    // Serve static assets
    try {
      const response = await env.ASSETS.fetch(request);

      // Add security headers
      const headers = new Headers(response.headers);
      headers.set("X-Frame-Options", "SAMEORIGIN");
      headers.set("X-Content-Type-Options", "nosniff");
      headers.set("X-XSS-Protection", "1; mode=block");
      headers.set("Vary", "HX-Request");

      return new Response(response.body, {
        status: response.status,
        headers,
      });
    } catch (error) {
      return new Response("Not Found", { status: 404 });
    }
  },
};
