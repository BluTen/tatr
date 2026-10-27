# add ls command!

- STATUS: OPEN
- PRIORITY: 50
- TAGS: feature, todo

Write a similar command to list tasks that are not closed, but unlike mista zozin's tatr, it wont handle updates. Rather will be a separate command such as `tatr edit` or `tatr update`.

Format example from tsoding/tatr:
```
$ tatr ls
./tasks/20260315-160715/TASK.md:1: OPEN [PRIORITY: 100] Build a graph of issues crossreferring to each other
./tasks/20260910-063140/TASK.md:1: OPEN [PRIORITY: 100] Can we make the executable fully static and ship it in the GitHub releases?
./tasks/20260829-225753/TASK.md:1: OPEN [PRIORITY:  90] [tql] All comparison operators could work with booleans too
./tasks/20260304-115038/TASK.md:1: OPEN [PRIORITY:  90] Priority becomes irrelevant when the task is closed (modification date is more important)
./tasks/20260914-105224/TASK.md:1: OPEN [PRIORITY:  80] JSON reports
./tasks/20260919-135303/TASK.md:1: OPEN [PRIORITY:  80] Record/replay testing
./tasks/20260305-120316/TASK.md:1: OPEN [PRIORITY:  50] [spec,scope] An ability to actually store `tasks` folder outside of the main repo
./tasks/20260316-142209/TASK.md:1: OPEN [PRIORITY:  50] [spec] Subtasks
./tasks/20260830-041724/TASK.md:1: OPEN [PRIORITY:  50] Mass rename a tag
./tasks/20260308-171346/TASK.md:1: OPEN [PRIORITY:  10] There should be a command that reports all the skipped weird folders and files found in the tasks/ folder
./tasks/20260315-160917/TASK.md:1: OPEN [PRIORITY:  10] [bug,spec] When you have several TASK.md opened, it's unclear which issues they belong to
```
