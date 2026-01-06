/**
 * mdbook-htmx Search Handler
 *
 * Phase 4: Search endpoint for Cloudflare Workers
 *
 * Provides server-side search using KV-stored search index.
 * Supports both JSON API and HTML fragment responses for HTMX.
 */

import type { Env } from "./index";

/** Search document from the search index */
interface SearchDocument {
  path: string;
  title: string;
  body: string;
  headings: Array<{
    level: number;
    text: string;
    anchor: string;
  }>;
  auth?: {
    authn?: string;
    authz?: string[];
  };
}

/** Search index structure */
interface SearchIndex {
  version: string;
  generated_at: string;
  config: {
    heading_split_level: number;
    max_excerpt_length?: number;
    include_auth: boolean;
  };
  documents: SearchDocument[];
}

/** Search result with relevance scoring */
interface SearchResult {
  document: SearchDocument;
  score: number;
  matches: {
    title: boolean;
    body: boolean;
    headings: string[];
  };
  excerpt?: string;
}

/**
 * Handle search requests
 *
 * @param request - The incoming request
 * @param env - Worker environment bindings
 * @param url - Parsed request URL
 * @returns Search results as JSON or HTML fragment
 */
export async function handleSearch(
  request: Request,
  env: Env & { SEARCH_INDEX?: KVNamespace },
  url: URL
): Promise<Response> {
  const query = url.searchParams.get("q")?.trim() || "";

  if (!query) {
    return emptyResults(request);
  }

  // Get search index from KV or static file
  const index = await getSearchIndex(env);

  if (!index) {
    return new Response(JSON.stringify({ error: "Search index not available" }), {
      status: 503,
      headers: { "Content-Type": "application/json" },
    });
  }

  // Perform search
  const results = search(index, query);

  // Return appropriate format
  const isHtmx = request.headers.get("HX-Request") === "true";
  const wantsHtml = request.headers.get("Accept")?.includes("text/html");

  if (isHtmx || wantsHtml) {
    return htmlResponse(results, query);
  }

  return jsonResponse(results);
}

/**
 * Get search index from KV or fallback to static file
 */
async function getSearchIndex(
  env: Env & { SEARCH_INDEX?: KVNamespace }
): Promise<SearchIndex | null> {
  // Try KV first
  if (env.SEARCH_INDEX) {
    const cached = await env.SEARCH_INDEX.get("search-index", "json");
    if (cached) {
      return cached as SearchIndex;
    }
  }

  // Fallback to static file
  try {
    const response = await env.ASSETS.fetch(
      new Request("https://placeholder/search-index.json")
    );
    if (response.ok) {
      return response.json();
    }
  } catch {
    // Index not available
  }

  return null;
}

/**
 * Search the index for matching documents
 */
function search(index: SearchIndex, query: string): SearchResult[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);

  if (terms.length === 0) {
    return [];
  }

  const results: SearchResult[] = [];

  for (const doc of index.documents) {
    const titleLower = doc.title.toLowerCase();
    const bodyLower = doc.body.toLowerCase();

    let score = 0;
    const matches = {
      title: false,
      body: false,
      headings: [] as string[],
    };

    for (const term of terms) {
      // Title match (highest weight)
      if (titleLower.includes(term)) {
        score += 10;
        matches.title = true;
      }

      // Heading match (medium weight)
      for (const heading of doc.headings) {
        if (heading.text.toLowerCase().includes(term)) {
          score += 5;
          if (!matches.headings.includes(heading.text)) {
            matches.headings.push(heading.text);
          }
        }
      }

      // Body match (lower weight)
      if (bodyLower.includes(term)) {
        score += 1;
        matches.body = true;
      }
    }

    if (score > 0) {
      // Generate excerpt around first match
      let excerpt: string | undefined;
      if (matches.body && doc.body) {
        const firstTerm = terms.find((t) => bodyLower.includes(t));
        if (firstTerm) {
          const pos = bodyLower.indexOf(firstTerm);
          const start = Math.max(0, pos - 50);
          const end = Math.min(doc.body.length, pos + 100);
          excerpt =
            (start > 0 ? "..." : "") +
            doc.body.slice(start, end) +
            (end < doc.body.length ? "..." : "");
        }
      }

      results.push({ document: doc, score, matches, excerpt });
    }
  }

  // Sort by score descending
  results.sort((a, b) => b.score - a.score);

  // Limit results
  return results.slice(0, 20);
}

/**
 * Return empty results response
 */
function emptyResults(request: Request): Response {
  const isHtmx = request.headers.get("HX-Request") === "true";

  if (isHtmx) {
    return new Response(
      '<div class="search-no-results">Enter a search term</div>',
      {
        headers: {
          "Content-Type": "text/html",
          Vary: "HX-Request",
        },
      }
    );
  }

  return new Response(JSON.stringify({ results: [] }), {
    headers: { "Content-Type": "application/json" },
  });
}

/**
 * Return JSON response
 */
function jsonResponse(results: SearchResult[]): Response {
  return new Response(
    JSON.stringify({
      results: results.map((r) => ({
        path: r.document.path,
        title: r.document.title,
        headings: r.matches.headings.slice(0, 3),
        excerpt: r.excerpt,
        score: r.score,
      })),
    }),
    {
      headers: {
        "Content-Type": "application/json",
        "Cache-Control": "private, max-age=60",
      },
    }
  );
}

/**
 * Return HTML fragment response for HTMX
 */
function htmlResponse(results: SearchResult[], query: string): Response {
  if (results.length === 0) {
    return new Response(
      `<div class="search-no-results">No results found for "${escapeHtml(query)}"</div>`,
      {
        headers: {
          "Content-Type": "text/html",
          Vary: "HX-Request",
          "Cache-Control": "private, max-age=60",
        },
      }
    );
  }

  const html = `
<ul class="search-results-list" role="listbox">
  ${results
    .map(
      (r) => `
  <li class="search-result-item" role="option">
    <a href="${escapeHtml(r.document.path)}"
       hx-get="${escapeHtml(r.document.path)}"
       hx-target="#content"
       hx-push-url="true"
       class="search-result-link">
      <span class="search-result-title">${escapeHtml(r.document.title)}</span>
      ${
        r.matches.headings.length > 0
          ? `
      <ul class="search-result-headings">
        ${r.matches.headings
          .slice(0, 3)
          .map((h) => {
            const heading = r.document.headings.find((dh) => dh.text === h);
            const anchor = heading?.anchor || "";
            return `
        <li>
          <a href="${escapeHtml(r.document.path)}${escapeHtml(anchor)}"
             hx-get="${escapeHtml(r.document.path)}${escapeHtml(anchor)}"
             hx-target="#content"
             hx-push-url="true"
             class="search-result-heading">
            ${escapeHtml(h)}
          </a>
        </li>`;
          })
          .join("")}
      </ul>`
          : ""
      }
      ${r.excerpt ? `<p class="search-result-excerpt">${escapeHtml(r.excerpt)}</p>` : ""}
    </a>
  </li>`
    )
    .join("")}
</ul>`;

  return new Response(html, {
    headers: {
      "Content-Type": "text/html",
      Vary: "HX-Request",
      "Cache-Control": "private, max-age=60",
    },
  });
}

/**
 * Escape HTML special characters
 */
function escapeHtml(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}
