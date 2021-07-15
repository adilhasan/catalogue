# Categorise
Program to find all PDF documents in a folder, annotate the documents and store the metafata in an sqlite database.

## Commands

## Scan directory
```
categorise [-c <config>] scan [-r] <directory>
```
Scans `<directory>` recursively storing the file path, checksum in a database `<dbase>`. The config file `<config>` contains configuration parameters as well as the list sql statements used to interact with the database.

## Search the database
```
categorise [-c <config>] list [-d -t] [-a <after-date> -b <before-date> -s <search>] <output-file>
```
By default the list will return all the entries in the database. The list can be narrowed by supplying dates and a string to search for. The '-d' will select entries with an null description, '-t' will return entries with a null title. 

## Annotate the database
```
categorise [-c <config>] annotate <file>
```
Takes as input a JSON file `<file>` that contains the records to be annotated. The structure of the JSON file is the same as output by the 'list' subcommand. 

