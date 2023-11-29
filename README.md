# Yteeee Server

> NOTE: Make sure you've `yt-dlp` installed as we rely on it to do everything.

## How to get subtitle of a youtube video.

Send a post request to `/get_subtitle`, example

```bash
curl -X POST http://127.0.0.1:8080/get_subtitle \
    -H 'Content-Type: application/json' \
    -d '{"video_url": "https://www.youtube.com/watch?v=L3MjPtK7ZP8"}'
```

The reponse will look something like following.

```
so there's a lot of ad blocker drama
going on right now and I I thought I'd
just make a video to give you my
thoughts on that so if you haven't heard
YouTube is essentially threatening users
that if they use ad blockers they're
going to make it so they can't see
videos they're also saying that it's
against terms of service and you should
unsa your ad blocker immediately
...
```
