# hero-synth

The end goal of this project is to experiment with different technologies around music creation.

The initial plan is to create an audio synthesizer that is able to model Additive, Substractive and FM synthesis all in one. The idea is to add some basic units such as oscilators, LFOs, evelopes, filters and effects, and allow the user to route them using a kind of [matrix mixer](http://en.wikipedia.org/wiki/Matrix_mixer).

I also plan to experiment with MIDI interfaces to control the system. But more of that later on ...

These are the sub-projects right now:
- core: core components to build synths and audio effects
- synth: the matrix synth implementation
- host: The audio/MIDI system that hosts the synth.
- wfgen: An utility to build wavetable data as code, used by the core wavetable component.
