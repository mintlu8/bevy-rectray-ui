# Signals

`bevy_aoui` uses signals for reactivity.

Signals are multi-producer multi-consumer cells that can only carry one value per frame.
Data sent through signals are type erased to avoid generics, which improves ergonomics and reduces
the number of systems needed for signals to function.
