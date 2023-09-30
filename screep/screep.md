# Screep Overmind 设计学习

这篇笔记主要跟随overmind作者的博客，学习一下overmind的设计

https://bencbartlett.wordpress.com/2017/12/19/screeps-0-a-brief-history-of-game-time/

https://bencbartlett.wordpress.com/2018/01/15/screeps-1-overlord-overload/

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230930150734.png)

## Level 0: Overmind and the tick cycle

I’ve simplified the structure of my [main loop](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/main.ts) considerably in this rewrite. Excluding memory checking and sandbox code, there are now three major phases in each tick, each of which is executed by a call to the [Overmind](https://github.com/bencbartlett/Overmind/blob/12ed8718608c51eccd998a22acad0ba486cdc385/src/Overmind.ts):

1. `build()` All caching and object instantiation is done in this phase. [Colonies and their overlords are instantiated](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/Overmind.ts#L31), then [colonies instantiate their hive clusters](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/Colony.ts#L149) and their overlords. Finally, [directives and their overlords are instantiated](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/Overmind.ts#L92). (More on overlords below.)
2. `init()` This phase handles all pre-state-changing actions, primarily various requests like [creep spawning requests](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/overlords/Overlord.ts#L106) and [transport and link requests](https://github.com/bencbartlett/Overmind/tree/dfcc17146de408112c1186b1a552d2be6572751c/src/resourceRequests).
3. `run()` This is where the action happens. All state-changing actions happen here; most will require information that is populated in the `init()` phase. HiveClusters will look through their various requests to determine what actions should be taken ([spawning the highest priority creep(s)](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/hiveClusters/hiveCluster_hatchery.ts#L169) from the requests, [loading/unloading the storage link](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/overlords/overlord_commandCenter.ts#L78), etc.). Overlords will scan through their Zerg and assign new tasks through a decision tree to each one that `isIdle`, such as [maintaining a miningSite](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/overlords/overlord_mine.ts#L41), [determining which structures to supply](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/overlords/overlord_supply.ts#L38), or [hauling back energy from a remote source](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/overlords/overlord_haul.ts#L52). The Overseer examines each room to look for any anomalous conditions, such as an [invasion](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/Overseer.ts#L29) or a [colony crash](https://github.com/bencbartlett/Overmind/blob/dfcc17146de408112c1186b1a552d2be6572751c/src/Overseer.ts#L39), and responds by placing Directives accordingly.

## Level 1: Colonies, Hive Clusters, and Directives

Not much has changed here; as before, the main idea behind colonies and hive clusters is to organize what belongs to what in a hierarchical manner based on what each object is instantiated by. Colonies are instantiated by an owned room and a list of outpost rooms (determined by directives) and organize these rooms into a single unit. HiveClusters are instantiated by a RoomObject belonging to a colony and group multiple structures and components together which share related functionality. Both Colonies and HiveClusters can have overlords put on them.

Directives are on the same hierarchical level as HiveClusters, but they are a little different, since the colony does not directly instantiate them. They are instantiated from flags by the Overmind and assigned to a colony based on their location. Directives don’t have much internal logic (some will remove themselves, but that’s about as complex as it gets right now) but their main function is to be a conditional attachment point for overlords. Directives are categorized by color codes, with the primary color indicating a category and a secondary color indicating the specific type. I’ve currently only written a few of the most essential directives, but the categories I have in mind are:

- Purple: colony directives – territory (claiming/reserving rooms and grouping rooms in colonies) and colony operations (incubating lil’ babby colony)
- Red: military directives – defend against NPC invaders, attack a room, etc.
- Orange: operational directives – deal with non-standard colony conditions, like recovering from a crash with a bootstrapping directive
- Yellow: energy and resource directives
- White: RoomPlanner directives (more about this in the next post!), which allow for guided planning of colonized rooms, such as positioning hive clusters and placing road routing waypoints.

## Level 2: Overlords and Overseers

Overlords are really the heart of this update, if you couldn’t tell by the title of this post. An Overlord is a generalization of a set of related things that need to be done in a colony like mining from a site, bootstrapping a new colony, guarding against invaders, or building construction sites. Overlords handle spawning or obtaining suitable creeps to do these things and contain the actual implementation of doing them, replacing the messy `Objective` system in the older AI. If HiveClusters are the organs of a colony, Overlords are the biological processes which make those organs function.

One of the biggest (and hardest) design decisions I had to make with this rewrite was how to handle instantiation of Overlords. Initially, I was drawn toward using directives as the only instantiation method, such that every process in a colony would have its own flag. However, I decided against this idea for two reasons: (1) it seemed to be unnecessary and unintuitive to use directives for normal operation (HiveClusters would need to be changed to be instantiated from flags, which is against their design, or would have a split cluster-directive nature which I didn’t like) and (2) there is speculation that the flag cap may eventually be lowered from 10,000 to 100, so I don’t want to rely on flags too heavily.

Eventually, I decided that an overlord can be instantiated from anything that has the following properties:

- `name`: for generating unique Overlord references
- `room`: an Overlord handles operations which primarily take place in one room
- `pos`: Overlords must be instantiated from a physical object
- `colony`: for assigning which colony handles the spawn requests (I added a self-referencing `Colony.colony` property so that Colonies could instantiate Overlords as well)
- `memory`: Overlord memory is stored in `instantiator.memory.overlords[this.ref]`

This allows Overlords to be instantiated from a Colony, HiveCluster, or Directive, which makes them a very versatile control model. Colony overlords are for constant, colony-wide operations, like handling workers to build new things. HiveCluster overlords are more specialized but still always present, like spawning miners for a site or a dedicated Hatchery attendant. Directive overlords tend to be conditional, like guarding against NPC invaders or claiming a new room.

When an Overlord is instantiated with a specified priority, it automatically adds itself to a priority queue on the colony `Overseer`. The Overseer is responsible for running all Directives and Overlords, as well as placing new Directives to respond to various stimuli.

## Thoughts

上面这块把可以借鉴的设计都贴了过来，一个比较关键的点就是尽可能的让一切都自动化，避免手操

 这里还有一些可能的reference：

https://www.jianshu.com/p/7226e08c4b8e

https://www.jianshu.com/p/d5e1a50473ce

主要需要设计的就是任务系统了，这里有几个问题：

1. 我们是有一个全局的队列，creep自己去任务队列里拉取任务，还是说overload主动给某个特定的creep发布任务
2. 不同角色要怎么确定任务的优先级，比如搬运者MOVE part比较多，怎么优先给搬运者分配搬运相关的任务
3. 不同creep的part要怎么动态分配？

现在想的一个比较折中的方案，多个Task构成一个图，creep被分配任务后就不断执行图中的任务。生成creep的时候给他分配一个带有无限循环的任务图，这样就可以实现role based系统了。

