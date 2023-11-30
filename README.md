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

## Get video info

```bash
 curl -X POST http://127.0.0.1:8080/get_video_info -H 'Content-Type: application/json' -d '{"video_url": "https://www.youtube.com/watch?v=L3MjPtK7ZP8"}'  | jq
```

Output

```json
{
  "id": "L3MjPtK7ZP8",
  "title": "My Thoughts on the YouTube Adblocker Drama",
  "thumbnail": "https://i.ytimg.com/vi/L3MjPtK7ZP8/maxresdefault.jpg",
  "description": "Here's my take on all of the ad blocker related drama going around the interwebs.\n\nHope you enjoyed the video!\n\nCheck out some code on my GitHub:\nhttps://github.com/realtux\nhttps://github.com/engineer-man/youtube\n\nOther Social:\nhttps://reddit.com/r/engineerman\nhttps://x.com/_EngineerMan\nhttps://discord.gg/engineerman",
  "channel_url": "https://www.youtube.com/channel/UCrUL8K81R4VBzm-KOYwrcxQ",
  "channel": "Engineer Man",
  "view_count": 5390
}
```
