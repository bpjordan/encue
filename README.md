# Encue

Encue (pronounced "on cue") is a simple TUI-based application
for running sound cues. It is primarily designed for use in a
theater setting.

Encue gets sound cues from a "script" stored in the app's
working directory. This script should be a
[YAML](https://yaml.org/refcard.html) file named `script.yaml`
and contain cues which reference audio files relative to the
working directory.

## Defining Cues

The most important field of the script is the `cues` field,
which contains an ordered list of all cues in the file.

A cue contains the following fields:

```yaml
- label: ...        # A unique identifier for this cue used by other cues to reference this one
  description: ...  # OPTIONAL: a short description of the cue to display to the user
  hint: ...         # OPTIONAL: the line or visual cue that signals this cue
  # ACTION: see below
```

### Cue actions

In addition to the fields above, each cue must contain one
action directive. Action directives usually have a short form
with a single parameter and a long form with more advanced
parameters.

Possible action directives include:

```yaml
playback: <filename>  # Play the file at `filename`
# OR
playback:
  file: <filename>
  volume: <x>       # Volume (0-100) to start at (default: 100)
  duration: <x>     # OPTIONAL: only play the first `x` seconds of the file
  fade_in: <x>      # OPTIONAL: fade in for `x` seconds

playlist: <folder>  # Play all files in `folder` as a playlist
# OR
playlist:
  folder: ...
  files:                  # OPTIONAL: list of files to play
    - ...
    - ...                 # Must specify at least one of `files` and `folder`
    - ...
  volume: ...             # Volume (0-100) to start at (default: 100)
  loop: <true|false>      # Loop playlist (default: false)
  shuffle: <true|false>   # Shuffle files in playlist (default: false)

fade: <target>      # fade cue <target>
# OR
fade:
  target: <target>
  volume: ...       # Volume (0-100) to fade to (default: 0)
  duration: ...     # Number of seconds to fade (default: 5)

stop: <target>      # immediately stop target cue

```

### Example Script file

The following is an example of a valid script file

```yaml
cues:
- label: SQ1
  description: my first sound cue
  playback: my_sound_cue.mp3
- label: SQ2
  description: fade out my first sound cue
  fade:
    target: SQ1
    duration: 20
```

## Planned Features

`encue` is under active development as a hobby/side project. Below are
some features I hope to implement soon:

Additional UI elements:
- An audio visualizer
- A clock
- A list of active playback cues with progress bar for elapsed time
- Show cue loading status
- Jump to a cue by its label

Additional cue parameters:
- Crossfade between files in a playlist
- Fade out playback cues after set duration


