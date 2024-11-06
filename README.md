# Run Stuff Off To Web

## Why?

If you want a nicely printed piece of technical documentation you need to write it with LaTeX.
LaTeX doesn't convert into html without hordes of wailing and gnashing of teeth.
Markdown converts nicely into html but makes printing almost as hard. Especially for interlinked
documents, the HyperText part of HTML.

What about DocBook? Unfortunately it sucks at both tasks. Getting a good print output is several
hours of frustration in writing your own XML-FOP for the layout and page description, stuff LaTeX
already does. The HTML conversion is slightly better for a single project, like a more conventional
book. If the documentation or collected material is mostly a collection of single articles under
a category or tag in a taxonomy then you are effectively going to rebuild most of
[Hugo](https://gohugo.io), [WordPress](https://wordpress.org), or whichever static site generator[^1]
or content management system[^2] your prefer.

Epub output is just different enough from the traditional output models that little, if any, software
exists to deal with handling Epub effectively. Effectively being defined as can take a taxonomy item
and make a proper table of contents from it.

[TexInfo](https://www.gnu.org/software/texinfo/) looked like a good option for a little while. It
could at least generate static html that was largely Hugo compatible if you didn't mind spending a
few hours to micromanaging the pagination rules for sections. It also makes
decently good printable output.

So after a bunch of paper cuts, I am going to break down and write my own tool. To satisfy the
renaming requirements in the TeX license I will call the project Run Stuff Off To Web or
wroff for the main executable name. This program is borrowing ideas from both Tex and Roff. The input
syntax and print output engine is TeX based. The HTML and ePub generators do not work off an
intermediary output format however. Several of the requirements for an SSG or CMS require a different
understanding of how input files are interlinked.

[^1]: Uses the acronym SSG.

[^2]: Uses the acronym CMS.

## Licensing

The Rust code is licensed under the GNU General Public License version 2.0 only.
The include macros are licensed under the Apache 2.0 License.
