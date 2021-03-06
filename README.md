# Rust Markdown to graph
This program converts a Markdown file into a graph. For now, it creates .dot file which graphviz uses to build graph.

It translate headings in a Markdown files into graph nodes and levels into edges with H1 heading as the graph root. That is, children headings are adjacent to a parent heading and same level headings are in the same level in the output graph, too.

## No more graph drawing
I've created it and am using to comprehend an article structure. It integrates two very close works: write an article and draw an easily understandable structure of the article to examine it. This will simplify writing workflows especially when you write a well-structured one like Pillar Page or share its anatomy with others.


## Example
If you have the following simple Markdown file, the program would create a dot file, which in turn can be converted in the following image using graphviz command line.

Please note that a result ignores everything except headings. It is intentional for simplisity of the output.

> # H1
> ## H2_1
> some texts
> ## H2_2
> ### H3_1
> #### H4_1
> #### H4_2
> ##### H5
> H is an abbreviation of Heading.

![Sample image](./examples/test.jpg)

## Caution
In this moment, for some reasons this program fails to handle README.md, so if you will try it, please use some other Markdown files.