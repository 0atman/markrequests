---
head: 0atman:branch-name-with-your-changes
assignee: 0atman
base: local-branch-name-on-this-repo
draft: true
issue: linked issue url?
reviewers:
  - 0atman
  - user2
  - user3
tags:
  - help wanted
  - bug
  - documentation
  - duplicate
  - enhancement
  - invalid
  - question
  - wontfix
---

# PR Title Goes Here

The first item in the document should be an H1, `#`, with the title of the PR.

This is a pull request template, with machine-readable frontmatter in YAML.

## (YAML Frontmatter Explanation)

Yaml frontmatter is both extremely human readable and very standard for Markdown metadata. It's used in many static site generators including Jekyll and in the biggest desktop markdown note taking tool, Obsidian.

Implementors must only strip out the YAML between `---` at the start, and you can then parse both the YAML and Markdown normally.

All the normal YAML caveats must be considered (such as [The Norway Problem](https://hitchdev.com/strictyaml/why/implicit-typing-removed/) so we'll want to define that a strict subset to be used, perhaps quoting all strings etc.

### Proposed YAML Keys

| key       | explaination                                                                                 | example                                                |
| --------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| head      | fully-qualified branch name with proposed changes. The repo must have exactly the same name. | 0atman:my-branch                                       |
| assignee  | the assignee (usually the proposer) of this PR                                               | 0atman                                                 |
| base      | the local branch name that we wish to modify (usually main)                                  | main                                                   |
| draft     | true/false this is often used to indicate a WIP PR                                           | true                                                   |
| issue     | a linked issue url, either inside this system, or external                                   | <http://example.com> |
| reviewers | a yaml list of usernames to review this PR                                                   | [0atman, user2, user3]                                 |
| tags      | a yaml list of tags categorising this PR                                                     | [bug, documentation]                                   |

I suppose arbitrary keys outside this list could be fine too, everyone will organise their PRs and issues in different ways.
I've chosen names based on the GitHub API, as a reasonable reference, except I changed 'labels' to 'tags' because that's what we call them.

## Comments

> I am unsure the best way to do comments, but perhaps appending quote blocks like this with the username after a bullet?

- 0atman

> Yeah, other Tris, sounds good!

- 0atman_with_moustache
