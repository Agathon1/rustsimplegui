# rustsimplegui

Easy-to-use rust GUI library, with an api heavily based on [pysimplegui](https://github.com/PySimpleGUI/PySimpleGUI)'s.

Current features:
- [x] Automatic layout using just vec![] 2d arrays.
- [x] Widgets : Text, Button, CheckBox, Radio, [Text]Input, Slider, Separator
- [x] Customisable widget size, padding, color
- [x] Boilerplate for adding new back-ends (somewhat modular)

Future plans:
- [ ] More backends.

- [ ] Fix some akwardness in the API.

- [ ] Add more functionality. (Custom colours, etc)

---

### Repo Index:

rsg_tk - Tkinter (Tcl/wish) backend for rsg, built on top of :

rstk - Modified version of https://crates.io/crates/rstk

rsg_core - core data structures shared by all backends, and user.

rustsimplegui - interface between backends <-> user.

examples - How to use.
