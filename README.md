# My take on kanban
Kanban is a scheduling. if you want to know more. Well. Internet :).

What's different?
a kanban usually has 3 stages
| TODO | DOING | Finished |
|------|-------|----------|
|      |       |          |

For me that wasn't enough so I added two more.

| Back Burner | TODO | DOING | Awaiting Changes | Finished |
|-------------|------|-------|------------------|----------|
|             |      |       |                  |          |
- Back Burner - Things I'd like to do in near future. e.g. few days
- TODO - Things to do after current task is done
- Doing - one currently active task.
- Awaiting changes - Things I consider done, but are waiting for feedback or there's a chance I'll have to go back to it in near future
- Finished - Tasks that I can look at to feel better about myself

 
# Building
use
``` sh
cargo build --release
```
to compile and then something like 

``` sh
cp target/release/kanban ~/.local/bin/kb
```
to install.
Everything is stored under `~/.local/kanban.yaml` in release and `./kbn.yaml` in debug build.
# Usage
I'll assume that you've renamed/aliased the executable to `kb`.
General usage is something like `kb COMMAND [args]` where command can be whole command or it's beginning.
e.g. `kb add`, `kb ad` and `kb a` do the exact same thing.

- `kb` Prints all the tasks and notes for current task
- `kb add TASK_NAME` adds task to the Back Burner
- `kb back TASK_NAME` moves task to the Back Burner
- `kb todo TASK_NAME` moves task to TODO
- `kb doing TASK_NAME` moves task into Doing state, if other task is in Doing state, it will be changed to TODO
- `kb waiting TASK_NAME` moves task to Awaiting changes state, moves first task from TODO to doing
- `kb finished TASK_NAME` moves task to Finished, moves first task from TODO to doing
- `kb purge` removes all tasks in Finished state
- `kb note add` adds note to the current task
- `kb note remove NUM` removes note number `NUM` from Task under Doing
- `kb note rm` same as `note remove`
- `kb note delete` same as `note remove`

# TODO
- comments
- fix my broken english
- help/usage message (`-h` argument)
- deb/arch packages??
