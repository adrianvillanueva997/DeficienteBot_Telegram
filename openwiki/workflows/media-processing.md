---
type: workflow
title: Media Download and Conversion
description: How the bot downloads direct video links and Telegram webm documents, converts webm content to MP4, and sends the result back to chat.
tags: [telegram, media, ffmpeg, downloads]
verified:
  - by: openwiki/0.9.0
    at: 2026-11-06T08:17:09.762Z
sources:
  - id: openwiki-source-bb1ebe868e35e9e500714501
    resource: repo://Dockerfile
  - id: openwiki-source-ed8bf05e307c6278442542c2
    resource: repo://src/lib.rs
  - id: openwiki-source-9d1091fc0510cc03ef25975e
    resource: repo://src/message_checks/url.rs
  - id: openwiki-source-016458ce932b873b1bd2fa3c
    resource: repo://src/message_checks/webm.rs
  - id: openwiki-source-5883e61c53f66e38002a0b24
    resource: repo://src/online_downloads/url_checker.rs
generated: { by: "pi", at: "2026-11-06T08:17:09.762Z" }
---

# Media Download and Conversion

The bot supports two independent media workflows: direct links to `.webm` or `.mp4` files in text messages, and uploaded Telegram documents whose filename contains `webm`. Both workflows reply to the originating message with a Telegram video.

## Direct video links

Text messages enter direct-media processing only when the entire message parses as a URL. A URL whose path extension is `.webm` or `.mp4` is first fetched with `reqwest` to check for a 2xx status. Non-success responses and network failures produce the Spanish unavailable-video reply.

For a successful `.webm` link, the bot creates timestamp-derived `.webm` and `.mp4` names in its working directory, downloads the full response body, invokes `ffmpeg -i <webm> <mp4>`, and uploads the MP4 as a reply. The `.mp4` direct-link branch downloads and sends the MP4 without transcoding. It deletes the MP4 after sending, but the direct `.webm` branch does not delete either generated file.

## Telegram document conversion

A document is eligible when its optional filename contains `webm` and its Telegram-reported size is at most 20,000,000 bytes. The bot retrieves the file through Telegram's API, writes it to a generated `.webm` file, converts it to MP4, uploads the converted file as a reply, and deletes both temporary files after a successful upload. Ineligible documents are ignored.

## Conversion and operational constraints

`convert_webm_to_mp4` starts the system `ffmpeg` executable and waits for it to finish. Its implementation does not inspect ffmpeg's exit status, so a process that starts but fails may leave a missing or unusable output for the later upload attempt. Downloading buffers an entire direct-link response in memory and has no application-level size limit.

The current production Dockerfile does not install `ffmpeg`; deployments that use conversion must provide it in the runtime image or otherwise make it available in the container. The working directory must also be writable because all temporary media is stored there.

See [Message Handling and Response Rules](message-handling.md) for the surrounding text-message sequence and [Runtime and Deployment Architecture](../architecture/runtime.md) for container and webhook configuration.
