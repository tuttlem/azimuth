# Data Model: Impact Screen Flash

| Field | Meaning | Validation |
|---|---|---|
| active | Whether a pulse is fading | One current pulse only |
| elapsed | Presentation time since impact | Nonnegative |
| opacity | White overlay strength | Bounded and rapidly decreasing |
| duration | Fade length | Short positive bounded value |
| source scale | Existing impact visual scale | Read-only finite input |

```text
resolved terrain impact → start/reset one pulse → fade each frame → inactive
out of bounds → no pulse
```

Flash state never mutates terrain, projectile, tanks, turn, camera, AI, or audio.
