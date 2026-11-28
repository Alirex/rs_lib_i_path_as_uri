# Brand book: path_as_uri

Name: `path_as_uri`. Tagline: "A path you can click." The name, colours, fonts and marks are one source,
[brand/brand.yaml](../brand/brand.yaml), with the master SVGs in [brand/masters/](../brand/masters/); this page is the
prose around them. The crate name is kept as published on crates.io (see the decision record in docs/decisions/).

## Mission

Turn an absolute filesystem path into a correct file:// URI, with one call.

## Vision

Every path a terminal or a log shows is a link that opens the file.

## Audience

Rust developers who print file paths in a terminal, a log or a report and want them clickable. Second: AI agents that add the crate to a project and need the right function for the job.

## Values

1. **Correct before short.** Spaces and special characters are encoded by the `url` crate, not by hand.
2. **Small.** Three functions, one error type, two dependencies.
3. **Honest about failure.** A relative path is an error, or a documented panic in the short form.

## Voice

Plain and exact: say what the tool does to which file, with the command that shows it. No marketing words.

| Do                                               | Do not                                                 |
| ------------------------------------------------ | ------------------------------------------------------ |
| Write "a relative path returns `PathAsUriError`" | Write "handles all paths seamlessly"                   |
| Keep the name in code type: `path_as_uri`        | Capitalise it, translate it or add a logo-style suffix |

## Logo rules

The mark is a path drawn as a rising line that ends in a link point in the ink colour, with one dot in the accent colour. Use it as supplied, on the paper
colour or a calm background; keep a quarter of its width free on every side; 16 px is the smallest size (use the small
mark there). Do not stretch, rotate, recolour or add effects. The artwork is CC-BY-4.0; a fork uses its own name and
mark.
