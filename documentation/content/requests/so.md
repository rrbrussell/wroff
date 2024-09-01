+++
title = 'so -- Switch source file.'
date = 2024-08-22T05:52:28-05:00
draft = false
+++

**.so** _filename_

Switch source file. The top input (file reading) level is switched to
_filename_. The effect of an **so** encountered in a macro occurs immediately.
When the new file ends, input is again taken from the original file. **so**'s
may be nested.
