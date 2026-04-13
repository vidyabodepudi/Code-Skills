# Performance Checklist — CodeHands Reference

> Quick-scan performance checklist for code review. Reference during `/review` phase.

## Frontend

### Loading
- [ ] Bundle size analyzed and within budget
- [ ] Code splitting at route boundaries
- [ ] Images in modern format (WebP/AVIF) with fallbacks
- [ ] Images lazy-loaded below the fold
- [ ] Images have explicit width/height (prevents CLS)
- [ ] Critical CSS inlined, non-critical deferred
- [ ] Fonts preloaded or using `font-display: swap`
- [ ] Third-party scripts loaded async/defer

### Runtime
- [ ] No unnecessary re-renders (React: React.memo, useMemo, useCallback)
- [ ] No expensive computations on main thread (use Web Workers)
- [ ] Event listeners cleaned up on unmount
- [ ] Virtualization for long lists (react-window, react-virtualized)
- [ ] Debounce/throttle on frequent events (scroll, resize, input)
- [ ] No DOM manipulation in loops

### Core Web Vitals
- [ ] LCP < 2.5s (Largest Contentful Paint)
- [ ] CLS < 0.1 (Cumulative Layout Shift)
- [ ] INP < 200ms (Interaction to Next Paint)

## Backend

### Database
- [ ] No N+1 queries (use eager loading, DataLoader, or joins)
- [ ] Indexes on frequently queried columns
- [ ] No unbounded queries (SELECT * without LIMIT)
- [ ] Connection pooling configured
- [ ] Slow query logging enabled
- [ ] Pagination for list endpoints

### API
- [ ] Response payloads minimal (no over-fetching)
- [ ] Pagination metadata included (total, page, limit)
- [ ] Response compression enabled (gzip/brotli)
- [ ] Appropriate cache headers (Cache-Control, ETag)
- [ ] Rate limiting configured

### Infrastructure
- [ ] CDN configured for static assets
- [ ] HTTP/2 or HTTP/3 enabled
- [ ] Keep-alive connections configured
- [ ] Appropriate timeouts set (connection, read, write)

## Algorithmic
- [ ] No O(n²) or worse in user-facing request paths
- [ ] Appropriate data structures (Map/Set for lookups vs Array)
- [ ] Memoization for expensive pure computations
- [ ] Streaming for large data processing (not loading all into memory)
