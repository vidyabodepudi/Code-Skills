# Accessibility Checklist — CodeHands Reference

> WCAG 2.2 AA compliance checklist for code review. Reference during `/review` phase.

## Semantic HTML
- [ ] Proper heading hierarchy (single `<h1>`, sequential `<h2>`-`<h6>`)
- [ ] Semantic elements used (`<nav>`, `<main>`, `<article>`, `<aside>`, `<footer>`)
- [ ] Lists use `<ul>`, `<ol>`, `<dl>` (not styled `<div>`s)
- [ ] Tables use `<th>` with `scope` attributes
- [ ] Language attribute set on `<html>` element

## Interactive Elements
- [ ] All interactive elements reachable via keyboard (Tab, Enter, Space)
- [ ] Focus order follows visual layout
- [ ] Focus indicator visible (never `outline: none` without replacement)
- [ ] Skip navigation link present for long pages
- [ ] Modal dialogs trap focus
- [ ] Custom controls have appropriate ARIA roles and states

## Images & Media
- [ ] All images have `alt` text (decorative images: `alt=""`)
- [ ] Alt text describes PURPOSE, not appearance
- [ ] Videos have captions/subtitles
- [ ] Audio content has transcript
- [ ] No information conveyed by color alone

## Forms
- [ ] All inputs have associated `<label>` elements (or `aria-label`)
- [ ] Error messages identify the field and describe the error
- [ ] Required fields indicated (not by color alone)
- [ ] Form validation errors announced to screen readers
- [ ] Autocomplete attributes set for common fields

## Color & Contrast
- [ ] Text contrast ratio ≥ 4.5:1 (normal text)
- [ ] Text contrast ratio ≥ 3:1 (large text, 18px+ or 14px+ bold)
- [ ] UI component contrast ≥ 3:1 against adjacent colors
- [ ] Information NOT conveyed by color alone (use icons, text, patterns)

## Motion & Timing
- [ ] Animations respect `prefers-reduced-motion` media query
- [ ] No content that flashes > 3 times per second
- [ ] Auto-playing content can be paused/stopped
- [ ] Time limits have extension mechanism (or are generous)

## Testing
- [ ] Tab through entire page — all interactive elements reachable?
- [ ] Screen reader test (VoiceOver on Mac: Cmd+F5)
- [ ] Zoom to 200% — layout still functional?
- [ ] High contrast mode — all content visible?
