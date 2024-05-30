tell me, oh tell me, the way it works, who is that inside of me?
here in this broken-down, broken-down world, you laugh, without seeing a thing
I'm just so damaged, I hold my breath
Not even the truth can unravel me, freeze
breakable, unbreakable, shakeable, unshakable
when I found you, it shook me
in this shook-up, twisted world, I'm gradually growing transparent and vanishing
don't look for me; don't look at me
I don't wish to hurt you in a world of someone else's imaging
remember who I am, my full, vivid self
entangled in the loneliness that fans out endlessly, stung by the memory of smiling so innocently,
I can't move, I can't move, I can't move, I can't move, I can't move, I can't move!

UNRAVELLING THE WORLD

UNRAVELLING THE WORLD

UNRAVELLING THE WORLD

I've completely changed, I couldn't change back
the two entwine, the couple perishes
breakable, unbreakable, shakeable, unshakable
I won't defile you!
in this shook-up, twisted world, I'm gradually growing transparent and vanishing
don't look for me; don't look at me
before the future comes completely undone, caught in a pit of solitude somebody set for me,
think back on who I am, my full, vivid self
don't forget me, don't forget me, don't forget me, don't forget me,
the change over me has me paralyzed,
in an immutable object paradise,
remember who I am
tell me, tell me, is there someone inside of me?


This program is about how to create a **singly linked list** and shows how **streams work**. 
Sometimes when there are many threads running, they may not wait for the other one to complete and will try to process the data that these threads need, so Mutex is a good solution. They allow you to avoid potential resource leaks and automatically release the lock when the mutex goes out of scope.

But anyway, because the mutexes were not good, we may have mutual locks that can lead to the program stopping. 
To get out of this situation, we can use the try_lock() method to obtain a lock - if it is impossible to obtain a lock using it, then the thread continues to work without blocking. This can help prevent deadlocks in situations where obtaining a lock is not critical.
