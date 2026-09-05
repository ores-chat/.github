# Organization governance rules

- Defaults belong here only when they apply across the organization; repository-specific rules remain local.
- Changes flow through pull requests with immutable dependencies and repository-local verification.
- Shared Auth owns identity; ORES Chat owns product authorization; customer/admin fallback is forbidden.
- Keep secrets out of Git and public discussion. Use private vulnerability reporting.
- Do not name nonexistent teams in CODEOWNERS; add ownership only after organization teams are created.

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.
