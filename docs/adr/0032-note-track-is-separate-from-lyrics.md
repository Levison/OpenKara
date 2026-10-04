# ADR 0032 — Note Track is separate from lyrics

Date: 2026-10-03
Status: accepted

## Context

OpenKara needs a SingStar-style pitch lane (Levison/OpenKara#10). Pitch data
arrives from UltraStar `.txt` files and may later arrive from a derived vocal
pipeline. Lyrics Acquisition already has a fixed multi-source chain
(ADRs 0015/0026/0027). Putting pitch fields on `WordToken` would force that
chain to understand UltraStar and would block derived pitch that does not match
lyric tokens one-to-one.

## Decision

Store pitch targets as a **Note Track**, a per-song artifact separate from
lyrics. OpenKara's serialized note-track model is canonical. Source formats
(UltraStar, derived, editor) convert into that model. Lyrics Acquisition does
not change.

The pitch lane is a persisted `lyrics_display_mode` value. It has no automatic
fallback to line lyrics. Songs without a note track still render the lane: flat
bars on the centre line for word-timed lyrics, one bar per line for line-timed
lyrics, plus a non-blocking "No pitch data" badge.

In pitch-lane mode, words under the lane come from note syllable text. Ordinary
lyrics modes keep using Lyrics Acquisition output. When a song has no lyrics and
an UltraStar file is imported, OpenKara also builds lyrics from note text and
phrase breaks so those modes are not empty.

The lyrics editor keeps writing LRC for now. Note-track editing is a later
surface.

## Consequences

- Storage and IPC for note tracks land before the UltraStar parser
  (Levison/OpenKara#3 before #4).
- Product-standards route for the pitch lane includes interaction/accessibility,
  language/terminology, interfaces/compatibility, and (for mic scoring later)
  security/privacy.
- Evidence for the display milestone includes 60 fps on a long song, reduced-
  motion playhead behavior, keyboard access for the mode toggle, and Playwright
  visuals with a mock pitched song.
