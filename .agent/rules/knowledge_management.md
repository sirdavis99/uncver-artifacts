---
trigger: always_on
---

# Knowledge Management & Quality Control Rule

This rule ensures that the AI agent maintains a consistent knowledge base and high code quality by checking specific documentation and codebase state before and after every major action.

## 1. Pre-Action Checks

Before responding to a user request or modifying code, you MUST:

- **Check AGENTS.md**: Read the current project learnings and guidelines to ensure your proposed changes align with established patterns and architectural decisions.
- **Trace Knowledge Links**: If `AGENTS.md` points to specific feature modules in `.agent/knowledge/`, you MUST read those relevant files to understand the full context and constraints.
- **Verify Code State**: Check existing imports, hooks, and components in the target file to avoid duplication and ensure compatibility.
- **Maintain Modular UI**: Confirm that UI components follow the small-file rule (max ~150 lines) before editing.
- **Detailed Planning**: Before any major code changes, you MUST create a **Detailed Implementation Plan** artifact. This plan must contain low-level findings, specific file paths, and a step-by-step execution path. High-level summaries alone are insufficient.

## 2. Post-Action Updates

After completing a task, you MUST:

- **Update Feature Knowledge**: If the task involved a specific feature or pattern, update/create the corresponding markdown file in `.agent/knowledge/`.
- **Sync AGENTS.md**: Ensure `AGENTS.md` includes a summary of the latest discoveries and points to any new or updated knowledge modules.
- **Document Enforcements**: Explicitly note any coding standards (especially modularity) or patterns that were enforced during the session into the knowledge base.
- **Quality Scan**: Run `cargo check` after any Rust edits, and confirm no new compiler errors or warnings were introduced.
