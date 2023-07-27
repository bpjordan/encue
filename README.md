# Encue

Encue (pronounced "on cue") is a simple TUI-based application
for running sound cues. It is primarily designed for use in a
theater setting.

Encue is still WIP and most funcitonality described here is not
yet implemented

Encue gets sound cues from a "script" stored in the app's
working directory. This script should be a YAML file named
`script.yaml` and contain cues which reference audio files
relative to the working directory.

## Example Script file

```yaml
cues:
- label: SQ1
  description: my first sound cue
  playback: my_sound_cue.mp3
- label: SQ2
  description: fade out my first sound cue
  fade:
    target: SQ1
    duration: 20s
```

## Planned Features Note

Everything listed below is not yet implemented, and is only
listed to give me a place to write down my plans for the app.

## Cues

Each cue contains a unique label, an action specifier, and an
optional description field, plus an optional context field
to give the line or visual cue to start the cue.

### Cue Fields

All cues can have the following fields:

- `label` (required) is a unique identifier for the cue, used both as a quick
identifier for the user and internally to target cues. You can label
cues according to your preferred convention. For example, `SQ1`, `SQ10`,
`SQ0.1`, `1`, and `my-cue` are all valid labels as long as they are unique
within the show.
- `description` is a longer description of the cue, such as "Scene change music",
"Phone rings", or "Distant explosion". Descriptions are only used for displaying
to the user and need not be unique.
- `context` provides the user with a note about when to start the cue. This can
be a line or a description of a visual cue (ex. "Truvy slaps the radio")

A cue also requires one action field, described [below](#cue-actions)

All three YAML objects below are examples of a valid cue:

```yaml
- label: SQ0.5
  playlist:
    folder: preshow

- label: 0.9
  fade: SQ0

- label: 1
  description: Gunshot
  context: 'Truvy: "Feel free to use as much hairspray as you want"'
  playback: sfx/gunshot.mp3

- label: stop-phone
  context: Truvy picks up phone
  stop: 1

- label: SQ2
  description: Gunshot & barking
  context: 'Annelle: "I had no idea"'
  playback:
    file: sfx/gunshot_barking.wav
```

### Cue Actions

The actual action of the cue can be one of the following:

- A `playback` cue starts playing a sound from a single audio file
- A `playlist` cue starts a playlist consisting of multiple audio files
- A `fade` cue fades another cue or output master to a target level
(default 0) over a target duration
- A `stop` cue stops another cue or all cues

The arguments to each action 
