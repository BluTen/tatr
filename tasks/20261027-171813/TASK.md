# add summary command!

- STATUS: OPEN
- PRIORITY: 50
- TAGS: feature, todo

Write a similar command to show a summary of open tasks, by tag and total. tsoding/tatr doesnt support unknown statuses, but mine does. So the command signature will be a lil different.

A flag to show summary for all status `--all`

and An argument [or list of arguments] of statuses that need summary.

=> `tatr summary OPEN REVIEW` to get summary of tasks with status `OPEN` or `REVIEW`,
=> or `tatr summary [ -a | --all ]` to get summary for all active statuses.

An example of a summary output from tsoding/tatr
```
$ tatr summary
STATUS:   OPEN
TOTAL:    27
UNTAGGED: 15
TAGGED:
    scope =>   5 - currently working on
     spec =>   5 - tasks that may require updating the spec
      bug =>   3
      tql =>   2 - Tatr Query Language
```