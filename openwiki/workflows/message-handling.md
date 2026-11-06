---
type: workflow
title: Message Handling and Response Rules
description: The ordered text and document message pipeline, including link rewrites, trigger responses, date-based pranks, and error behavior.
tags: [telegram, messages, rules, rust]
verified:
  - by: openwiki/0.9.0
    at: 2026-11-06T08:17:09.762Z
sources:
  - id: openwiki-source-ed8bf05e307c6278442542c2
    resource: repo://src/lib.rs
  - id: openwiki-source-602e8d764f5b04cab0c6b160
    resource: repo://src/message_checks/bad_words.rs
  - id: openwiki-source-dd36b334272e5754f5bc84da
    resource: repo://src/message_checks/reddit.rs
  - id: openwiki-source-c34c075bbc5165f4c41d80e6
    resource: repo://src/message_checks/tiktok.rs
  - id: openwiki-source-e5b5d9fe4d8bdfbb8f9708ac
    resource: repo://src/message_checks/twitter.rs
  - id: openwiki-source-9d1091fc0510cc03ef25975e
    resource: repo://src/message_checks/url.rs
  - id: openwiki-source-cdd4e5d0b52f6f0e652c51a4
    resource: repo://src/prank/day_check.rs
  - id: openwiki-source-dd14ac3cf44e4d16f2317300
    resource: repo://src/prank/randomizer.rs
  - id: openwiki-source-bd0e6d74911b4ac1383fd632
    resource: repo://src/social_media_handler.rs
  - id: openwiki-source-0993c2c0112ac99cf6999828
    resource: repo://src/utils.rs
  - id: openwiki-source-f6073dab05580a0ca002820f
    resource: repo://tests/tests_bad_words.rs
generated: { by: "pi", at: "2026-11-06T08:17:09.762Z" }
---

# Message Handling and Response Rules

`handle_messages` dispatches Telegram text to the text pipeline and documents to the media pipeline. It converts any downstream error into `BotError::Processing`, which the listener logs rather than returning to Telegram.

## Text-message order

The text pipeline evaluates several independent behaviors in this order:

1. If the full text is a URL, it first applies a social-media rewrite. It then checks the URL extension for direct `.webm` and `.mp4` processing; both extension checks can run after the rewrite step.
2. On Spanish April Fools' Day (December 28) or April 1, a 10% random gate enables one prank branch: a 15% upside-down reply, otherwise a 70% sarcastic reply, otherwise a randomly selected embedded image with caption.
3. It lowercases the original text and tests it for the configured bad-word regular expression. A match queues a `Deficiente` reply.
4. It finds every configured copypasta trigger present in the lowercase text. Normal triggers queue their configured text response; the `viernes` trigger uploads the embedded Friday video immediately.
5. If the lowercase message contains `gracias asuka` on a Thursday in the server's local timezone, it queues a randomly selected greeting.

Queued text replies are sent sequentially after a typing action, with a one-second delay between sends. The selected prank image and Friday video are immediate upload operations rather than queued replies.

## Social-link rewriting

For URL text, the bot chooses at most one rewriter using priority: Twitter/X, then TikTok, then Reddit. A successful rewrite deletes the original Telegram message and posts formatted replacement content containing the sender's username when available, or an anonymous label otherwise.

Twitter links are rewritten to `fxtwitter.com`, X links to `fixupx.com`, TikTok domains are changed to `tnktok`, and Reddit links use `old.reddit`. Because the social handler returns after the first applicable rewrite, a message with multiple platform types follows the first matching platform's rule.

## Rule data and test coverage

Bad-word detection uses one lazily compiled case-insensitive regular expression covering `uwu`, `:v`, and `owo` with surrounding non-word characters. Copypasta responses are static in-memory trigger/response data; the documentation intentionally does not duplicate their content.

Focused unit tests cover URL recognition, social-link rewrite variants, and bad-word matches. The integration test suite also verifies the three bad-word tokens and a non-match.

For document conversion and direct video links, see [Media Download and Conversion](media-processing.md). Runtime listener behavior is described in [Runtime and Deployment Architecture](../architecture/runtime.md).
