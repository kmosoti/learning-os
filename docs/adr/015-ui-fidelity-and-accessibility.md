# ADR-015: LearnHarness fidelity and accessible PWA adaptation

Status: Proposed formal record; fixed direction accepted and implementation authorized by the owner on 2026-10-25 with “Implement the proposed plan.” Mechanism choices are deferred to the approved experiments. PR approval/merge remains pending.

Related: [#49](https://github.com/kmosoti/learning-os/issues/49) and [execution contract](../architecture/v2-execution-contract.md).

## Decision
Use LearnHarness 0795ef197cca04f6a8e7d50d10ed4dc71bd1a0cc as the pinned visual/interaction reference. Reconstruct its graph home, dark palette, serif session titles, mono controls, sidebar, transcript, dock and plan marginalia independently in Leptos. Source inventory and actual upstream screenshots are in [fidelity baseline](../ui-reference/learnharness-fidelity.md).

## Deliberate differences
LearningOS branding; browser-native controls instead of imitation desktop chrome; visible keyboard focus instead of global outline suppression; accessible mobile drawers/sheets; structured graph list; source/evidence/uncertainty inspection; honest local activity and unavailable remote features. No decorative fake agent activity.

## Attribution
Keep the repository license and attribute reference screenshots to upstream contributors. Do not transplant source, agent/store architecture, marks or prompts. Distributed fonts need their own retained licenses. Actual runtime comparisons must identify viewport/revision and not label old upstream screenshots as new captures.

## Confirmation
Screenshot states at 1440×900, 1280×720, 390×844 and 360×740; landmark/token comparison with deliberate differences documented; axe plus keyboard/screen-reader/zoom/reduced-motion tests; mobile keyboard does not cover controls. Real Android remains a named owner-run release gate.
